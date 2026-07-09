import os

import httpx

from voicepilot_inference.prompts.engineering import SYSTEM_PROMPTS

DEFAULT_HOST = os.environ.get("OLLAMA_HOST", "http://127.0.0.1:11434")
DEFAULT_MODEL = "qwen3:8b"
CLEANUP_TEMPERATURE = 0.3


class OllamaUnavailable(Exception):
    pass


class OllamaClient:
    def __init__(self, host: str = DEFAULT_HOST, timeout: float = 10.0) -> None:
        self.host = host.rstrip("/")
        self._client = httpx.Client(base_url=self.host, timeout=timeout)

    def is_running(self) -> bool:
        try:
            response = self._client.get("/api/tags")
            return response.status_code == 200
        except httpx.HTTPError:
            return False

    def list_models(self) -> list[str]:
        response = self._client.get("/api/tags")
        response.raise_for_status()
        return [model["name"] for model in response.json().get("models", [])]

    def has_model(self, name: str) -> bool:
        return name in self.list_models()

    def pull_model(self, name: str) -> None:
        with self._client.stream("POST", "/api/pull", json={"name": name}) as response:
            response.raise_for_status()
            for _ in response.iter_lines():
                pass  # ponytail: drain progress stream, no UI hookup yet

    def cleanup_prompt(self, text: str, mode: str = "engineering", model: str = DEFAULT_MODEL) -> str:
        system_prompt = SYSTEM_PROMPTS.get(mode, SYSTEM_PROMPTS["engineering"])
        response = self._client.post(
            "/api/generate",
            json={
                "model": model,
                "prompt": text,
                "system": system_prompt,
                "stream": False,
                "options": {"temperature": CLEANUP_TEMPERATURE},
            },
        )
        response.raise_for_status()
        return response.json()["response"].strip()
