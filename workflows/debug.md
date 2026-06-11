---
description: Gặp lỗi khó sửa? Để AI soi log và sửa giúp bạn.
---

# /debug - Systematic Problem Investigation

## 🔴 TRI-PART LINKAGE & MEMPALACE GATEWAY (MANDATORY)

**STEP 0: Context Initialization**
1. **Mempalace Check:** You MUST invoke mempalace_status or search tools to retrieve the latest architecture and state before taking any action. Do not guess the project structure.
2. **Tri-Part Linkage:** Before writing code or making system changes, you MUST read .agents/.shared/SHARED_INDEX.md and .agents/skills/SKILL_INDEX.md to load the exact DNA and skills required for this task.

**STEP END: State Persistence**
- When the task is completed, you MUST invoke mempalace_diary_write to log your actions and decisions into the permanent memory graph.

$ARGUMENTS

---

## Purpose

This command activates DEBUG mode for systematic investigation of issues, errors, or unexpected behavior.

---

## Behavior

When `/debug` is triggered:

1. **Gather information**
   - Error message
   - Reproduction steps
   - Expected vs actual behavior
   - Recent changes

2. **Form hypotheses**
   - List possible causes
   - Order by likelihood

3. **Investigate systematically**
   - Test each hypothesis
   - Check logs, data flow
   - Use elimination method

4. **Fix and prevent**
   - Apply fix
   - Explain root cause
   - Add prevention measures

---

## Output Format

```markdown
##  Debug: [Issue]

### 1. Symptom
[What's happening]

### 2. Information Gathered
- Error: `[error message]`
- File: `[filepath]`
- Line: [line number]

### 3. Hypotheses
1.  [Most likely cause]
2.  [Second possibility]
3.  [Less likely cause]

### 4. Investigation

**Testing hypothesis 1:**
[What I checked] → [Result]

**Testing hypothesis 2:**
[What I checked] → [Result]

### 5. Root Cause
 **[Explanation of why this happened]**

### 6. Fix
```[language]
// Before
[broken code]

// After
[fixed code]
```

### 7. Prevention
️ [How to prevent this in the future]
```

---

## Examples

```
/debug login not working
/debug API returns 500
/debug form doesn't submit
/debug data not saving
```

---

## Key Principles

- **Ask before assuming** - get full error context
- **Test hypotheses** - don't guess randomly
- **Explain why** - not just what to fix
- **Prevent recurrence** - add tests, validation

