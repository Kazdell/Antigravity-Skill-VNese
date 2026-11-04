# Native, managed, and packaged artifacts

> **Guide Resolution**: Every specialized format and provider guide mentioned below is stored offline inside the `guides/` subfolder of this skill (`pe-certificates.md`, `pe-imports.md`, `pe-load-config.md`, `elf-notes.md`, `kernel-modules.md`, `binary-diagnostics.md`, `go-binaries.md`, `rust-binaries.md`, `python-pyc.md`, `erlang-beam.md`, `sqlite-databases.md`, `wasm-artifacts.md`, `apk-signature.md`, `il2cpp-metadata.md`, `model-artifacts.md`, `godot-pck.md`, `ghidra-nativeaot.md`, `ghidra-dos.md`, `raw-binary.md`, `windows-ghidra-p0.md`, `native-function-matching.md`, `desktop-applications.md`, `apple-ips.md`, `recorded-crashes.md`). Use `read_file` to inspect the matching file in `guides/` whenever needed.

For release comparisons, export `list_procedures` Evidence from each target before closing it. `match_native_functions` pairs unique exact names and, when supplied, complete observed body bytes. Reuse optional same-context `list_names`, `procedure_info`, `read_bytes` and existing `analyze_function` dossiers as `left_observations`/`right_observations`; an MCP `analyze_function` summary reply contributes its `parent_evidence_id` as a `retained-evidence` reference, not the summary itself. Avoid decompiling every function merely to match it. Its summary retains a parent for `inspect_analysis_view` `function-matches` collections `matches` and `functions`.

## Apple app resources

Use `open_bundle_resources` with the selected app path, then `inspect_plist`, `decode_interface_builder`, `inspect_asset_catalog`, or `inspect_keyed_archive`. Use `open_binary` to switch to executable analysis, and `close_binary` to release either session kind.

## Desktop installation directories

When an installation directory is supplied and its runtime is unknown, use `inspect_desktop_application` with its path. Default MCP summary delivery retains complete Evidence. Read `files`, `omissions`, `candidates`, `components` or `next_probes` with `inspect_analysis_view` kind `desktop-application`, selecting exact paths, batched file/omission IDs or families and only needed fields and pages. (See `desktop-applications.md` in `guides/`).

## Go, Rust, Python `.pyc`, Erlang `.beam`, Model & IL2CPP metadata

- **Go build metadata**: Use `inspect_go_binary` for an explicit local ELF, PE or Mach-O. No `open_binary`, Go installation or target execution is required. (See `go-binaries.md` in `guides/`).
- **Model files**: Use `inspect_model_artifact` for local GGUF v2/v3 and safetensors declarations without a model runtime. Retained `model-artifact` views page `metadata`, `tensors` and derived `types`. (See `model-artifacts.md` in `guides/`).
- **IL2CPP metadata**: Use `inspect_il2cpp_metadata` for an explicit local Unity `global-metadata.dat` (versions 24, 27, 29, 31 and 39). Query `sections`, `images`, `types`, `methods`, `fields`, `properties`, `events`, `parameters` or `string_literals` through `inspect_analysis_view`. (See `il2cpp-metadata.md` in `guides/`).
- **Rust dependency metadata**: Use `inspect_rust_binary` for an explicit local ELF, PE, or Mach-O to inspect `.dep-v0` cargo-auditable crate lists. (See `rust-binaries.md` in `guides/`).
- **CPython `.pyc` files**: Use `inspect_python_pyc` for CPython 3.7–3.13 `.pyc` headers, PEP 552 hashes, nested code-object names, and bytecode SHA-256. (See `python-pyc.md` in `guides/`).
- **Erlang BEAM files**: Use `inspect_erlang_beam` for `FOR1`/`BEAM` files (`detail: "summary"` with `beam` view). (See `erlang-beam.md` in `guides/`).

## PE & ELF Target-Free Tables

- **PE Authenticode certificates**: `inspect_pe_certificates` (see `pe-certificates.md` in `guides/`).
- **PE imports & delay-load tables**: `inspect_pe_imports` (see `pe-imports.md` in `guides/`).
- **PE TLS callbacks & load configuration**: `inspect_pe_load_config` (see `pe-load-config.md` in `guides/`).
- **PE resource trees**: `inspect_pe_resources` with the target binary path (see `binary-diagnostics.md` in `guides/`).
- **PE & ELF export tables**: `inspect_native_exports` with `detail: "summary"`, then page via `inspect_analysis_view` with `view: {kind: "native-exports", collection: "exports", ...}` (see `binary-diagnostics.md` in `guides/`).
- **ELF note tables & layout**: `inspect_elf_notes` and `inspect_binary_layout` (see `elf-notes.md` and `binary-diagnostics.md` in `guides/`).
- **Linux kernel modules**: `inspect_kernel_module` for `.modinfo`, `__versions`, and PKCS#7 signature trailers (see `kernel-modules.md` in `guides/`).
- **Crash reports**: `inspect_apple_ips` for Apple `.ips` files (`apple-ips.md` in `guides/`) and `inspect_recorded_crash` for Linux x86-64 ELF core dumps (`recorded-crashes.md` in `guides/`).

## SQLite, WASM & APK Signatures

- **SQLite snapshots**: `inspect_sqlite_database` for compact schema inventory and selected table rows (requires Node.js 24.15+ / 26+). Retain both database and `-wal` digests. (See `sqlite-databases.md` in `guides/`).
- **Offline WASM**: `inspect_wasm_artifact` with `detail: "summary"` and `inspect_analysis_view` kind `wasm` (`exports`, `sections`, `imports`, `candidates`, `glue`, `WAT`). For comparing two WASM modules, use `compare_wasm_interfaces`. (See `wasm-artifacts.md` in `guides/`).
- **Offline APK signatures**: `inspect_apk_signature` for APK/ZIP/AAB v1/v2/v3/v3.1 signing blocks and `inspect_analysis_view` kind `apk-signature`. (See `apk-signature.md` in `guides/`).

## Native targets (Ghidra, IDA, Hopper, Kuna)

After `open_binary`, use focused search, procedure, or function tools directly. Use `binary_overview` when target metadata or inventory context helps answer the question. Prefer literal search, names, decompilation, callers, callees, and cross-references.

For a completed `analyze_function` dossier, use `inspect_analysis_view` with `view: {kind: "native", facet: "value_flow_operations", offset: 0, limit: 32}` and the exact retained Evidence ID to read selected facts without reanalysis. Native facets include `procedure`, `pseudocode`, `assembly`, `basic_blocks`, `comments`, `callers`, `callees`, `references`, `unresolved_calls`, `referenced_strings`, `referenced_names`, `native_apis`, and `value_flow_operations`.

### Ghidra startup deadline recovery

`provider_timeout` during the first Ghidra query can cover import, auto-analysis, bridge startup, and readiness. The default 330,000 ms is configurable through `REA_GHIDRA_STARTUP_TIMEOUT_MS` or at runtime via `update_analysis_settings` with `ghidra.startup_timeout_ms`. For per-function timeouts, update `ghidra.decompile_timeout_seconds`.

### Static memory and original file bytes

For Ghidra, `read_bytes` reads initialized analysis memory, including loader fixups; it does not read a running process. Use `address_to_file_offset` to anchor one observed address to original file bytes. Keep source ranges and analysis-memory observations separate.

### Function annotations in Ghidra

Use `annotate_native_function` for one function name and/or entry comments, or `set_address_name` / `set_addresses_names` for primary labels. Note: Windows Ghidra P0 is strictly read-only and does not admit database mutations (see `windows-ghidra-p0.md` in `guides/`).

## Managed PE/CLI & .NET NativeAOT

Start with `inspect_managed_artifact` (`detail: "summary"`) and `inspect_analysis_view` kind `managed` (`references`, `resources`, `attributes`). Use `inspect_managed_members` (`detail: "summary"` with `type_names`) and `inspect_analysis_view` kind `managed-members` (`types`, `exported_types`, `fields`, `methods`, `member_refs`, `call_edges`, `field_accesses`, `user_strings`).

For **.NET NativeAOT**, binaries contain native machine code. Open the native image with Ghidra/IDA or use REA's built-in x64 PE .NET 8 RTR 9.1 metadata parser (see `ghidra-nativeaot.md` in `guides/`).

## Packages, Extraction & Godot PCK

Use `inspect_artifact` (`detail: "summary"`) for archives, ZIP/APK/IPA/MSIX/AppX, ASAR, DMG, or Godot `.pck` inputs. Use `extract_artifact` when materialized regular files from a supported format are needed (materializes into a temporary directory managed by REA). For Godot PCK specifics, read `godot-pck.md` in `guides/`.
