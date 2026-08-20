# Master Outline & Atomic Task Breakdown Template

> **Usage**: Refer to this template when creating high-level project plans (`docs/PLAN-*.md`) or executing multi-part tasks.
> **Rule Link**: [agents.md (Directive #12)](file:///C:/Users/ACER/Downloads/Antigravity-Skill-VNese/.agents/rules/agents.md#L33)

---

## 1. Master Outline (Architectural Overview)

Define the top-level architectural modules and their dependencies using Tree of Thoughts (ToT) evaluation.

```
[System Architecture Master Outline]
├── Module 1: Core Engine & Data Models
├── Module 2: Business Logic / Domain Features
├── Module 3: UI / Interface Controls
└── Module 4: Testing & Verification
```

---

## 2. Module & Sub-task Decomposition (Atomic Breakdown)

Break down each module into **Atomic Tasks** (Single responsibility, < 50 lines of code change per task).

### Module 1: Core Engine & Data Models
- [ ] **Task 1.1 [Atomic]**: Define core data schemas and interfaces.
- [ ] **Task 1.2 [Atomic]**: Implement data loader/repository with error handling.

### Module 2: Business Logic / Features
- [ ] **Task 2.1 [Atomic]**: Implement Feature A core function.
- [ ] **Task 2.2 [Atomic]**: Implement Feature B core function.

### Module 3: UI & Controls
- [ ] **Task 3.1 [Atomic]**: Render primary layout frame.
- [ ] **Task 3.2 [Atomic]**: Bind Feature A to UI events.

---

## 3. Atomic Execution & Evaluation Gate Protocol

For **EVERY** Atomic Task in the chain:

1. **Execute**: Write/Modify code strictly scoped to that single atomic task.
2. **Verify**: Run build/test command or verify in sandbox.
3. **Evaluate (Human-in-the-Loop Gate)**: Confirm clean operation before advancing to the next atomic task link in the chain.
