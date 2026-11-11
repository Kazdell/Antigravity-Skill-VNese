---
trigger: model_decision
description: "When routing code generation, architecture, security, or multi-domain tasks to domain skills, shared DNA, or Native Swarm workers."
---

# AGENT & SKILL ROUTING PROTOCOL

> **Goal**: Enforce streamlined Tri-Part Linkage (Domain Role -> Shared DNA -> Skills) and 3-Tier Teamwork routing without dead-path lookups.

## 1. Streamlined Routing Workflow

For non-trivial coding, architecture, or reverse engineering tasks, execute BEFORE mutating code:

1. **INTENT TRANSPARENCY & RISK ASSESSMENT:**
   - Locate the task within **Goal -> Analyze -> Act -> Observe**.
   - If High-Risk / Irreversible (DB schema migration, public API break, binary patching, file deletion), **HALT AND PROMPT FOR HUMAN APPROVAL GATE** before touching files.

2. **TEAMWORK TIER & ROLE SELECTION:**
   - `<= 5` files (single domain): Execute directly (Tier 0).
   - `2–3` independent read-only lookups: Use `invoke_subagent` scouts (Tier 1).
   - `> 5` files, multi-domain, or Author <-> Verifier workflow: Load built-in `swarm` (`native_swarm`) or `/orchestrate` skill and assign job-named workers (`backend`, `frontend`, `re-native`, `verifier`, `monitor`).

3. **DNA & SKILL LOADING (ON-DEMAND ONLY):**
   - Query `.agents/.shared/SHARED_INDEX.md` (if present) and `.agents/skills/SKILL_INDEX.md` to locate only the exact standard/skill needed.
   - Read `.agents/skills/<skill-name>/SKILL.md` directly. **Never scan the entire `skills/` directory.**

4. **AUDITABILITY:**
   - Upon completing code/RE mutations, output a concise Audit Trail summary with verified evidence/compile status.

---

## 2. Execution Standards

- **No Guessing**: Never infer hidden business rules or protocol offsets; verify in codebase/binaries or ask the User.
- **CoT & ToT Reasoning**:
  - **CoT (Bug Fixes / `/debug`)**: Symptom -> 2+ Hypotheses -> Evidence Elimination -> Root-Cause Fix.
  - **ToT (Architecture / `/plan`)**: Evaluate 2–3 candidate approaches against Complexity, Risk, and Maintenance; prune suboptimal paths before coding.
