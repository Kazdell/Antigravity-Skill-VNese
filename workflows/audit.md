---
description: Sắp bàn giao khách? Kiểm tra lại toàn diện cho chắc.
---

# /audit - Comprehensive Quality Check

## 🔴 TRI-PART LINKAGE & MEMPALACE GATEWAY (MANDATORY)

**STEP 0: Context Initialization**
1. **Mempalace Check:** You MUST invoke mempalace_status or search tools to retrieve the latest architecture and state before taking any action. Do not guess the project structure.
2. **Tri-Part Linkage:** Before writing code or making system changes, you MUST read .agents/.shared/SHARED_INDEX.md and .agents/skills/SKILL_INDEX.md to load the exact DNA and skills required for this task.

**STEP END: State Persistence**
- When the task is completed, you MUST invoke mempalace_diary_write to log your actions and decisions into the permanent memory graph.

$ARGUMENTS

---

## Task
This command runs a full audit of the project to ensure enterprise quality.

### Steps:
1.  **Security Scan**: Check for vulnerabilities (`npm audit`, `pip check`).
2.  **Lint Check**: Run `eslint` or `pylint`.
3.  **Type Check**: Run `tsc` (TypeScript) or `mypy` (Python).
4.  **SEO Audit**: Check key pages for Meta tags (if web project).
5.  **Design Audit**: Validate the design system spec (`DESIGN.md`) and contrast ratios using the `design-linter` skill (`npx @google/design.md lint`).
6.  **AI Code Review**: Run the internal code review process using the `open-code-review` skill to inspect the recent code changes (`git diff`).
7.  **Report**: Generate `AUDIT_REPORT.md` combining the findings from all checks (including security vulnerability, lint, types, design audit, and AI code review annotations) and suggest fixes.

---

## Usage
```
/audit          # Run all checks (including AI Code Review and Design Audit)
/audit security # Only security scan
/audit seo      # Only SEO check
/audit design   # Only Design System linting
/audit review   # Only AI Code Review (git diff check)
```

