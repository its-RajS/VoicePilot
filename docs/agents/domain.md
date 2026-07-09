# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

## Before exploring, read these

- **`docs/CONTEXT.md`** — glossary, hardware assumptions, Linux desktop prerequisites.
- **`docs/adr/`** — read ADRs that touch the area you're about to work in. Doesn't exist yet; created lazily by `/domain-modeling` when a decision is worth recording.

If `docs/adr/` doesn't exist, proceed silently — don't flag its absence.

## File structure

Single-context repo:

```
/
├── docs/
│   ├── CONTEXT.md
│   └── adr/
└── src/, src-tauri/, inference/
```

## Use the glossary's vocabulary

When your output names a domain concept (issue title, refactor proposal, hypothesis, test name), use the term as defined in `docs/CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

If the concept you need isn't in the glossary yet, that's a signal — either you're inventing language the project doesn't use (reconsider) or there's a real gap (note it for `/domain-modeling`).

## Flag ADR conflicts

If your output contradicts an existing ADR, surface it explicitly rather than silently overriding.
