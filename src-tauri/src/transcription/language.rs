/// Maps an app language code to the value passed to `whisper-cli --language`.
///
/// whisper.cpp accepts ISO 639-1 codes ("de", "en", …) directly and also "auto".
/// This function normalises caller-provided values and returns the correct CLI arg.
pub fn to_whisper_lang(code: &str) -> String {
    match code.trim().to_lowercase().as_str() {
        "de" | "german" => "de",
        "en" | "english" => "en",
        "fr" | "french" => "fr",
        "es" | "spanish" => "es",
        "it" | "italian" => "it",
        "pt" | "portuguese" => "pt",
        "nl" | "dutch" => "nl",
        "pl" | "polish" => "pl",
        "ru" | "russian" => "ru",
        "ja" | "japanese" => "ja",
        "zh" | "chinese" => "zh",
        "auto" | "" => "auto",
        other => return other.to_string(), // pass through unknown codes
    }
    .to_string()
}
