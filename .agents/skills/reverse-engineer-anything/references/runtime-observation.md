# Runtime observation

> **Guide Resolution**: For detailed contracts and prerequisites of dynamic workflows, use `read_file` to inspect `frida-instrumentation.md`, `reverse-engineering-tools.md` (GDB/Rizin/Cutter), `process-capture.md`, `native-investigation.md`, `browser-observation.md`, `browser-scenario-contract.md`, `electron-observation.md`, `javascript-runtime-observation.md`, `javascript-runtime-reconciliation.md`, and `web-network-captures.md` inside the `guides/` subfolder of this skill.

## Native runtime questions

Start from the requested observation. State the selected target, input, expected result, failure signal and stopping condition before a run. Reuse static Evidence to choose a function or hypothesis; `trace_native_values` follows recovered static value dependencies and does not measure runtime values. Prefer static inspection or existing capture comparison when it already answers the question.

| Question | Existing workflow | Host and effects |
| --- | --- | --- |
| Which selected native entry is reached, with which integer argument registers? | `observe_native_calls` on the active Mach-O, with exact function/Objective-C breakpoints and a duration/event bound | macOS with LLDB and required debugger access. Launches a new owned process, stops at entries, records and continues. |
| What does caller-authored instrumentation observe? | `list_frida_devices` / `list_frida_processes` when discovery is needed; `start_frida_session`, `load_frida_script`, then status/unload/close | Requires compatible installed Frida bindings and selected device access. Attach selects an existing PID; spawn selects a program and starts it paused until `resume_frida_session`. |
| Which debugger state or controlled execution explains the behavior? | `start_gdb_session` and `gdb_console`, or `start_rizin_debug_session` and `rizin_debug_command` | Persistent debugger workflows are Linux-only. Close an active Ghidra session explicitly first before running unrestricted GDB/Rizin commands. |
| How do terminal output, process lifetime and selected files differ between runs? | `capture_process_scenario`, then `compare_process_captures` with compatible recorded inputs | New captures require Linux or macOS; comparing retained captures does not execute targets and is available on Windows. |
| What does an existing desktop window show, or what changes after a declared UI action? | `observe_native_ui`; use `capture_native_ui_scenario` only for requested actions | macOS 14+ with required Screen Recording/Accessibility access. |

Close each persistent session with its matching tool (`close_frida_session`, GDB/Rizin close). To test a static hypothesis across two builds, retain compatible static comparison Evidence and `compare_process_captures` Evidence, then supply explicit finding/dimension mappings to `correlate_static_and_runtime`.

## Browser and Electron

Supply the existing browser or Electron instance's literal-port loopback CDP endpoint in the request. Browser discovery lists eligible HTTP(S) pages; `allowed_origins` optionally filters them. Electron discovery lists eligible local `file://` targets.

Start passive browser work with `list_browser_targets`; start Electron runtime work with `list_electron_targets`. Before an electron-main Inspector attempt, check packaged fuses with `inspect_electron_fuses`. Observation with these tools is passive: never claim REA clicked, navigated, evaluated page JavaScript, invoked IPC, or captured prior activity. Use `capture_web_screenshot` when pixels are needed.

## Who performs the action

- **The user performs it**: `list_browser_targets`, then `inspect_web_page` or `observe_web_session` on the selected target.
- **The agent must perform it**: one `capture_browser_scenario` with declared actions. Use connect mode with the loopback CDP endpoint and `target_id` for an existing browser, or launch mode with a caller-selected browser executable on the host for a temporary profile. Check `browser.cleanup` for the outcome. Request only the DOM, accessibility, or network details needed for the question.

Focused follow-up tools:
- `observe_web_execution`: execution coverage during a window.
- `inspect_web_event_listeners`: source locations for listeners registered directly on the first CSS match.
- `compare_web_captures`: two normalized passive captures or recorded scenarios.
- `discover_webmcp_tools`: inspect page-declared WebMCP inventory; never invoke the discovered tools.
- `reconcile_javascript_runtime`: use after static application Evidence and passive browser/Electron Evidence exist to correlate static modules with runtime scripts.

## Fiddler Everywhere & Network Traffic Debugging

For large saved captures (`.har`, `.saz`, `qlog`), request `detail: "summary"`, then page retained Evidence with `inspect_analysis_view` and `view: {kind: "capture", detail: "preview", offset: 0, limit: 20}`. Select complete record pages only for needed headers, metadata or payloads.

For selected protobuf/gRPC bodies, keep raw bytes as the source and use `inspect_analysis_view` kind `protobuf`, collection `bodies`, `frames` or `fields`.

Inspect native `.saz` snapshots with `inspect_web_network_capture` and `format: "saz"`. For live traffic analysis, connect Fiddler Everywhere's separate MCP server and use its advertised session tools (`LiveTraffic` or `AgentCalls`).
