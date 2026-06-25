---
description: Chưa biết bắt đầu từ đâu? Lập kế hoạch trước.
---

# /plan - Project Planning Mode

$ARGUMENTS

---

## 🔴 CRITICAL RULES

1. **NO CODE WRITING** - This command creates plan file only
2. **Tri-Part Linkage:** Before writing code or making system changes, you MUST read `.agents/.shared/SHARED_INDEX.md` and `.agents/skills/SKILL_INDEX.md` to load the exact DNA and skills required for this task.
3. **Use mk12-system-architect agent** - NOT Antigravity Agent's native Plan mode
4. **Socratic Gate** - Ask clarifying questions before planning
5. **Dynamic Naming** - Plan file named based on task

---

## Task

Use the `mk12-system-architect` agent with this context:

```
CONTEXT:
- User Request: $ARGUMENTS
- Mode: PLANNING ONLY (no code)
- Output: docs/PLAN-{task-slug}.md (dynamic naming)

NAMING RULES:
1. Extract 2-3 key words from request
2. Lowercase, hyphen-separated
3. Max 30 characters
4. Example: "e-commerce cart" → PLAN-ecommerce-cart.md

RULES:
1. Follow Socratic Gate (ask clarifying questions before writing the plan).
2. Utilize the `mk12` System Architect and `plan-writing` skill for structuring.
3. 🔴 **MEMPALACE GATEWAY**: Use `mempalace_status` or search to pull architecture context before writing.
4. Create PLAN-{slug}.md with task breakdown
5. DO NOT write any code files
6. REPORT the exact file name created
```

---

## Expected Output

| Deliverable | Location |
|-------------|----------|
| Project Plan | `docs/PLAN-{task-slug}.md` |
| Task Breakdown | Inside plan file |
| Agent Assignments | Inside plan file |
| Verification Checklist | Phase X in plan file |

---

## After Planning

Tell user:
```
[OK] Plan created: docs/PLAN-{slug}.md

Next steps:
- Review the plan
- Run `/create` to start implementation
- Or modify plan manually
```

---

## Naming Examples

| Request | Plan File |
|---------|-----------|
| `/plan e-commerce site with cart` | `docs/PLAN-ecommerce-cart.md` |
| `/plan mobile app for fitness` | `docs/PLAN-fitness-app.md` |
| `/plan add dark mode feature` | `docs/PLAN-dark-mode.md` |
| `/plan fix authentication bug` | `docs/PLAN-auth-fix.md` |
| `/plan SaaS dashboard` | `docs/PLAN-saas-dashboard.md` |

---

## Usage

```
/plan e-commerce site with cart
/plan mobile app for fitness tracking
/plan SaaS dashboard with analytics
```
