# SKILL_INDEX.md — Master Skill Registry

> **Usage**: Query this file first (Layer 1) before loading any SKILL.md. Never scan the full `/skills/` directory.
> **Format**: `skill-folder-name` | Description | Trigger keywords

---

## TIER 1 — Reverse Engineering & Security (Primary Domain)

| Skill Folder | Description | When to Use |
|---|---|---|
| `reverse-engineer-anything` | Universal RE skill: Android APK, iOS, .NET, Electron, WASM, binary, firmware | ANY reverse engineering task, decompile, analyze binary |
| `dynamic-runtime-tracing` | Dynamic analysis: Frida, GDB, pwndbg, runtime hook, tracing, memory capture | Runtime tracing, hook functions, capture API calls |
| `static-binary-triage` | Static binary analysis: strings, checksec, entropy, file format triage | First-pass binary analysis, triage unknown binary |
| `feature-reconstruction` | Reconstruct undocumented features from decompiled code | Reconstruct logic, map decompiled code to features |
| `customize-stack` | Customize the RE toolchain and agent environment | Setup RE environment, install tools, configure stack |
| `Reverse_Engineering_Offsets` | Static/dynamic offset hunting: Ghidra, x64dbg, ReClass.NET, AOB scanning | Find memory offsets, pattern scan, SDK dump |
| `Memory_Access_Stealth_Bypass` | Out-of-process RPM/WPM, Direct Syscalls, Kernel Driver (Ring 0), DMA, Anti-Cheat bypass | Stealth memory read/write, bypass anti-cheat |
| `Game_Engine_Math_Overlay` | Unity/Unreal W2S math, View Matrix, GLFW transparent overlay | Game overlay, World-to-Screen, ESP drawing |
| `Java_Swing_Javassist_Modding` | Java bytecode patching via Javassist, JAR modding, Swing overlay menu | Java game modding, bytecode patch, JAR repack |

---

## TIER 2 — General Software Development (Enrichment)

| Skill Folder | Description | When to Use |
|---|---|---|
| `master` | Master orchestration skill: routes to correct specialist | Unsure which skill → start here |
| `plan` | Planning & architecture design, feature breakdown | New feature, project planning, architecture |
| `create` | Code generation & scaffolding | Write new code, implement feature |
| `debug` | Debugging, root cause analysis, fix errors | Bug fix, error investigation |
| `test` | Test writing: unit, integration, E2E | Write tests, TDD, test coverage |
| `audit` | Code review & security audit | PR review, security scan, code quality |
| `code-quality` | Code quality standards and refactoring | Refactor, clean code, reduce tech debt |
| `orchestrate` | Multi-agent task orchestration | Complex multi-step tasks, delegation |
| `status` | Report task status and progress | Status update, progress report |
| `handover` | Context handover between sessions/agents | End of session, agent switch |
| `reframe` | Reframe problem when stuck or blocked | Stuck, need new perspective |
| `workflow-simplified` | Simplified workflow reference | Quick workflow lookup |

---

## TIER 3 — Backend & API Patterns

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

## TIER 4 — Frontend & TypeScript

| Skill Folder | Description | When to Use |
|---|---|---|
| `typescript-expert` | TypeScript deep expertise: types, generics, monorepo, build, migration | **Mọi** TypeScript/JS issue — routing sang sub-expert nếu cần |
| `typescript-advanced-types` | Conditional types, mapped types, template literals | Type-level programming thuần túy |
| `javascript-mastery` | 33 core JS concepts: closures, prototype, event loop, async | JS fundamentals, explain/debug JS concepts |

---

## TIER 5 — Systems & Infrastructure

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
| `agents.md` | Always on — core constitution |
| `agent-routing.md` | Always on — routing protocol |
| `GEMINI.md` | Always on — entrypoint, list all rules |
| `re-constitution.md` | Reverse engineering tasks |
| `static-and-binary-analysis.md` | Static binary analysis, Ghidra, strings |
| `reconstruction-and-debug.md` | Reconstruct logic, debug decompiled code |
| `runtime-and-capture-safety.md` | Dynamic tracing, Frida, runtime hooks |
| `game-modding-external-policy.md` | Game modding, anti-cheat, memory offsets |
| `ai-agentic.md` | LLM, RAG, agents, MCP, tool integrations |
| `debug.md` | Bug fix, error analysis, test failures |
| `planning-architect.md` | Architecture design, project planning |
| `security.md` | Security-sensitive code, secrets, auth |
| `backend.md` | Backend development tasks |
| `frontend.md` | Frontend development tasks |
| `infrastructure.md` | DevOps, Docker, CI/CD, infra |
| `testing-mandatory.md` | Writing or reviewing tests |
