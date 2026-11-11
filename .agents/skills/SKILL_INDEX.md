# SKILL_INDEX.md — Master Skill Registry

> **Usage**: Query this file first (Layer 1) before loading any SKILL.md. Never scan the full `/skills/` directory.
> **Format**: `skill-folder-name` | Description | Trigger keywords

---

## TIER 1 — Orchestration & Workflow

| Skill Folder | Description | When to Use |
|---|---|---|
| `plan` | Planning & architecture design, feature breakdown | New feature, project planning, architecture |
| `create` | Code generation & scaffolding | Write new code, implement feature |
| `debug` | Debugging, root cause analysis, fix errors | Bug fix, error investigation |
| `test` | Test writing: unit, integration, E2E | Write tests, TDD, test coverage |
| `audit` | Code review & security audit | PR review, security scan, code quality |
| `code-quality` | Code quality standards and refactoring | Refactor, clean code, reduce tech debt |
| `orchestrate` | Unified 3-Tier & Native Swarm orchestration | Complex multi-domain tasks (`>5` files), parallel RE, Author/Verifier |
| `deploy` | Production & preview deployment with pre-flight & health checks | Deploy app, rollback, pre-deploy audit |
| `reframe` | Technical blind-spot scan & Sandwich prompt restructuring | Ambiguous specs, complex prompt framing |
| `log-error` | Systematic error logging to `ERRORS.md` & prevention rules | Recurring bugs, post-mortem logging |
| `monitor` | Production observability: structured logs, Sentry, `/health` | Setup monitoring, logging, health checks |
| `status` | Report task status, Swarm state, and progress | Status update, progress report |
| `handover` | Context handover between sessions/agents | End of session, agent switch |
| `workflow-simplified` | Simplified workflow reference | Quick workflow lookup |
| `customize-stack` | Customize toolchain & agent environment for specific stack | Setup environment, configure stack per language/framework |

---

## TIER 2 — Backend & API Patterns

| Skill Folder | Description | When to Use |
|---|---|---|
| `api-patterns` | REST/GraphQL/tRPC API design patterns | API design, endpoint planning |
| `api-design-principles` | Principled REST & GraphQL API design | API review, design standards |
| `backend-dev-guidelines` | Node.js + Express + TypeScript backend standards | Node.js backend development |
| `backend-development-feature-development` | End-to-end backend feature delivery | Feature development workflow |
| `dotnet-backend-patterns` | C#/.NET backend: EF Core, Dapper, xUnit, MVC | .NET/C# backend development |
| `nodejs-backend-patterns` | Node.js production patterns: Express/Fastify | Node.js REST/GraphQL API |
| `nodejs-best-practices` | Node.js best practices: event loop, async, security | Node.js code review, optimization |
| `logging-patterns` | Structured logging patterns | Add logging, observability |
| `docker-deploy` | Docker containerization & deployment | Dockerize app, CI/CD deploy |
| `design-patterns` | Software design patterns (GoF, SOLID, DDD) | Architecture decision, pattern selection |

---

## TIER 3 — Frontend & TypeScript

| Skill Folder | Description | When to Use |
|---|---|---|
| `typescript-expert` | TypeScript deep expertise: types, generics, monorepo | Complex TypeScript, type system |
| `typescript-advanced-types` | Conditional types, mapped types, template literals | Advanced TS type engineering |
| `javascript-mastery` | 33 core JS concepts: closures, event loop, async | JS fundamentals, explain concepts |

---

## TIER 4 — Systems & Infrastructure

| Skill Folder | Description | When to Use |
|---|---|---|
| `rust-async-patterns` | Rust async with Tokio, async traits, concurrency | Rust async code, Tokio |
| `memory-safety-patterns` | RAII, ownership, smart pointers across Rust/C++/C | Memory-safe systems programming |
| `full-stack-scaffold` | Project scaffolding: Node, Python, Rust, Mobile | New project setup |
| `document` | Documentation writing | Write docs, README, API docs |

---

## RULES INDEX

| Rule File | Trigger Condition |
|---|---|
| `agents.md` | Always on — core constitution & 3-Tier Swarm policy |
| `agent-routing.md` | Model decision — streamlined skill & Swarm routing |
| `GEMINI.md` | Always on — redirect to agents.md |
| `ai-agentic.md` | LLM, RAG, agents, MCP, tool integrations |
| `debug.md` | Bug fix, error analysis, test failures |
| `planning-architect.md` | Architecture design, project planning |
| `security.md` | Security-sensitive code, secrets, auth |
| `backend.md` | Backend development tasks |
| `frontend.md` | Frontend development tasks |
| `infrastructure.md` | DevOps, Docker, CI/CD, infra |
| `testing-mandatory.md` | Writing or reviewing tests |
