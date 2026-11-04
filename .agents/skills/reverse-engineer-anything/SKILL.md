---
name: reverse-engineer-anything
description: Reverse engineer native, managed, Electron/JavaScript, packaged, firmware, Android, and browser targets with REA, and build agent environments (AgentEnv bundles) of web applications from a live instance or a capture. Use shipped-artifact or requested runtime evidence to explain features, compare versions, decompile code, or guide a reconstruction. Skip REA for ordinary source-repository architecture analysis.
metadata:
  version: "6.8.0"
---

# REA — Reverse Engineer Anything

Use REA when a claim depends on a shipped binary or package, decompilation, requested runtime evidence, or comparison with behavior not established by available source. For ordinary analysis of a complete source repository, use normal repository tools and do not run REA readiness or provider commands.

> **Instruction for Reference & Guide Resolution**:
> All supporting reference manuals (`references/*.md`), deep format/provider guides (`guides/*.md`), and bridge/hook resources (`resources/`) are bundled directly inside this skill's directory (the parent folder of this `SKILL.md` file as listed in `Available skills`). Whenever this document or a sub-guide names a reference file or guide, resolve its path from this skill's directory and read it using the native `read_file` tool. Never use hardcoded external URLs or assume a fixed drive letter.

## Connect only when needed

Installing this skill supplies instructions; it does not register the REA MCP server, install analysis engines, or add tools to an already-running agent.

If REA tools are available and their registration is not known to be stale, proceed directly to the target. Do not run diagnostics or setup before every investigation. Use the connected server's actual tool list and input schemas; keep complete inline Evidence when that server does not advertise retained references.

If tools are absent, a call is blocked by the client, or a required provider is unavailable, use `read_file` to read `connection-and-recovery.md` inside this skill's `references/` folder. Diagnose the observed blocker, continue useful supported work, and resume the original task after repair. Client approval, shell PATH, registration and engine availability have different remedies; avoid repeating setup for all of them.
If the host times out without a REA result, do not repeat the unchanged call or assume work stopped. Read the recovery guide's deadline distinction before retrying.
If the MCP connection drops, retry that call once only when it is safe to repeat or known not to have started. If it may already have changed the target, or it fails again, tell the user and ask them to restart the MCP client. If REA could not start its connection, retry once, then tell the user and ask them to restart the MCP client. Do not retry a startup failure that returns the same unchanged result. Run `rea doctor` when REA could not start or tools stay missing after that restart. For `cleanup_incomplete`, a retryable capacity limit, or `configuration_invalid`, follow the recovery guide's ladder before asking for a restart.

1. Diagnose without changing REA or client configuration. On Windows PowerShell, invoke `npx.cmd -y rea-agents@6.8.0 doctor --client antigravity --json` (or substitute the active client ID: `antigravity`, `claude_code`, `claude_desktop`, `codex`, `cursor`, `gemini_cli`, `windsurf`, `devin`, `opencode`, `copilot_cli`, `commandcode`, `vscode`, `grok_build`, `omp`, or `grok_bot`). If the client is unknown, use `doctor --json` and inspect its registration results before choosing a setup scope.
2. Distinguish the reason. Missing, malformed, or stale registration needs a scoped configuration repair. An aligned registration with no tools in the active session needs a restart/reconnection; doctor cannot prove that the current agent has connected. A missing provider affects only tasks requiring that provider: static JavaScript inspection needs neither Hopper nor Ghidra, and Android inspection has separate bring-your-own JADX/Java prerequisites.
3. For a configuration repair, prepare the read-only plan:
   `npx.cmd -y rea-agents@6.8.0 setup --client antigravity --skill=false --dry-run --json` (using `--skill=false` when you want to preserve this customized skill folder). Show its exact proposed paths, backups, and changes, and obtain approval before setup writes configuration or installs Hopper. After approval, apply the same scope with `--yes`. Add `--install-hopper` only if that separate installation was needed and explicitly approved.
4. Restart/reconnect the affected agent. Verify that REA tools actually appear in the session, then resume the original investigation. If they remain absent, inspect the client's MCP launch error rather than repeating setup.

REA setup never installs or upgrades Node.js, npm, Homebrew, Java, Ghidra, IDA, JADX, Kuna, Binwalk, or Unblob. Use existing prerequisites; do not install unrelated software to repair MCP registration. For an unsupported client, use manual stdio registration with a version-pinned `rea-agents` package or continue through the CLI.

A concrete CLI fallback for an operator-supplied JavaScript tree or ASAR (resolve `<target-app-path>` to the actual path provided by the user or workspace) is:

```powershell
npx.cmd -y rea-agents@6.8.0 analyze-javascript-application "<target-app-path>" --json
```

No MCP registration or native engine is required. Read the returned Evidence, graph, limitations, and unknowns with the same care as an MCP result; the CLI returns the Evidence record directly. Native CLI tasks still require their selected engine. Prefer an existing `rea` CLI; when none is installed and REA package acquisition is within the authorized scope, use `npx.cmd -y rea-agents@6.8.0` in its place.

## Route the target first

Choose the first tool from the target the user supplied. Use `open_binary` for active-target native or archive workflows; target-free tools take their own explicit path or endpoint and do not need it.

- **ASAR or extracted JavaScript/Electron tree**: `analyze_javascript_application` (see `javascript-applications.md` in `references/` and `javascript-artifact-reconstruction.md` in `guides/`).
- **Archive/package member inventory** (ZIP/APK/IPA/MSIX/AppX, DMG, Godot PCK, or a PyInstaller executable): `open_binary` with the supplied local path; use `inspect_artifact` when its graph and findings help answer the question. A PyInstaller CArchive is the application layer; the bootloader is only the native prefix. Start with `detail: "summary"` for large inventories, then use retained inventory views for selected member fields/pages. Consult `python-packaged-applications.md` in `guides/`.
- **Apple app plist, Interface Builder, asset catalog, or keyed archive resources**: `open_bundle_resources`, then the selected resource tool (`inspect_plist`, `decode_interface_builder`, `inspect_asset_catalog`, `inspect_keyed_archive`). Consult `apple-application-analysis.md` in `guides/`.
- **Android APK code, classes, methods, or incoming references**: `inspect_android_package`, then focused Android tools (`search_android_classes`, `inspect_android_class`, `inspect_android_method`, `trace_android_references`). Read `android-applications.md` in `references/` and `android-analysis.md` / `apktool-resource-analysis.md` / `adb-device-analysis.md` / `jeb-analysis.md` in `guides/`.
- **Managed PE/CLI assembly**: `inspect_managed_artifact` and `inspect_managed_members` (consult `managed-code-analysis.md` in `guides/`).
- **Firmware image**: `inspect_firmware_regions` when advertised. Use `extract_firmware` when extraction is requested, with a caller-selected new output directory. Consult `firmware-analysis.md` and `devicetree.md` in `guides/`.
- **.NET NativeAOT PE/ELF**: native Ghidra analysis. Consult the NativeAOT section in `native-and-artifacts.md` (`references/`) and `ghidra-nativeaot.md` (`guides/`).
- **Native program behavior, live calls, instrumentation, or command-run effects**: choose the matching workflow in `runtime-observation.md` (`references/`). `open_binary` and `trace_native_values` establish static facts; they do not observe execution.
- **User-owned browser page already open**: `list_browser_targets`.
- **Browser action the agent must perform (click, type, navigate)**: `capture_browser_scenario` with declared actions. Read `runtime-observation.md` in `references/` and `browser-observation.md` / `browser-scenario-contract.md` in `guides/`.
- **Agent environment (AgentEnv bundle) of a web application**: read `environments.md` in `references/` and `agent-environments.md` in `guides/` before recording; it records with `capture_browser_scenario` and writes the bundle with `export_environment_bundle`.
- **Fiddler Everywhere traffic or a native `.saz` capture**: inspect saved SAZ with `inspect_web_network_capture` and `format: "saz"`; connect Everywhere's separate MCP server for live traffic.
- **Retained HAR/SAZ/native mitmproxy/aioquic qlog capture**: `inspect_web_network_capture`. Prefer `detail: "summary"`, then kind `qlog` trace/stream pages or `api-surface` views. Consult `web-network-captures.md` and `http3-capture-lab.md` in `guides/`.
- **User-owned Electron runtime already open**: `list_electron_targets`. Before an electron-main Inspector attempt, check packaged fuses with `inspect_electron_fuses`; disabled `EnableNodeCliInspectArguments` means the app ignores `--inspect`. Consult `electron-observation.md` and `electron-fuses.md` in `guides/`.
- **Packet-field discovery & recorded packet captures**: `inspect_packet_fields`, `inspect_packet_capture`, and `inspect_pcap_capture` with MCP `detail: "summary"`, then retained kind `pcap` pages. Consult `packet-captures.md` and `pcap-analysis.md` in `guides/`.
- **Decoder discovery & recorded logic/signal traces**: `inspect_signal_decoders` and `decode_signal_recording`. Consult `signal-recordings.md` in `guides/`.
- **Explicit local WebAssembly artifact**: `inspect_wasm_artifact` (supply WABT through `REA_WABT_BIN_DIRECTORY`). Consult `wasm-artifacts.md` in `guides/`.
- **Explicit local Android APK (or ZIP/AAB) signing**: `inspect_apk_signature` and `inspect_analysis_view` kind `apk-signature`. Consult `apk-signature.md` in `guides/`.
- **Explicit local EVM bytecode carrier**: `inspect_evm_interface` with caller-selected `raw` or `hex` encoding. Consult `evm-bytecode.md` in `guides/`.
- **Embedded Go compiler, module or build-setting metadata**: `inspect_go_binary`. Consult `go-binaries.md` in `guides/`.
- **Embedded CUDA/HIP code objects**: `inspect_gpu_code_objects`. Consult `gpu-code-objects.md` in `guides/`.
- **Bundled GGUF or safetensors model declarations**: `inspect_model_artifact` and `inspect_analysis_view` kind `model-artifact`. Consult `model-artifacts.md` in `guides/`.
- **Unity IL2CPP names, member owners and string literals from `global-metadata.dat`**: `inspect_il2cpp_metadata` and `inspect_analysis_view`. Consult `il2cpp-metadata.md` in `guides/`.
- **Embedded cargo-auditable Rust dependency metadata**: `inspect_rust_binary`. Consult `rust-binaries.md` in `guides/`.
- **PE Authenticode certificate table, imports & load configuration**: `inspect_pe_certificates`, `inspect_pe_imports`, `inspect_pe_load_config`, `inspect_pe_resources`, and `inspect_native_exports`. Consult `pe-certificates.md`, `pe-imports.md`, `pe-load-config.md`, and `binary-diagnostics.md` in `guides/`.
- **ELF note tables, Linux kernel modules & recorded crashes**: `inspect_elf_notes`, `inspect_kernel_module`, `inspect_binary_layout`, and `inspect_recorded_crash`. Consult `elf-notes.md`, `kernel-modules.md`, `binary-diagnostics.md`, and `recorded-crashes.md` in `guides/`.
- **Apple `.ips` crash reports**: `inspect_apple_ips`. Consult `apple-ips.md` in `guides/`.
- **CPython 3.7–3.13 `.pyc` & Erlang BEAM `.beam` files**: `inspect_python_pyc` and `inspect_erlang_beam`. Consult `python-pyc.md` and `erlang-beam.md` in `guides/`.
- **Explicit local SQLite database snapshot**: `inspect_sqlite_database`. Consult `sqlite-databases.md` in `guides/`.
- **Flutter & HarmonyOS packages**: Consult `flutter-analysis.md` and `harmony-analysis.md` in `guides/`.
- **Native executable, library, or analysis database**: `open_binary`, then use focused analysis tools directly; call `binary_overview` when metadata or inventory context is useful and available from the selected provider.
  - For IDA (`provider_id: "ida"`), pass the original input binary (not `.idb`/`.i64`) and consult `ida-provider.md` in `guides/`.
  - For Kuna (`provider_id: "kuna"`), consult `kuna-provider.md` in `guides/`.
  - For Ghidra (including Windows Ghidra P0), consult `ghidra-provider.md` and `windows-ghidra-p0.md` in `guides/`.

If the user says only "my app" without a name, path, or target already selected for this task, ask which app before searching for artifacts. Never choose an example app on the user's behalf.

## Work summary-first

Start with the default result and use its inline Evidence and graph context. For a JavaScript application or ELF layout whose size is unknown or large, call `analyze_javascript_application` or `inspect_binary_layout` with `"detail": "summary"`, then inspect the retained `parent_evidence_id` with `inspect_analysis_view`. Over MCP, `analyze_function` is summary-first without a flag: its default reply is the summary, and the complete dossier stays retained under `parent_evidence_id` for facet paging. Pass that ID as a `retained-evidence` reference to `compare_functions`, or inside `match_native_functions` `left_observations`/`right_observations`; request `"detail": "complete"` only when the dossier must travel inline, as `build_call_path` requires.

If MCP reports `resource_constraint` with `details.resource: "transport"`, call `inspect_analysis_view` with its reported same-session Evidence reference for a summary, one section/module, or a stable page. Use `trace_application_feature` once a seed is known, or call `export_evidence_bundle` to write the complete session to a selected path in the conversation's `scratch/` directory or workspace.

Every conclusion must distinguish observations, inferences, and unknowns. Cite Evidence IDs, preserve limitations and incomplete coverage, and never imply that static analysis observed execution.

## Plan broader investigations

For requests that span multiple features or subsystems, use a staged workflow:

1. Turn the request into a checklist of questions and the evidence each answer needs. Resolve target identity and constraints from the conversation and workspace before asking for information again.
2. Inspect the current REA session, artifact identity, saved analysis database, bookmarks, and prior evidence. Reuse matching state; do not open duplicate sessions or repeat identical analysis.
3. Start with the smallest useful overview or inventory. Follow each question from its entry point through relevant data and state changes to its result. Batch related operations around a specific hypothesis, then expand only when the returned evidence leaves a concrete gap.
4. Inspect relevant packaged resources and configuration alongside code when they affect the question. Use format-aware inventory and parsers; do not infer behavior from filenames, strings, or layout alone.
5. Corroborate a conclusion with the evidence type it requires. Use runtime observation only when static evidence cannot answer the question and the required host runtime and OS access are available.
6. Decompose work into independent questions. When parallel workers are available and the questions do not depend on one another, assign distinct scopes, point workers to existing evidence, and ask them to return sources, conclusions, and unresolved gaps.
7. Keep a concise finding ledger linking each conclusion to Evidence IDs, confidence/evidence type, search boundary, and remaining unknowns.

## Read only the relevant reference or guide (via `read_file` inside this skill folder)

- **Core Reference Manuals** (in `references/` subfolder of this skill):
  - `native-and-artifacts.md` — Native binaries, managed assemblies, archives, PE/ELF/Mach-O, SQLite, WASM, IL2CPP, and extraction.
  - `javascript-applications.md` — ASARs, extracted JavaScript, module paging, feature tracing, and version comparison.
  - `android-applications.md` — Android APK declarations, classes, methods, JADX setup, and static references.
  - `runtime-observation.md` — Native instrumentation (Frida/GDB/LLDB/Rizin), browser/Electron CDP observation, Fiddler `.saz`/`.har`, and static/runtime comparison.
  - `evidence-workflows.md` — Evidence paging (`inspect_analysis_view`), comparisons, `api-surface` / `environment-manifest` / `environment-state` projections, and verification.
  - `environments.md` — Recording and exporting Agent environments (`AgentEnv` bundles) of web applications from live instances or HAR files.
  - `connection-and-recovery.md` — MCP/CLI connection troubleshooting, client/server deadline recovery, and `rea doctor` / `rea setup`.
- **Deep Format & Provider Guides** (85 specialized `.md` guides in `guides/` subfolder of this skill):
  - Read the matching file in `guides/` whenever a task requires deep format specifics (e.g., `windows-ghidra-p0.md`, `ghidra-nativeaot.md`, `ida-provider.md`, `kuna-provider.md`, `jeb-analysis.md`, `frida-instrumentation.md`, `web-network-captures.md`, `pcap-analysis.md`, `mcp-prompts.md`, `cli.md`, `README_vi.md`).
- **Bridges, Hooks & Examples** (in `resources/` subfolder of this skill):
  - `resources/bridge/` (Ghidra, JADX, NativeAOT, Hopper, Cutter, LLDB, Mitmproxy, Pwndbg, Pwntools scripts), `resources/hooks/` (Electron active hooks), `resources/third-party-guides/`, and `resources/examples/`.

## Finish the task

Explain findings in plain language and tie them to returned evidence. When the user asks to build something, use normal coding tools and separate observed behavior from design choices. Close an opened native session with `close_binary` when the investigation is complete.
