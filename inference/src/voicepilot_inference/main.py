from fastapi import FastAPI
from voicepilot_inference.api.health import router as health_router

app = FastAPI(title="VoicePilot Inference", version="0.1.0")
app.include_router(health_router, prefix="/api")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("voicepilot_inference.main:app", host="127.0.0.1", port=8765, reload=True)
