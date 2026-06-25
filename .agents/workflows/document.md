---
description: Lười viết docs? Để AI tự viết cho.
---

# /document - Auto Documentation

## 🔴 TRI-PART LINKAGE & MEMPALACE GATEWAY (MANDATORY)

**STEP 0: Context Initialization**
1. **Mempalace Check:** You MUST invoke mempalace_status or search tools to retrieve the latest architecture and state before taking any action. Do not guess the project structure.
2. **Tri-Part Linkage:** Before writing code or making system changes, you MUST read .agents/.shared/SHARED_INDEX.md and .agents/skills/SKILL_INDEX.md to load the exact DNA and skills required for this task.

**STEP END: State Persistence**
- When the task is completed, you MUST invoke mempalace_diary_write to log your actions and decisions into the permanent memory graph.

$ARGUMENTS

---

## Task
Keep documentation in sync with code.

### Steps:
1.  **Scan Codebase**: Read function signatures and comments.
2.  **Generate API Docs**: Create Swagger/OpenAPI spec.
3.  **Update README**: Refresh feature list and usage examples.
4.  **Internal Docs**: Update `ARCHITECTURE.md` if structure changed.

---

## Usage
```
/document api      # Generate API reference
/document readme   # Update README.md
/document all      # Full documentation refresh
```

