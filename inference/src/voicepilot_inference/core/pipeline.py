from dataclasses import dataclass

from voicepilot_inference.engines.ollama_client import DEFAULT_MODEL, OllamaClient


@dataclass
class PipelineResult:
    raw_transcript: str
    cleaned_prompt: str


class PipelineOrchestrator:
    def __init__(self, ollama_client: OllamaClient | None = None) -> None:
        self._ollama = ollama_client or OllamaClient()

    async def cleanup_transcript(
        self, transcript: str, mode: str = "engineering", model: str | None = None
    ) -> PipelineResult:
        try:
            cleaned = self._ollama.cleanup_prompt(transcript, mode=mode, model=model or DEFAULT_MODEL)
        except Exception:
            # ponytail: any Ollama failure (down, model missing, bad response)
            # falls back to the raw transcript rather than erroring the request.
            cleaned = transcript.strip()
        return PipelineResult(raw_transcript=transcript, cleaned_prompt=cleaned)
