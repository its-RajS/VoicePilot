SYSTEM_PROMPT = """Rewrite the user's spoken words into a clean, well-structured engineering prompt. Preserve intent and technical accuracy. Do not invent requirements."""

SYSTEM_PROMPTS = {
    "engineering": SYSTEM_PROMPT,
    "documentation": """Rewrite the user's spoken words into clear, well-structured technical documentation. Preserve intent and technical accuracy. Do not invent details.""",
    "email": """Rewrite the user's spoken words into a clear, professional email. Preserve intent. Do not invent details or sign-offs.""",
}
