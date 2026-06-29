#!/usr/bin/env python3
"""Optional NVIDIA NeMo worker for LocalVoice.

The main Tauri process starts this script as a separate process and talks to it
over newline-delimited JSON. The worker is intentionally optional: health checks
must fail cleanly when Python, torch, or NeMo are not installed.
"""

from __future__ import annotations

import argparse
import json
import sys
import traceback
from typing import Any


MODEL: Any | None = None
MODEL_PATH: str | None = None
STREAMING_BUFFER: bytearray = bytearray()


def emit(payload: dict[str, Any]) -> None:
    print(json.dumps(payload, ensure_ascii=False), flush=True)


def health() -> None:
    try:
        import nemo.collections.asr as nemo_asr  # noqa: F401

        try:
            import torch

            cuda_available = bool(torch.cuda.is_available())
        except Exception:
            cuda_available = False

        emit(
            {
                "type": "health",
                "ok": True,
                "message": "Python and NeMo ASR are available.",
                "pythonPath": sys.executable,
                "detail": f"cudaAvailable={cuda_available}",
            }
        )
    except Exception as exc:
        emit(
            {
                "type": "health",
                "ok": False,
                "message": "NeMo ASR is not available in this Python environment.",
                "pythonPath": sys.executable,
                "detail": str(exc),
            }
        )


def load_model(model_path: str) -> None:
    global MODEL, MODEL_PATH
    if MODEL is not None and MODEL_PATH == model_path:
        emit({"type": "loaded", "ok": True, "modelPath": model_path})
        return

    import nemo.collections.asr as nemo_asr

    MODEL = nemo_asr.models.ASRModel.restore_from(restore_path=model_path)
    MODEL.eval()
    MODEL_PATH = model_path
    emit({"type": "loaded", "ok": True, "modelPath": model_path})


def transcribe_file(audio_path: str, language: str) -> None:
    if MODEL is None:
        raise RuntimeError("No .nemo model is loaded.")

    kwargs: dict[str, Any] = {}
    if language and language != "auto":
        kwargs["target_lang"] = language

    try:
        result = MODEL.transcribe([audio_path], timestamps=True, **kwargs)[0]
    except TypeError:
        result = MODEL.transcribe([audio_path], **kwargs)[0]

    text = getattr(result, "text", None)
    if text is None:
        text = str(result)

    timestamp = getattr(result, "timestamp", None) or {}
    words = []
    for item in timestamp.get("word", []) if isinstance(timestamp, dict) else []:
        word_text = item.get("word") or item.get("text") or item.get("segment")
        if not word_text:
            continue
        words.append(
            {
                "startMs": seconds_to_ms(item.get("start", 0.0)),
                "endMs": seconds_to_ms(item.get("end", 0.0)),
                "text": word_text,
                "confidence": item.get("confidence"),
            }
        )

    segments = []
    for item in timestamp.get("segment", []) if isinstance(timestamp, dict) else []:
        segment_text = item.get("segment") or item.get("text")
        if not segment_text:
            continue
        segments.append(
            {
                "startMs": seconds_to_ms(item.get("start", 0.0)),
                "endMs": seconds_to_ms(item.get("end", 0.0)),
                "text": segment_text,
                "confidence": item.get("confidence"),
            }
        )

    emit(
        {
            "type": "transcription",
            "ok": True,
            "text": text,
            "segments": segments,
            "words": words,
            "detectedLanguage": None,
        }
    )


def handle_stream_audio(request: dict[str, Any]) -> None:
    # NeMo exposes several model-specific streaming APIs. LocalVoice only enables
    # this path once the installed runtime provides a compatible warm stream
    # method; until then the Rust side falls back to WAV transcription on stop.
    if not hasattr(MODEL, "streaming_transcribe"):
        emit(
            {
                "type": "error",
                "ok": False,
                "message": "The installed NeMo model/runtime does not expose LocalVoice-compatible streaming.",
            }
        )
        return

    emit(
        {
            "type": "error",
            "ok": False,
            "message": "LocalVoice-compatible NeMo streaming is not enabled for this runtime yet.",
        }
    )


def finalize_stream() -> None:
    emit(
        {
            "type": "error",
            "ok": False,
            "message": "NeMo streaming finalization is not enabled for this runtime yet.",
        }
    )


def cancel_stream() -> None:
    STREAMING_BUFFER.clear()
    emit({"type": "cancelled", "ok": True})


def seconds_to_ms(value: Any) -> int:
    try:
        return int(round(float(value) * 1000.0))
    except Exception:
        return 0


def handle_request(request: dict[str, Any]) -> None:
    kind = request.get("type")
    if kind == "health":
        health()
    elif kind == "load":
        load_model(str(request["modelPath"]))
    elif kind == "transcribe_file":
        transcribe_file(str(request["audioPath"]), str(request.get("language", "auto")))
    elif kind in {"audio", "stream_chunk"}:
        handle_stream_audio(request)
    elif kind == "finalize":
        finalize_stream()
    elif kind == "cancel":
        cancel_stream()
    else:
        emit({"type": "error", "ok": False, "message": f"Unknown request type: {kind}"})


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--health", action="store_true")
    args = parser.parse_args()

    if args.health:
        health()
        return 0

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            handle_request(json.loads(line))
        except Exception as exc:
            emit(
                {
                    "type": "error",
                    "ok": False,
                    "message": str(exc),
                    "detail": traceback.format_exc(limit=8),
                }
            )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
