---
trigger: always_on
---

# static-and-binary-analysis.md — Static Binary, Package & Artifact Rules

## 1. TARGET ROUTING & EXECUTION-FREE STATIC ANALYSIS
1. **Route by Artifact Format First**:
   - **Target-Free Static Inspectors** (do NOT call `open_binary` first):
     - JavaScript / Electron tree or `.asar`: `analyze_javascript_application`
     - Android APK code/classes/methods: `inspect_android_package`, `search_android_classes`, `inspect_android_class`, `inspect_android_method`, `trace_android_references`
     - APK/ZIP/AAB signing block: `inspect_apk_signature`
     - Managed .NET PE/CLI: `inspect_managed_artifact`, `inspect_managed_members`
     - Unity IL2CPP `global-metadata.dat`: `inspect_il2cpp_metadata`
     - PE static tables: `inspect_pe_imports`, `inspect_pe_certificates`, `inspect_pe_load_config`, `inspect_pe_resources`, `inspect_native_exports`
     - ELF static tables: `inspect_binary_layout`, `inspect_elf_notes`, `inspect_kernel_module`, `inspect_recorded_crash`, `inspect_native_exports`
     - Go / Rust / Python / Erlang / WASM / EVM / Model / SQLite: `inspect_go_binary`, `inspect_rust_binary`, `inspect_python_pyc`, `inspect_erlang_beam`, `inspect_wasm_artifact`, `inspect_evm_interface`, `inspect_model_artifact`, `inspect_sqlite_database`
     - Network & signal recordings: `inspect_web_network_capture` (`.har`/`.saz`/`qlog`), `inspect_pcap_capture`, `inspect_packet_capture`, `decode_signal_recording`
   - **Session-Bound Targets** (call `open_binary` first, close with `close_binary` when done):
     - Native executables/libraries (PE, ELF, Mach-O, DOS MZ/COM, raw firmware) for deep analysis via Ghidra, IDA, Hopper, or Kuna.
     - Archive/package member inventories (`.zip`, `.apk`, `.ipa`, `.msix`, `.dmg`, Godot `.pck`, PyInstaller executables) via `inspect_artifact` / `extract_artifact`.
2. **Preserve Identity & Coordinates**:
   - Always preserve artifact SHA-256 digests, virtual addresses (VA), relative virtual addresses (RVA), file offsets (`address_to_file_offset`), and section/segment boundaries.
   - Distinguish initialized static analysis memory (`read_bytes`) from raw file bytes and runtime memory.
3. **Read-Only Provider Discipline**:
   - Ghidra uses ephemeral databases without modifying target executable bytes; Windows Ghidra P0 is strictly read-only (`annotate_native_function` / `set_address_name` are not supported on Windows P0).
   - IDA attached GUI sessions are read-only—never save or close the user's attached GUI database.
