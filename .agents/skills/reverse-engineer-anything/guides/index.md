---
layout: home

hero:
  name: REA
  text: Reverse Engineer Anything
  tagline: Understand binaries, applications, and runtime behavior with one CLI and MCP server.
  actions:
    - theme: brand
      text: Get started
      link: /installation
    - theme: alt
      text: View on GitHub
      link: https://github.com/morluto/rea

features:
  - title: Investigate with your agent
    details: Connect REA to your coding agent, describe the app or feature you want to understand, and follow the evidence.
    link: /mcp-prompts
  - title: Inspect across formats
    details: Explore native binaries, JavaScript and Electron apps, .NET assemblies, Android packages, firmware, and websites.
    link: /installation
  - title: Keep the evidence
    details: Analysis runs locally and returns observations, limitations, and unknowns through the CLI and MCP.
    link: /mcp-contracts
---

## Start with one command

```bash
npx rea-agents setup
```

Choose your agent, review the planned changes, and approve setup. Native binary
analysis can use an existing Hopper, Ghidra, or IDA installation; static
JavaScript analysis needs no native analysis engine.

For a first result from an extracted JavaScript application or ASAR:

```bash
npx -y rea-agents@latest analyze-javascript-application <target-app-path> --json
```

See [installation and setup](./installation.md) for requirements and configuration.

For runtime instrumentation, see the [Frida guide](./frida-instrumentation.md).

## Choose a guide

| Task                                                | Guides                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Identify a desktop installation                     | Runtime candidates, evidence and follow-up files (`desktop-applications.md` in `guides/`)                                                                                                                                                                                                                                                                                                                                                                                                             |
| Inspect native functions, references, or types      | Native binaries (`native-investigation.md` in `guides/`), Go metadata (`go-binaries.md` in `guides/`), Rust dependencies (`rust-binaries.md` in `guides/`), CPython pyc files (`python-pyc.md` in `guides/`), PE certificates (`pe-certificates.md` in `guides/`), PE imports (`pe-imports.md` in `guides/`), PE TLS/load config (`pe-load-config.md` in `guides/`), ELF notes (`elf-notes.md` in `guides/`), Linux kernel modules (`kernel-modules.md` in `guides/`), Apple IPS (`apple-ips.md` in `guides/`), Erlang BEAM (`erlang-beam.md` in `guides/`), offline ELF layout (`binary-diagnostics.md` in `guides/`), recorded crashes (`recorded-crashes.md` in `guides/`) |
| Inspect native functions, references, or types      | Native binaries (`native-investigation.md` in `guides/`), Go metadata (`go-binaries.md` in `guides/`), Rust dependencies (`rust-binaries.md` in `guides/`), CPython pyc files (`python-pyc.md` in `guides/`), PE certificates (`pe-certificates.md` in `guides/`), PE imports (`pe-imports.md` in `guides/`), PE TLS/load config (`pe-load-config.md` in `guides/`), ELF notes (`elf-notes.md` in `guides/`), Apple IPS (`apple-ips.md` in `guides/`), Erlang BEAM (`erlang-beam.md` in `guides/`), offline ELF layout (`binary-diagnostics.md` in `guides/`), recorded crashes (`recorded-crashes.md` in `guides/`)                                            |
| Select a native provider or import format           | Ghidra semantics (`ghidra-provider.md` in `guides/`), Windows (`windows-ghidra-p0.md` in `guides/`), DOS (`ghidra-dos.md` in `guides/`), raw bytes (`raw-binary.md` in `guides/`), GameCube DOL (`ghidra-dol.md` in `guides/`), MIPS (`ghidra-mips.md` in `guides/`), PSP (`ghidra-psp.md` in `guides/`), NativeAOT (`ghidra-nativeaot.md` in `guides/`), IDA (`ida-provider.md` in `guides/`), Kuna (`kuna-provider.md` in `guides/`), JEB (`jeb-analysis.md` in `guides/`)                                                                                                                                                                      |
| Analyze JavaScript or Electron artifacts            | Static analysis (`javascript-artifact-reconstruction.md` in `guides/`), tracing and comparisons (`javascript-application-workflows.md` in `guides/`), application graph (`javascript-application-graph.md` in `guides/`), source recovery (`javascript-recovery.md` in `guides/`)                                                                                                                                                                                                                                                               |
| Compare static JavaScript with runtime observations | Runtime attribution (`javascript-runtime-observation.md` in `guides/`), reconciliation (`javascript-runtime-reconciliation.md` in `guides/`), Electron observation (`electron-observation.md` in `guides/`), Electron fuses (`electron-fuses.md` in `guides/`)                                                                                                                                                                                                                                                                                  |
| Inspect assemblies, packages, or firmware           | Godot PCK (`godot-pck.md` in `guides/`), .NET (`managed-code-analysis.md` in `guides/`), Python packages (`python-packaged-applications.md` in `guides/`), Android (`android-analysis.md` in `guides/`), APK signatures (`apk-signature.md` in `guides/`), Apktool resources (`apktool-resource-analysis.md` in `guides/`), ADB acquisition (`adb-device-analysis.md` in `guides/`), Apple (`apple-application-analysis.md` in `guides/`), Harmony (`harmony-analysis.md` in `guides/`), firmware (`firmware-analysis.md` in `guides/`), WASM (`wasm-artifacts.md` in `guides/`), EVM (`evm-bytecode.md` in `guides/`)                                          |
| Observe or compare a browser scenario               | CDP observation (`browser-observation.md` in `guides/`), scenarios (`browser-scenario-contract.md` in `guides/`), runtime evidence (`web-runtime.md` in `guides/`), historical captures (`web-network-captures.md` in `guides/`)                                                                                                                                                                                                                                                                                                                |
| Serve recorded behavior as an agent environment     | Agent environments (`agent-environments.md` in `guides/`)                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Trace captured website source                       | Source locations (`web-source-location.md` in `guides/`), module traces (`website-module-trace.md` in `guides/`), script export (`website-script-export.md` in `guides/`)                                                                                                                                                                                                                                                                                                                                                         |
| Capture and compare a command                       | Process capture (`process-capture.md` in `guides/`)                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Use CLI/MCP and retain evidence                     | CLI (`cli.md` in `guides/`), MCP contracts (`mcp-contracts.md` in `guides/`), agent prompts (`mcp-prompts.md` in `guides/`), obligation ledgers (`reconstruction-obligation-ledgers.md` in `guides/`)                                                                                                                                                                                                                                                                                                                                           |

For development, use testing (`testing.md` in `guides/`), client verification (`client-verification.md` in `guides/`),
tool design (`tool-design.md` in `guides/`), admission limits and recovery (`limits-and-recovery.md` in `guides/`),
and releasing (`releasing.md` in `guides/`).
Reconstruction readiness (`reconstruction-readiness.md` in `guides/`) describes a development
conformance verifier. [The 6.2 migration](migration-6.2.md) records that release's
compatibility changes; the roadmap (`roadmap.md` in `guides/`) tracks future work.

![REA inspecting a native binary in Hopper](./assets/rea-hopper-analysis.png)
