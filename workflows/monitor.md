---
description: Server có ổn không? Cài giám sát ngay.
---

# /monitor - DevOps & Observability

## 🔴 TRI-PART LINKAGE & MEMPALACE GATEWAY (MANDATORY)

**STEP 0: Context Initialization**
1. **Mempalace Check:** You MUST invoke mempalace_status or search tools to retrieve the latest architecture and state before taking any action. Do not guess the project structure.
2. **Tri-Part Linkage:** Before writing code or making system changes, you MUST read .agents/.shared/SHARED_INDEX.md and .agents/skills/SKILL_INDEX.md to load the exact DNA and skills required for this task.

**STEP END: State Persistence**
- When the task is completed, you MUST invoke mempalace_diary_write to log your actions and decisions into the permanent memory graph.

$ARGUMENTS

---

## Task
Setup logging and monitoring for Production.

### Steps:
1.  **Logging**: Config structured logging (JSON format).
2.  **Error Tracking**: Setup Sentry/LogRocket/Datadog.
3.  **Health Checks**: Create `/health` API endpoint.
4.  **Alerting**: Define thresholds for CPU/Memory warnings.

---

## Usage
```
/monitor setup sentry   # Install Sentry SDK
/monitor check          # Verify observability
```

