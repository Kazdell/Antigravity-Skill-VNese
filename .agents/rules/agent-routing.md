---
trigger: always_on
---

# AGENT ROUTING PROTOCOL

> **Goal**: Enforce the mandatory Tri-Part Linkage routing workflow (Agents -> Shared DNA -> Skills) whenever processing user requests.

## 1. Mandatory Routing Workflow

Upon receiving any request (specifically code generation, architectural changes, or bug fixes), Jarvis **MUST** execute the following 5 routing steps BEFORE writing code:

1. **INTENT TRANSPARENCY & RISK ASSESSMENT:**
   - Explicitly declare analysis intent and locate the task within the **Goal -> Analyze -> Act -> Observe** loop.
   - Evaluate Risk Level (High/Medium/Low) and Reversibility. If the task is High-Risk / Irreversible (main source code edits, binary patching, file deletions), **HALT AND PROMPT FOR HUMAN APPROVAL GATE** before touching files.

2. **AGENT IDENTIFICATION:**
   - Analyze the domain of the task (Frontend, Backend, DevOps, DB, Game Dev...).
   - Load and invoke the responsible MK Specialist located under `.agents/agents/mkXX-*.md`.
   - Retrieve `MK_ID` (e.g., `mk01`, `mk02`, `mk17`).

3. **DNA STANDARDS CHECK:**
   - Agent **MUST** query `.agents/.shared/SHARED_INDEX.md` to locate relevant standards files instead of loading the entire `.shared` directory to **save tokens**.
   - Load only the DNA files strictly relevant to the task (e.g., read `api-standards` for BE; skip `design-system`).

4. **SKILL LOADING:**
   - Agent **MUST** query `.agents/skills/SKILL_INDEX.md` to locate the target Skill name.
   - Access `.agents/skills/{MK_ID}/{skill-name}/SKILL.md` to load instructions. **Do not scan the entire directory** to prevent context bloat.

5. **MEM0 MEMORY & AUDITABILITY:**
   - **Task Start**: Call `search_facts` or `get_all_facts` from Mem0 to retrieve project context. Never guess project architecture.
   - **Task Completion**: Output an Audit Trail report (distinguishing AI-generated vs User-provided vs preserved codebase code) and call Mem0 `add_fact` to persist lessons learned.

---

## 2. Execution Standards

- **No Guessing**: Never infer implementation logic if not specified in `.shared/` or `skills/`. Ask the User or query Mem0/codebase.
- **Selection Reporting**: Report to the User which MK Specialist, Shared DNA, and Skill from `SKILL_INDEX.md` are being invoked.

---

## 3. ADVANCED PROMPT CHAINING: CoT & ToT PROTOCOL

To maximize reasoning accuracy and prevent hasty architectural decisions, Agent MUST apply Chain of Thought (CoT) and Tree of Thoughts (ToT) prompting patterns:

### A. Chain of Thought (CoT) Protocol — Step-by-Step Reasoning
- **When to trigger**: Applied MANDATORY during the `Analyze` stage of G-A-A-O for all non-trivial bug fixes, code refactoring, and crash analysis (`/mod-debug`).
- **Execution**:
  1. *Observed Phenomenon*: State exact error log/symptom.
  2. *Hypothesis Matrix*: List at least 2 potential root causes.
  3. *Evidence-Based Elimination*: Verify against source code/logs to eliminate invalid hypotheses.
  4. *Final Selection*: State exact justification before mutating code.

### B. Tree of Thoughts (ToT) Protocol — Multi-Path Evaluation
- **When to trigger**: Applied MANDATORY during `/mod-plan` or architectural decision-making when facing complex targets, anti-cheat detection, or open-ended features.
- **Execution**:
  1. *Branching (Tree)*: Generate 2 to 3 distinct implementation paths (e.g., Branch A: Source Patch, Branch B: Internal Hook, Branch C: External RPM/Overlay).
  2. *Path Evaluation*: Score each branch against 3 metrics: Stealth/AC Risk, Technical Complexity, Maintenance Overhead.
  3. *Selection & Pruning*: Prune high-risk/suboptimal branches and proceed ONLY with the highest-scoring path.
