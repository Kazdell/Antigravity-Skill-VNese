---
trigger: always_on
---

# runtime-and-capture-safety.md — Dynamic Observation, Instrumentation & Cleanup Rules

## 1. EXPLICIT AUTHORIZATION FOR LIVE EXECUTION & MUTATION
1. **Static Before Dynamic**:
   - Always prefer static inspection or offline capture analysis (`.har`, `.saz`, `.pcap`, `.sqlite`, core dumps) when it can answer the user's question.
   - Do NOT launch unknown native binaries, inject Frida scripts, run GDB/Rizin commands, or execute write scenarios against a live web app unless the user's task explicitly requests runtime observation and names the target.
2. **Browser, Electron & AgentEnv Safety**:
   - Passive observation (`list_browser_targets`, `inspect_web_page`, `observe_web_session`, `list_electron_targets`) never clicks, navigates, or mutates state.
   - Active browser scenarios (`capture_browser_scenario`) must set an explicit `timeout_ms` and `expected_status` on every step.
   - When recording an `AgentEnv` bundle that performs write operations (`POST`/`PUT`/`PATCH`/`DELETE`), verify that the user confirmed the target instance is disposable or seeded for testing.
   - Never ask for plaintext passwords in chat; always reference credentials via environment variables (`{source: "secret", secret_id: "..."}`).

## 2. MANDATORY LIFECYCLE & SESSION CLEANUP
1. **Always Close Opened Sessions**:
   - Close native binary sessions with `close_binary` once the investigation is complete (export any needed Evidence bundle via `export_evidence_bundle` before closing).
   - Close dynamic instrumentation sessions with their matching tool (`close_frida_session`, GDB/Rizin close) and verify cleanup completion.
   - If an operation reports `cleanup_incomplete`, retry the exact cleanup owner before starting new work that requires those resources.
2. **Windows Environment Conventions**:
   - When running CLI commands on Windows PowerShell, invoke `npx.cmd` (e.g., `npx.cmd -y rea-agents@6.8.0 ...`) and store temporary Evidence/view JSON files in the conversation's `scratch/` directory or `$env:TEMP`.
