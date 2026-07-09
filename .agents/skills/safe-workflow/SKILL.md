---
name: safe-workflow
description: >
  SAFe development workflow guidance including branch naming conventions,
  commit message format, rebase-first workflow, and CI validation. Use when
  starting work on a Linear ticket, preparing commits, creating branches,
  writing PR descriptions, or asking about contribution guidelines.
---

# SAFe Workflow Skill

> **TEMPLATE**: This skill uses `VOICE` as a placeholder. Replace with your project's ticket prefix (e.g., `WOR`, `PROJ`, `FEAT`).

## Purpose

Enforce SAFe-compliant git workflow with standardized branch naming, commit message format, and rebase-first merge strategy.

## When This Skill Applies

- Starting work on a ticket
- Creating commits or branches
- Asking about PR workflow or contribution guidelines
- Asking "how should I commit this?"

## Branch Naming Convention

**Format**: `VOICE-{number}-{short-description}`

```text
# Good
VOICE-447-create-safe-workflow-skill
VOICE-123-fix-login-redirect

# Bad
feature/add-dark-mode       (missing ticket number)
john-new-feature            (personal naming)
```

## Commit Message Format

**Format**: `type(scope): description [VOICE-XXX]`

| Type       | When to Use                   |
| ---------- | ----------------------------- |
| `feat`     | New feature                   |
| `fix`      | Bug fix                       |
| `docs`     | Documentation only            |
| `refactor` | Code restructuring            |
| `test`     | Adding or updating tests      |
| `chore`    | Maintenance, dependencies     |

```text
feat(harness): create safe-workflow skill [VOICE-447]
fix(auth): resolve login redirect [VOICE-57]
```

## Rebase-First Workflow

```bash
# 1. Start from latest main
git checkout main && git pull origin main

# 2. Create feature branch
git checkout -b VOICE-{number}-{description}

# 3. Make commits
git commit -m "type(scope): description [VOICE-XXX]"

# 4. Before pushing - rebase
git fetch origin && git rebase origin/main

# 5. Push with force-with-lease
git push --force-with-lease
```

## Pre-PR Checklist

1. Branch name follows convention
2. All commits have ticket reference
3. Rebased on latest main
4. CI passes: `{{CI_VALIDATE_COMMAND}}`

## Evidence Template

When closing a ticket, attach evidence:

```markdown
**Work Evidence**

**Ticket**: VOICE-XXX
**Branch**: VOICE-XXX-description
**PR**: #NNN

**Commits:**
- type(scope): description

**Validation:**
- CI: PASS
- Tests: X/X passing
- Lint: Clean
```

## Reference

- **CONTRIBUTING.md** - Full contributor guide
- **AGENTS.md** - Development context and agent roles
