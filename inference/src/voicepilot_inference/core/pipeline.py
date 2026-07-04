from dataclasses import dataclass

@dataclass
class PipelineResult:
    raw_transcript: str
    cleaned_prompt: str

class PipelineOrchestrator:
    async def cleanup_transcript(self, transcript: str, mode: str = "engineering") -> PipelineResult:
        # TODO: call Ollama via LlmClient; fall back to raw transcript on failure.
        return PipelineResult(raw_transcript=transcript, cleaned_prompt=transcript.strip())
