---
description: Dự án đang đến đâu rồi? Xem Dashboard báo cáo.
---

# /status - Show Status

## 🔴 TRI-PART LINKAGE & MEMPALACE GATEWAY (MANDATORY)

**STEP 0: Context Initialization**
1. **Mempalace Check:** You MUST invoke mempalace_status or search tools to retrieve the latest architecture and state before taking any action. Do not guess the project structure.
2. **Tri-Part Linkage:** Before writing code or making system changes, you MUST read .agents/.shared/SHARED_INDEX.md and .agents/skills/SKILL_INDEX.md to load the exact DNA and skills required for this task.

**STEP END: State Persistence**
- When the task is completed, you MUST invoke mempalace_diary_write to log your actions and decisions into the permanent memory graph.

$ARGUMENTS

---

## Task

Show current project and agent status.

### What It Shows

1. **Project Info**
   - Project name and path
   - Tech stack
   - Current features

2. **Agent Status Board**
   - Which agents are running
   - Which tasks are completed
   - Pending work

3. **File Statistics**
   - Files created count
   - Files modified count

4. **Preview Status**
   - Is server running
   - URL
   - Health check

---

## Example Output

```
=== Project Status ===

 Project: my-ecommerce
 Path: C:/projects/my-ecommerce
️ Type: nextjs-ecommerce
 Status: active

 Tech Stack:
   Framework: next.js
   Database: postgresql
   Auth: clerk
   Payment: stripe

 Features (5):
   • product-listing
   • cart
   • checkout
   • user-auth
   • order-history

 Pending (2):
   • admin-panel
   • email-notifications

 Files: 73 created, 12 modified

=== Agent Status ===

 database-architect → Completed
 backend-specialist → Completed
🔄 frontend-specialist → Dashboard components (60%)
 test-engineer → Waiting

=== Preview ===

 URL: http://localhost:3000
 Health: OK
```

---

## Technical

Status uses these scripts:
- `python .agent/scripts/session_manager.py status`
- `python .agent/scripts/auto_preview.py status`

