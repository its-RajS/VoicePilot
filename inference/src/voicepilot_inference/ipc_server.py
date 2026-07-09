import asyncio
import json
import sys
from typing import Any

from voicepilot_inference.core.pipeline import PipelineOrchestrator


def emit(message: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(message) + "\n")
    sys.stdout.flush()


def main() -> int:
    pipeline = PipelineOrchestrator()

    for raw_line in sys.stdin:
        line = raw_line.strip()
        if not line:
            continue

        try:
            request = json.loads(line)
            request_id = int(request.get("request_id", 0))
            request_type = request.get("type")

            if request_type == "health_check":
                emit({"type": "health_status", "request_id": request_id, "status": "ok"})
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
                        request.get("transcript", ""), request.get("mode", "engineering")
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
