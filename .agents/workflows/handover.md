---
description: Generate a standardized thread handover document and migration prompt when context window exceeds 20% or a logic block is completed, preserving state and project rules.
---

# WORKFLOW: /handover

> **Default Temperature**: `0.2` (Baseline Documentation & State Preservation)
> **Goal**: Package the complete technical state, verified features, file map, and next steps into a standardized Handover Document (`docs/handover/handover-YYYY-MM-DD.md`). Generate a compact **New Chat Bootstrapping Prompt** allowing seamless session migration without context degradation or loss of critical project guardrails.

---

## 1. TRIGGER

This workflow is triggered when:
1. The User issues the explicit command: `/handover`
2. Context window usage exceeds **20%** (Proactive New Chat directive).
3. A major feature module or testing milestone is completed and ready for session reset.

---

## 2. HANDOVER CORE RULES & REQUIREMENTS (HANDSHAKE PROTOCOL)

Every handover MUST contain the following 7 core components:

```
 ┌────────────────────────────────────────────────────────────────────────┐
 │ 1. SESSION CONTEXT & PROJECT ARCHITECTURE                              │
 │    • Project Name, Tech Stack, Operating Environment                  │
 │    • Active Domain (Backend / Frontend / DevOps / Security / Modding) │
 │    • Active Branch / PR / Sprint status                               │
 ├────────────────────────────────────────────────────────────────────────┤
 │ 2. VERIFIED & COMPLETED FEATURES                                       │
 │    • List of implemented features verified 100%                       │
 │    • File locations, API endpoints, test coverage                     │
 ├────────────────────────────────────────────────────────────────────────┤
 │ 3. AUTOMATED TEST HARNESS STATUS                                       │
 │    • Test scripts, suites, and last-run outcomes                      │
 │    • Passing / Failing / Pending tests                                │
 ├────────────────────────────────────────────────────────────────────────┤
 │ 4. IMPORTANT FILE & ARTIFACT MAP                                       │
 │    • Table mapping files, paths, and technical descriptions            │
 ├────────────────────────────────────────────────────────────────────────┤
 │ 5. NEXT STEPS QUEUE (ROADMAP)                                          │
 │    • Ordered list of pending tasks for the next chat session          │
 ├────────────────────────────────────────────────────────────────────────┤
 │ 6. TECHNICAL CONSTRAINTS & YAGNI RULES                                 │
 │    • Code conventions, thread safety, defensive checks, DO NOTs       │
 ├────────────────────────────────────────────────────────────────────────┤
 │ 7. NEW CHAT BOOTSTRAPPING PROMPT                                       │
 │    • Compact, copy-pasteable prompt to resume work in a New Chat      │
 └────────────────────────────────────────────────────────────────────────┘
```

---

## 3. EXECUTION WORKFLOW (4-STEP PROCESS)

### STEP 1: CONTEXT & ARTIFACT AUDIT
1. Read active workspace state, recent git modifications, and current Mem0 facts via `search_facts`.
2. Verify all completed features against test outputs and build status.

### STEP 2: GENERATE HANDOVER DOCUMENT
Create or update `docs/handover/handover-YYYY-MM-DD.md` in the target project directory.

### STEP 3: MEMORY PERSISTENCE (MEM0 CHECKPOINT)
Execute `mem0/add_fact` to persist the handover summary to long-term memory:
- Fact payload: Project name, tech stack, completed features, active handover doc path.

### STEP 4: OUTPUT NEW CHAT BOOTSTRAPPING PROMPT
Present the generated Handover Document link to the User, along with a copy-pasteable **New Chat Bootstrapping Prompt**.

---

## 4. STANDARDIZED HANDOVER DOCUMENT TEMPLATE

When generating `docs/handover/handover-YYYY-MM-DD.md`, strictly use this format:

```markdown
# handover-YYYY-MM-DD.md — [Project Name]

## 1. Session Context & Project Architecture
- **Project**: [Project / Application Name]
- **Tech Stack**: [e.g. Spring Boot 3 / Angular 17 / PostgreSQL / Redis]
- **Active Domain**: [Backend / Frontend / DevOps / Security / Modding]
- **Environment**: [Local Dev / Staging / Production]

## 2. Completed & Verified Features
### [Feature 1 Name]
- **Location**: [File Path / API Endpoint]
- **Description**: [Summary of implementation]
- **Status**: Verified 100%

### [Feature 2 Name]
- **Location**: [File Path]
- **Status**: Verified 100%

## 3. Automated Test Harness Status
- **Test Suite**: `[path/to/test_suite]`
- **Last Result**: Passed / Failed / Pending
- **Coverage**: [XX%]

## 4. Important File & Artifact Map
| File | Path | Description |
| :--- | :--- | :--- |
| **[File 1]** | `[path]` | [Description] |
| **[File 2]** | `[path]` | [Description] |

## 5. Next Steps Queue (Roadmap for Next Session)
1. [Next Task 1]
2. [Next Task 2]
3. [Next Task 3]

## 6. Technical Constraints & YAGNI Guardrails
- [Constraint 1]
- [Constraint 2]
```

---

## 5. NEW CHAT BOOTSTRAPPING PROMPT TEMPLATE

At the end of `/handover` execution, present the following prompt to the user:

```markdown
### NEW CHAT BOOTSTRAPPING PROMPT

Copy and paste the prompt below into a **New Chat** to resume immediately:

> *"I am starting a new chat session to continue development on [Project Name].*
> *Please load context from the handover document at: [handover-YYYY-MM-DD.md](file:///path/to/handover-YYYY-MM-DD.md).*
>
> *Session Highlights:*
> *- Tech Stack: [Stack]*
> *- Active Domain: [Domain]*
> *- Current Progress: Completed [List 2-3 main features].*
> *- Immediate Next Task: [Next Task 1 from Next Steps Queue].*
>
> *Directives: Read the handover document first via view_file, query Mem0 via search_facts, enforce YAGNI minimalist code, and begin with: 'Thưa chủ nhân,'"*
```
