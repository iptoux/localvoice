#include "parakeet_capi.h"

#include <algorithm>
#include <cctype>
#include <cmath>
#include <cstdlib>
#include <cstdint>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

namespace {

parakeet_ctx* g_ctx = nullptr;
parakeet_stream* g_stream = nullptr;
std::string g_model_path;
std::string g_language = "auto";
std::string g_full_text;

void emit(const std::string& payload) {
  std::cout << payload << '\n';
  std::cout.flush();
}

std::string json_escape(const std::string& value) {
  std::ostringstream out;
  for (unsigned char c : value) {
    switch (c) {
      case '"':
        out << "\\\"";
        break;
      case '\\':
        out << "\\\\";
        break;
      case '\b':
        out << "\\b";
        break;
      case '\f':
        out << "\\f";
        break;
      case '\n':
        out << "\\n";
        break;
      case '\r':
        out << "\\r";
        break;
      case '\t':
        out << "\\t";
        break;
      default:
        if (c < 0x20) {
          out << "\\u";
          const char* hex = "0123456789abcdef";
          out << "00" << hex[(c >> 4) & 0x0f] << hex[c & 0x0f];
        } else {
          out << c;
        }
    }
  }
  return out.str();
}

std::size_t find_field_value(const std::string& json, const std::string& key) {
  const std::string needle = "\"" + key + "\"";
  std::size_t pos = json.find(needle);
  if (pos == std::string::npos) {
    return std::string::npos;
  }
  pos = json.find(':', pos + needle.size());
  if (pos == std::string::npos) {
    return std::string::npos;
  }
  ++pos;
  while (pos < json.size() && std::isspace(static_cast<unsigned char>(json[pos]))) {
    ++pos;
  }
  return pos;
}

std::string get_string(const std::string& json, const std::string& key,
                       const std::string& fallback = "") {
  std::size_t pos = find_field_value(json, key);
  if (pos == std::string::npos || pos >= json.size() || json[pos] != '"') {
    return fallback;
  }
  ++pos;
  std::string out;
  while (pos < json.size()) {
    const char c = json[pos++];
    if (c == '"') {
      return out;
    }
    if (c == '\\' && pos < json.size()) {
      const char escaped = json[pos++];
      switch (escaped) {
        case '"':
        case '\\':
        case '/':
          out.push_back(escaped);
          break;
        case 'b':
          out.push_back('\b');
          break;
        case 'f':
          out.push_back('\f');
          break;
        case 'n':
          out.push_back('\n');
          break;
        case 'r':
          out.push_back('\r');
          break;
        case 't':
          out.push_back('\t');
          break;
        default:
          break;
      }
    } else {
      out.push_back(c);
    }
  }
  return fallback;
}

long long get_int(const std::string& json, const std::string& key, long long fallback = 0) {
  std::size_t pos = find_field_value(json, key);
  if (pos == std::string::npos) {
    return fallback;
  }
  const char* begin = json.c_str() + pos;
  char* end = nullptr;
  long long value = std::strtoll(begin, &end, 10);
  return end == begin ? fallback : value;
}

std::string extract_json_array(const std::string& json, const std::string& key) {
  std::size_t pos = find_field_value(json, key);
  if (pos == std::string::npos || pos >= json.size() || json[pos] != '[') {
    return "[]";
  }
  int depth = 0;
  bool in_string = false;
  bool escape = false;
  for (std::size_t i = pos; i < json.size(); ++i) {
    const char c = json[i];
    if (escape) {
      escape = false;
      continue;
    }
    if (c == '\\' && in_string) {
      escape = true;
      continue;
    }
    if (c == '"') {
      in_string = !in_string;
      continue;
    }
    if (in_string) {
      continue;
    }
    if (c == '[') {
      ++depth;
    } else if (c == ']') {
      --depth;
      if (depth == 0) {
        return json.substr(pos, i - pos + 1);
      }
    }
  }
  return "[]";
}

int base64_value(unsigned char c) {
  if (c >= 'A' && c <= 'Z') return c - 'A';
  if (c >= 'a' && c <= 'z') return c - 'a' + 26;
  if (c >= '0' && c <= '9') return c - '0' + 52;
  if (c == '+') return 62;
  if (c == '/') return 63;
  return -1;
}

std::vector<unsigned char> decode_base64(const std::string& input) {
  std::vector<unsigned char> out;
  int value = 0;
  int bits = -8;
  for (unsigned char c : input) {
    if (std::isspace(c) || c == '=') {
      continue;
    }
    const int decoded = base64_value(c);
    if (decoded < 0) {
      continue;
    }
    value = (value << 6) | decoded;
    bits += 6;
    if (bits >= 0) {
      out.push_back(static_cast<unsigned char>((value >> bits) & 0xff));
      bits -= 8;
    }
  }
  return out;
}

std::vector<float> pcm16_base64_to_float(const std::string& encoded) {
  const std::vector<unsigned char> bytes = decode_base64(encoded);
  std::vector<float> pcm;
  pcm.reserve(bytes.size() / 2);
  for (std::size_t i = 0; i + 1 < bytes.size(); i += 2) {
    const int16_t sample = static_cast<int16_t>(
        static_cast<uint16_t>(bytes[i]) | (static_cast<uint16_t>(bytes[i + 1]) << 8));
    pcm.push_back(std::max(-1.0f, std::min(1.0f, static_cast<float>(sample) / 32768.0f)));
  }
  return pcm;
}

std::string last_error() {
  const char* raw = parakeet_capi_last_error(g_ctx);
  return raw == nullptr ? "Unknown parakeet.cpp streaming error." : raw;
}

void cleanup_stream() {
  if (g_stream != nullptr) {
    parakeet_capi_stream_free(g_stream);
    g_stream = nullptr;
  }
}

void cleanup_model() {
  cleanup_stream();
  if (g_ctx != nullptr) {
    parakeet_capi_free(g_ctx);
    g_ctx = nullptr;
  }
  g_model_path.clear();
  g_full_text.clear();
}

void emit_error(const std::string& message) {
  emit("{\"type\":\"error\",\"ok\":false,\"message\":\"" + json_escape(message) + "\"}");
}

void handle_health() {
  std::ostringstream out;
  out << "{\"type\":\"health\",\"ok\":true,\"protocolVersion\":1,"
      << "\"abiVersion\":" << parakeet_capi_abi_version() << ","
      << "\"message\":\"parakeet.cpp streaming worker is available.\"}";
  emit(out.str());
}

void handle_load(const std::string& line) {
  const std::string model_path = get_string(line, "modelPath");
  const std::string language = get_string(line, "language", "auto");
  if (model_path.empty()) {
    emit_error("Missing modelPath.");
    return;
  }

  if (g_ctx == nullptr || g_model_path != model_path) {
    cleanup_model();
    g_ctx = parakeet_capi_load(model_path.c_str());
    if (g_ctx == nullptr) {
      emit_error("Failed to load GGUF model.");
      return;
    }
    g_model_path = model_path;
  } else {
    cleanup_stream();
  }

  g_language = language.empty() ? "auto" : language;
  g_stream = parakeet_capi_stream_begin_lang(g_ctx, g_language.c_str());
  if (g_stream == nullptr) {
    emit_error(last_error());
    return;
  }
  g_full_text.clear();
  emit("{\"type\":\"loaded\",\"ok\":true,\"modelPath\":\"" + json_escape(g_model_path) +
       "\",\"language\":\"" + json_escape(g_language) + "\"}");
}

void append_delta(const std::string& delta) {
  if (delta.empty()) {
    return;
  }
  if (g_full_text.empty()) {
    g_full_text = delta;
    return;
  }
  const char first = delta.front();
  if (std::ispunct(static_cast<unsigned char>(first))) {
    g_full_text += delta;
  } else {
    g_full_text += " ";
    g_full_text += delta;
  }
}

void emit_transcript(const std::string& kind, long long sequence, const std::string& engine_json,
                     const std::string& delta) {
  emit("{\"type\":\"" + kind + "\",\"ok\":true,\"sequence\":" + std::to_string(sequence) +
       ",\"text\":\"" + json_escape(g_full_text) + "\",\"delta\":\"" + json_escape(delta) +
       "\",\"finalizedText\":\"" + json_escape(delta) + "\",\"engineJson\":" + engine_json +
       "}");
}

void handle_audio(const std::string& line) {
  if (g_stream == nullptr) {
    emit_error("No active stream. Send load before audio.");
    return;
  }
  const long long sequence = get_int(line, "sequence", 0);
  const std::vector<float> pcm = pcm16_base64_to_float(get_string(line, "pcm16"));
  if (pcm.empty()) {
    return;
  }

  char* raw = parakeet_capi_stream_feed_json(g_stream, pcm.data(), static_cast<int>(pcm.size()));
  if (raw == nullptr) {
    emit_error(last_error());
    return;
  }
  const std::string engine_json(raw);
  parakeet_capi_free_string(raw);
  const std::string delta = get_string(engine_json, "text");
  append_delta(delta);
  if (!delta.empty()) {
    emit_transcript("partial", sequence, engine_json, delta);
  }
}

void handle_finalize(const std::string& line) {
  if (g_stream == nullptr) {
    emit_error("No active stream to finalize.");
    return;
  }
  const long long sequence = get_int(line, "sequence", 0);
  char* raw = parakeet_capi_stream_finalize_json(g_stream);
  if (raw == nullptr) {
    emit_error(last_error());
    return;
  }
  const std::string engine_json(raw);
  parakeet_capi_free_string(raw);
  const std::string delta = get_string(engine_json, "text");
  append_delta(delta);
  emit_transcript("final", sequence, engine_json, delta);
  cleanup_stream();
}

void handle_cancel() {
  cleanup_stream();
  g_full_text.clear();
  emit("{\"type\":\"cancelled\",\"ok\":true}");
}

void handle_line(const std::string& line) {
  const std::string type = get_string(line, "type");
  if (type == "health") {
    handle_health();
  } else if (type == "load") {
    handle_load(line);
  } else if (type == "audio") {
    handle_audio(line);
  } else if (type == "finalize") {
    handle_finalize(line);
  } else if (type == "cancel") {
    handle_cancel();
  } else {
    emit_error("Unknown request type: " + type);
  }
}

}  // namespace

int main(int argc, char** argv) {
  if (argc > 1 && std::string(argv[1]) == "--health") {
    handle_health();
    return 0;
  }

  std::string line;
  while (std::getline(std::cin, line)) {
    if (line.empty()) {
      continue;
    }
    handle_line(line);
  }
  cleanup_model();
  return 0;
}
