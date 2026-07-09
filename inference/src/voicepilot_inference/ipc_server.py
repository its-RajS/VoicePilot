import asyncio
import json
import sys
from typing import Any

from voicepilot_inference.core.pipeline import PipelineOrchestrator

PROTOCOL_VERSION = 1


def emit(message: dict[str, Any]) -> None:
    payload = json.dumps(message).encode("utf-8")
    stdout = sys.stdout.buffer
    stdout.write(len(payload).to_bytes(4, "big"))
    stdout.write(payload)
    stdout.flush()


def read_frame() -> bytes | None:
    stdin = sys.stdin.buffer
    len_bytes = stdin.read(4)
    if not len_bytes:
        return None
    if len(len_bytes) < 4:
        raise EOFError("truncated frame length header")
    length = int.from_bytes(len_bytes, "big")
    payload = stdin.read(length)
    if len(payload) < length:
        raise EOFError("truncated frame payload")
    return payload


def main() -> int:
    pipeline = PipelineOrchestrator()

    while True:
        try:
            frame = read_frame()
        except EOFError:
            break
        if frame is None:
            break

        try:
            request = json.loads(frame)
            request_id = int(request.get("request_id", 0))
            request_type = request.get("type")

            if request_type == "health_check":
                emit(
                    {
                        "type": "health_status",
                        "request_id": request_id,
                        "status": "ok",
                        "protocol_version": PROTOCOL_VERSION,
                    }
                )
            elif request_type == "start_recording":
                emit({"type": "recording_started", "request_id": request_id})
            elif request_type == "audio_chunk":
                pcm_bytes = request.get("pcm_s16le") or []
                emit(
                    {
                        "type": "partial_transcript",
                        "request_id": request_id,
                        "text": f"received {len(pcm_bytes)} bytes",
                        "confidence": None,
                    }
                )
            elif request_type == "cleanup_request":
                result = asyncio.run(
                    pipeline.cleanup_transcript(
                        request.get("transcript", ""),
                        request.get("mode", "engineering"),
                        request.get("model"),
                    )
                )
                emit(
                    {
                        "type": "cleanup_response",
                        "request_id": request_id,
                        "raw_transcript": result.raw_transcript,
                        "cleaned_prompt": result.cleaned_prompt,
                    }
                )
            else:
                emit(
                    {
                        "type": "error",
                        "request_id": request_id,
                        "message": f"unsupported message type: {request_type}",
                    }
                )
        except Exception as exc:
            emit({"type": "error", "request_id": 0, "message": str(exc)})

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
