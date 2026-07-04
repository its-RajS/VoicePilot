from enum import StrEnum

class PipelineState(StrEnum):
    BUFFERING = "buffering"
    VAD_ACTIVE = "vad_active"
    STT_STREAMING = "stt_streaming"
    LLM_CLEANUP = "llm_cleanup"
    COMPLETE = "complete"
