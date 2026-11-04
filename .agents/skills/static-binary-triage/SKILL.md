---
name: static-binary-triage
description: >-
  Khảo sát nhanh và dịch ngược tĩnh (Execution-Free Static Triage) cho các tập tin
  PE (.exe/.dll/.sys), ELF (.so/.ko/core), Mach-O, Android APK (.apk/.aab),
  Electron/JS (.asar), .NET/IL2CPP, Go/Rust/Python/Erlang/WASM/EVM, SQLite và
  Firmware mà không cần chạy chương trình mục tiêu.
---

# /static-binary-triage — Execution-Free Static Binary & Package Triage

Quy trình khảo sát tĩnh an toàn, không thực thi mã độc/binary mục tiêu, tuân thủ nguyên tắc **Summary-First** và **Evidence Provenance (`Observed` vs `Inferred` vs `Unknown`)**.

> **Instruction truy cập tài liệu chuyên sâu**:
> Toàn bộ 85 cẩm nang định dạng (`guides/*.md`), 7 cẩm nang điều phối (`references/*.md`) và bộ bridge scripts (`resources/bridge/`) nằm trong thư mục của skill `reverse-engineer-anything` (xác định từ danh sách `Available skills`). Dùng `read_file` mở trực tiếp file tương ứng khi cần tra cứu chi tiết.

## 1. Bảng Điều Hướng Công Cụ Theo Định Dạng Mục Tiêu

| Loại Tập Tin / Gói | Công cụ MCP / CLI đầu tiên (Target-Free / Session) | Tài liệu trong `reverse-engineer-anything/guides/` |
| :--- | :--- | :--- |
| **Windows PE (`.exe`, `.dll`, `.sys`)** | `inspect_pe_imports`, `inspect_pe_certificates`, `inspect_pe_load_config`, `inspect_pe_resources`, `inspect_native_exports` (`detail: "summary"`) | `pe-imports.md`, `pe-certificates.md`, `pe-load-config.md`, `binary-diagnostics.md` |
| **Linux ELF / Kernel Module / Core** | `inspect_binary_layout` (`detail: "summary"`), `inspect_elf_notes`, `inspect_kernel_module`, `inspect_recorded_crash` | `binary-diagnostics.md`, `elf-notes.md`, `kernel-modules.md`, `recorded-crashes.md` |
| **macOS / iOS (`.app`, `.ipa`, Mach-O)** | `open_bundle_resources` (`inspect_plist`, `inspect_asset_catalog`), `inspect_macho` (`detail: "summary"`), `inspect_apple_ips` | `apple-application-analysis.md`, `apple-ips.md` |
| **Android (`.apk`, `.aab`)** | `inspect_apk_signature`, `inspect_android_package` $\rightarrow$ `search_android_classes` $\rightarrow$ `inspect_android_method` | `android-analysis.md`, `apk-signature.md`, `apktool-resource-analysis.md`, `jeb-analysis.md` |
| **Electron / JS (`.asar` hoặc thư mục)** | `analyze_javascript_application` (`detail: "summary"`, chọn `source_paths` khi đã biết entry) | `javascript-artifact-reconstruction.md`, `electron-fuses.md` |
| **.NET Managed / NativeAOT / Unity IL2CPP** | `inspect_managed_artifact` (`detail: "summary"`), `inspect_il2cpp_metadata` (`global-metadata.dat`) | `managed-code-analysis.md`, `ghidra-nativeaot.md`, `il2cpp-metadata.md` |
| **Go / Rust / Python `.pyc` / Erlang `.beam`** | `inspect_go_binary`, `inspect_rust_binary`, `inspect_python_pyc`, `inspect_erlang_beam` | `go-binaries.md`, `rust-binaries.md`, `python-pyc.md`, `erlang-beam.md`, `python-packaged-applications.md` |
| **WASM / EVM Bytecode / AI Model** | `inspect_wasm_artifact`, `inspect_evm_interface`, `inspect_model_artifact` (GGUF/safetensors) | `wasm-artifacts.md`, `evm-bytecode.md`, `model-artifacts.md` |
| **SQLite / Firmware / Godot `.pck`** | `inspect_sqlite_database`, `inspect_firmware_regions`, `open_binary` + `inspect_artifact` | `sqlite-databases.md`, `firmware-analysis.md`, `godot-pck.md` |

## 2. Quy Trình 4 Bước Phân Tích Tĩnh Sâu (Deep Static Analysis)

1. **Bước 1 — Định danh & Kiểm tra Bề mặt (Surface Triage)**:
   - Chạy các công cụ target-free ở bảng trên với `detail: "summary"` để thu thập SHA-256 digest, danh sách import/export, chuỗi ký tự, chữ ký số và cấu trúc gói.
2. **Bước 2 — Phân trang có chọn lọc (`inspect_analysis_view`)**:
   - Dùng `parent_evidence_id` thu được để truy vấn đúng trang/module/symbol cần tìm thay vì tải toàn bộ báo cáo lớn vào ngữ cảnh.
3. **Bước 3 — Dịch ngược Hàm / Luồng dữ liệu (Decompilation & Cross-References)**:
   - Với native binary cần đọc giả mã C/Assembly: gọi `open_binary` (chọn provider `ghidra`, `ida`, `hopper`, hoặc `kuna`), dùng `search_strings` / `list_procedures` định vị hàm, sau đó gọi `analyze_function` và đọc các facet `pseudocode`, `callers`, `callees`, `value_flow_operations`.
4. **Bước 4 — Đóng phiên & Lập Báo cáo Bằng chứng (Evidence Ledger)**:
   - Luôn gọi `close_binary` khi kết thúc phiên native.
   - Ghi báo cáo chi tiết kèm `evidence_id`, địa chỉ VA/RVA/file offset và phân loại rõ `Observed` / `Inferred` / `Unknown`.
