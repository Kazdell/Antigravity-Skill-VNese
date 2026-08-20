---
description: Reframe user prompts to eliminate technical blind spots, mitigate data bias, avoid over-guessing, enforce Responsible AI & YAGNI, and produce structured technical prompts for Software Development, Security, and Engineering tasks.
---

# WORKFLOW: /reframe

> **Goal**: Transform raw, ambiguous, or incomplete user requests into high-precision, context-enriched, and professional technical prompts for **Software Engineering**, **Security**, **DevOps**, and **Architecture**. Eliminate technical blind spots, prevent AI over-guessing, mitigate data bias, enforce version awareness, and apply the **Attention-Optimized Sandwich Architecture**, **High-Power Power Phrases**, and **Prompt Chaining**.

---

## 1. TRIGGER

This workflow is triggered when the User issues the command:
`/reframe [raw_request_or_technical_idea]`

*Examples:*
- `/reframe build auth service with JWT in Spring Boot`
- `/reframe fix N+1 query issue in EF Core`
- `/reframe design microservice communication for order + payment`

---

## 2. ATTENTION-OPTIMIZED PROMPTING ARCHITECTURE (SANDWICH)

To prevent LLM **Middle-Lost Syndrome**, every reframed prompt MUST follow the **Sandwich Prompt Architecture**:

```
 ┌────────────────────────────────────────────────────────────────────────┐
 │ TOP (Primacy Effect - 30%): CRITICAL GUARDRAILS & HIGH-POWER PHRASES  │
 │ • Persona & Expertise Scope (Senior Engineer / Architect / SecOps)    │
 │ • Recommended Temperature Tier (0.0-0.1 Deep Tech / 0.2 / 0.7-0.8)  │
 │ • High-Power Phrase: "Let's think step by step"                       │
 │ • Tech Stack, Exact Version, Architecture & Environment               │
 ├────────────────────────────────────────────────────────────────────────┤
 │ MIDDLE (Body Area): PROMPT CHAINING & FEW-SHOT ANCHORING              │
 │ • Chain Step 1: Analysis & Discovery                                  │
 │ • Chain Step 2: Core Implementation / Fix                             │
 │ • Chain Step 3: Integration, Testing & Validation                     │
 │ • Few-Shot Ground Truth: 1-2 verified code/pattern examples           │
 ├────────────────────────────────────────────────────────────────────────┤
 │ BOTTOM (Recency Effect - 30%): HARD CONSTRAINTS & CRITIQUE            │
 │ • "Focus STRICTLY on [X], DO NOT [Y]"                                │
 │ • "Act as a critical Code Reviewer"                                   │
 │ • YAGNI Minimalist Code Limits & Defensive Checks                    │
 │ • Output Format Specification                                         │
 └────────────────────────────────────────────────────────────────────────┘
```

---

## 3. HIGH-POWER PHRASES & PROMPT CHAINING

### A. High-Power Phrases
1. **Sequential Reasoning**: *"Let's think step by step. First, analyze [target]; Second, isolate [method]; Finally, implement [minimal code]."*
2. **Strict Negative Constraints**: *"Focus STRICTLY on [target module] and DO NOT introduce unrequested abstractions."*
3. **Critical Review**: *"Act as a critical Code Reviewer. What is the potential failure mode or security risk of this implementation?"*
4. **Alternative Paths**: *"Provide 2 alternative approaches and evaluate trade-offs (performance vs. complexity)."*

### B. Prompt Chaining Protocol
For complex tasks, structure a **3-Phase Prompt Chain**:
- **Chain Prompt 1 (Discovery)**: Analyze current state, identify root cause or target area.
- **Chain Prompt 2 (Implementation)**: Write minimal working code for the core mechanism.
- **Chain Prompt 3 (Integration)**: Wire up integrations, run tests, verify end-to-end.

### C. Long Thread Handoff Protocol
When a chat thread reaches >20% context usage:
1. Trigger the **`/handover` Workflow** to generate a complete Handover Document.
2. Use the generated **New Chat Bootstrapping Prompt** to open a **New Chat**.

---

## 4. RESPONSIBLE AI & DOMAIN DIRECTIVES

### A. Data Bias Mitigation
- **Avoid Over-Engineering Bias**: Enforce **YAGNI & Ponytail Minimalist Coding Protocol**.
- **Avoid Default-to-Complex**: Do NOT jump to microservices if a monolith solves the problem.

### B. Anti-Guessing Enforcement
- **Identify Missing Context**: Explicitly highlight missing parameters such as Tech Stack version, cloud provider, database type, or runtime environment.

### C. Version Boundary Management
- **Explicit Versioning**: Pin exact framework versions (e.g., Spring Boot 3.2 vs 2.7, Angular 17 vs 16, .NET 8 vs 6).

### D. Dynamic Temperature Policy
- **Ultra-Strict (0.0 - 0.1)**: Binary patching, DB schema mutations, security exploit PoC, critical offset calculation.
- **Default Baseline (0.2)**: Standard coding, refactoring, architecture, Q&A.
- **High Creative (0.7 - 0.8)**: UI/UX brainstorming, concept generation, multi-angle strategy.

---

## 5. EXECUTION WORKFLOW (5-STEP PROCESS)

### STEP 1: TECHNICAL INTENT & BLIND-SPOT SCAN
1. Parse core technical objective.
2. Scan for blind spots: Tech stack version, environment, architecture, constraints.

### STEP 2: CONTEXT RETRIEVAL
1. Search Mem0 via `search_facts` for existing project metadata.
2. Query relevant workspace files for context.

### STEP 3: ANTI-GUESSING CLARIFICATION QUESTIONS
Formulate **2 to 4 direct questions** addressing unresolved blind spots.

### STEP 4: SANDWICH PROMPT REFRAMING
Construct the **Reframed Technical Prompt** using Sandwich ordering + Temperature Tier + High-Power Phrases + Prompt Chaining.

### STEP 5: STANDARDIZED OUTPUT GENERATION

---

## 6. STANDARDIZED OUTPUT FORMAT

```markdown
### TECHNICAL & BLIND-SPOT ANALYSIS
- **Technical Intent**: [Core objective parsed]
- **Tech Stack & Version**: [e.g. Spring Boot 3.2 / Java 21 / PostgreSQL 16]
- **Recommended Temperature**: [0.0-0.1 | 0.2 | 0.7-0.8]
- **Detected Blind Spots**: [List missing parameters]
- **Bias Evaluation**: [Over-engineering / premature optimization risks]

### CLARIFYING QUESTIONS
1. [Question 1 regarding version / environment]
2. [Question 2 regarding scope / constraints]
3. [Question 3 regarding performance / security requirements]

### REFRAMED TECHNICAL PROMPT (Sandwich Architecture)
> **[TOP - GUARDRAILS]**
> You are a Senior Software Engineer / Architect.
> - Temperature: [Tier] ([Reasoning])
> - Let's think step by step.
> - Tech Stack: [Stack + Version], Environment: [Dev/Staging/Prod]
>
> **[MIDDLE - PROMPT CHAIN]**
> Phase 1: [Discovery objective]
> Phase 2: [Core implementation]
> Phase 3: [Integration & verification]
> Few-Shot Reference: [Code pattern example]
>
> **[BOTTOM - CONSTRAINTS]**
> - Focus STRICTLY on [target]. DO NOT over-engineer.
> - Act as Code Reviewer. Check for NPE, race conditions, resource leaks.
> - Output: Complete working code with inline comments.

### THREAD HANDOFF SUGGESTION
*If context >20%, run `/handover` to generate Handover Document and Bootstrapping Prompt.*
```
