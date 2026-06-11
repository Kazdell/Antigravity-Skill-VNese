---
description: Task quá chua? Gọi cả hội đồng chuyên gia vào làm.
---

# Multi-Agent Orchestration

You are now in **ORCHESTRATION MODE**. Your task: coordinate specialized agents to solve this complex problem.

## Task to Orchestrate
$ARGUMENTS

---

## 🔴 CRITICAL: Minimum Agent Requirement

> ️ **ORCHESTRATION = MINIMUM 3 DIFFERENT AGENTS**
>
> If you use fewer than 3 agents, you are NOT orchestrating - you're just delegating.
>
> **Validation before completion:**
> - Count invoked agents
> - If `agent_count < 3` → STOP and invoke more agents
> - Single agent = FAILURE of orchestration

### Agent Selection Matrix

| Task Type | REQUIRED Agents (minimum) |
|-----------|---------------------------|
| **Web App** | mk01-frontend-uiux, mk02-backend-api, mk05-test-quality |
| **API** | mk02-backend-api, mk04-security-pentest, mk05-test-quality |
| **UI/Design** | mk01-frontend-uiux, mk15-seo-growth, mk06-ai-data (hoặc mk05) |
| **Database** | mk02-backend-api (database), mk02-backend-api, mk04-security-pentest |
| **Full Stack** | mk12-system-architect, mk01-frontend-uiux, mk02-backend-api, mk03-infra-deploy |
| **Debug** | mk14-incident-responder, mk08-system-terminal, mk05-test-quality |
| **Security** | mk04-security-pentest, mk04-security-pentest, mk03-infra-deploy |

---

## Pre-Flight: Mode Check

| Current Mode | Task Type | Action |
|--------------|-----------|--------|
| **plan** | Any |  Proceed with planning-first approach |
| **edit** | Simple execution |  Proceed directly |
| **edit** | Complex/multi-file | ️ Ask: "This task requires planning. Switch to plan mode?" |
| **ask** | Any | ️ Ask: "Ready to orchestrate. Switch to edit or plan mode?" |

---

## 🔴 STRICT 2-PHASE ORCHESTRATION

### PHASE 1: PLANNING (Sequential - NO parallel agents)

| Step | Agent | Action |
|------|-------|--------|
| 1 | `mk12-system-architect` | Create implementation_plan.md |
| 2 | (optional) `mk08-system-terminal` | Codebase discovery if needed |

> 🔴 **NO OTHER AGENTS during planning!** Only mk12-system-architect and mk08-system-terminal.

### ️ CHECKPOINT: User Approval

```
After PLAN.md is complete, ASK:

" Kế hoạch đã được tạo: implementation_plan.md

Bạn có phê duyệt không? (Y/N)
- Y: Bắt đầu triển khai
- N: Tôi sẽ sửa lại kế hoạch"
```

> 🔴 **DO NOT proceed to Phase 2 without explicit user approval!**

### PHASE 2: IMPLEMENTATION (Parallel agents after approval)

| Parallel Group | Agents |
|----------------|--------|
| Foundation | `mk02-backend-api (database)`, `mk04-security-pentest` |
| Core | `mk02-backend-api`, `mk01-frontend-uiux` |
| Polish | `mk05-test-quality`, `mk03-infra-deploy` |

>  After user approval, invoke multiple agents in PARALLEL.

## Available Agents (17 total)

| Agent | Domain | Use When |
|-------|--------|----------|
| `mk12-system-architect` | Planning | Task breakdown, PLAN.md |
| `mk08-system-terminal` | Discovery | Codebase mapping |
| `mk01-frontend-uiux` | UI/UX | React, Vue, CSS, HTML |
| `mk02-backend-api` | Server | API, Node.js, Python |
| `mk02-backend-api (database)` | Data | SQL, NoSQL, Schema |
| `mk04-security-pentest` | Security | Vulnerabilities, Auth |
| `mk04-security-pentest` | Security | Active testing |
| `mk05-test-quality` | Testing | Unit, E2E, Coverage |
| `mk03-infra-deploy` | Ops | CI/CD, Docker, Deploy |
| `mk09-mobile-app` | Mobile | React Native, Flutter |
| `mk06-ai-data (hoặc mk05)` | Speed | Lighthouse, Profiling |
| `mk15-seo-growth` | SEO | Meta, Schema, Rankings |
| `mk12-system-architect` | Docs | README, API docs |
| `mk14-incident-responder` | Debug | Error analysis |
| `mk17-game-dev` | Games | Unity, Godot |
| `mk13-multi-agent` | Meta | Coordination |

---

## Orchestration Protocol

### Step 0: 🔴 MEM0 LOCAL MEMORY (MANDATORY)
1. Invoke `search_facts` or `get_all_facts` of Mem0 to pull the latest system context and architecture.
2. If uncertain about specific domains, use `search_facts`.
3. You MUST NOT start orchestrating without Mem0 memory context.

### Step 1: Analyze Task Domains
Identify ALL domains this task touches:
```
□ Security     → mk04-security-pentest, mk04-security-pentest
□ Backend/API  → mk02-backend-api
□ Frontend/UI  → mk01-frontend-uiux
□ Database     → mk02-backend-api (database)
□ Testing      → mk05-test-quality
□ DevOps       → mk03-infra-deploy
□ Mobile       → mk09-mobile-app
□ Performance  → mk06-ai-data (hoặc mk05)
□ SEO          → mk15-seo-growth
□ Planning     → mk12-system-architect
```

### Step 2: Phase Detection

| If Plan Exists | Action |
|----------------|--------|
| NO `implementation_plan.md` | → Go to PHASE 1 (planning only) |
| YES `implementation_plan.md` + user approved | → Go to PHASE 2 (implementation) |

### Step 3: Execute Based on Phase

**PHASE 1 (Planning):**
```
Use the mk12-system-architect agent to create PLAN.md
→ STOP after plan is created
→ ASK user for approval
```

**PHASE 2 (Implementation - after approval):**
```
Invoke agents in PARALLEL:
Use the mk01-frontend-uiux agent to [task]
Use the mk02-backend-api agent to [task]
Use the mk05-test-quality agent to [task]
```

**🔴 CRITICAL: Context Passing & Tri-Part Linkage (MANDATORY)**

When invoking ANY subagent, you MUST include the following in the prompt:

1. **Original User Request:** Full text of what user asked
2. **Decisions Made:** All user answers to Socratic questions
3. **Previous Agent Work:** Summary of what previous agents did
4. **Current Plan State:** If plan files exist in workspace, include them
5. **TRI-PART ROUTING INSTRUCTION (Mandatory):** Explicitly tell the sub-agent to lookup `.agents/.shared/SHARED_INDEX.md` and `.agents/skills/SKILL_INDEX.md` to find their specific DNA and skills before coding. Provide them with their specific `MKXX` ID.

**Example with FULL context:**
```
Use the mk12-system-architect agent to create PLAN.md:

**CONTEXT:**
- User Request: "Öğrenciler için sosyal platform, mock data ile"
- Decisions: Tech=Vue 3, Layout=Grid Widget, Auth=Mock, Design=Genç Dinamik
- Previous Work: Orchestrator asked 6 questions, user chose all options
- Current Plan: playful-roaming-dream.md exists in workspace with initial structure

**TASK:** Create detailed PLAN.md based on ABOVE decisions. Do NOT infer from folder name.
**ROUTING:** You are MK12 (Project Planner). Before starting, MUST read `.agents/.shared/SHARED_INDEX.md` and `.agents/skills/SKILL_INDEX.md` to load your specific planning DNA and skills.
```

> ️ **VIOLATION:** Invoking subagent without full context = subagent will make wrong assumptions!

### Step 4: Verification (MANDATORY)
The LAST agent must run appropriate tests and quality checks:
- Execute `npm test`, `dotnet test`, or any applicable framework testing suite.
- Request an independent review from `mk05-test-quality` or `mk04-security-pentest` if making high-risk changes.
- Ensure 0 regressions before reporting back to the user.

### Step 5: Synthesize Results
Combine all agent outputs into unified report.

---

## Output Format

```markdown
##  Orchestration Report

### Task
[Original task summary]

### Mode
[Current Antigravity Agent mode: plan/edit/ask]

### Agents Invoked (MINIMUM 3)
| # | Agent | Focus Area | Status |
|---|-------|------------|--------|
| 1 | mk12-system-architect | Task breakdown |  |
| 2 | mk01-frontend-uiux | UI implementation |  |
| 3 | mk05-test-quality | Verification scripts |  |

### Verification Checklist
- [x] All Unit/E2E Tests passing
- [x] Regression checked

### Key Findings
1. **[Agent 1]**: Finding
2. **[Agent 2]**: Finding
3. **[Agent 3]**: Finding

### Deliverables
- [ ] PLAN.md created
- [ ] Code implemented
- [ ] Tests passing
- [ ] Scripts verified

### Summary
[One paragraph synthesis of all agent work]
```

---

## 🔴 EXIT GATE

Before completing orchestration, verify:

1.  **Agent Count:** `invoked_agents >= 3`
2.  **Tests Executed:** At least one framework testing suite or QA review ran
3.  **Report Generated:** Orchestration Report with all agents listed
4.  **Mem0 Updated:** Invoke `add_fact` of Mem0 to log what was done.

> **If any check fails → DO NOT mark orchestration complete. Invoke more agents or run tests.**

---

**Begin orchestration now. Select 3+ agents, execute sequentially, run verification tests, synthesize results.**
