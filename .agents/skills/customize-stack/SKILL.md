---
name: customize-stack
description: >-
  Chuyên biệt hóa bộ Reverse Engineering & Reconstruction Harness (.agents) cho
  ngôn ngữ / framework đích cụ thể (như Rust, .NET / C#, Go, Python, Node.js,
  Java / Spring Boot, C/C++, React/Angular...). Kích hoạt ngay khi người dùng
  yêu cầu "cấu hình phù hợp cho ngôn ngữ X" hoặc "init stack".
---

# /customize-stack — Target Stack Specialization for Reverse Engineering & Reconstruction

Khi người dùng yêu cầu: *"Hãy cấu hình bộ `.agents` này phù hợp cho ngôn ngữ Rust / .NET / Go / Python..."* (để tái tạo tính năng dịch ngược hoặc viết công cụ phân tích trên ngôn ngữ đó), hãy thực hiện quy trình 4 bước dưới đây để chuyên biệt hóa phần ngôn ngữ đích **mà vẫn giữ nguyên 100% kỷ luật REA Evidence Provenance (`Observed` vs `Inferred` vs `Unknown`), Zero-Waste và PreToolUse Hook**.

---

## Quy trình 4 Bước Chuyên Biệt Hóa

### Bước 1: Nhận diện Ngôn ngữ Mục tiêu (Target Stack)
1. Nếu workspace đã có dự án đích, kiểm tra nhanh file manifest (`Cargo.toml`, `*.sln`/`*.csproj`, `go.mod`, `pyproject.toml`, `package.json`, `build.gradle`).
2. Nếu người dùng chỉ nêu tên ngôn ngữ (ví dụ: *"cấu hình cho Rust"* hoặc *"cấu hình cho .NET"*), áp dụng ngay **Preset Chuẩn Thực Chiến** bên dưới.

### Bước 2: Chuyên biệt hóa Bộ Rules (`.agents/rules/`)
Dùng `edit_file` cập nhật:
1. **`rules/backend.md`**:
   - Thu hẹp `glob` ở frontmatter về đúng đuôi file của ngôn ngữ đích (`**/*.rs`, `**/*.cs`, `**/*.go`, `**/*.py`, `**/*.java`).
   - Điền chi tiết vào mục **4. STACK-SPECIFIC IDIOMS**: quy ước chuyển hóa giả mã decompiler sang code idiomatic của ngôn ngữ đích, quản lý lỗi, parser nhị phân/giao thức, và lệnh kiểm tra biên dịch nhanh.
2. **`rules/frontend.md`** (nếu tái tạo giao diện Web/Electron):
   - Thu hẹp `glob` và bổ sung quy ước riêng của framework UI đích.
3. **`rules/testing-mandatory.md`**:
   - Cập nhật lệnh kiểm tra biên dịch nhanh 1 lần ở cuối và cú pháp chạy scoped test / Golden I/O test khớp với công cụ build của ngôn ngữ đích.

### Bước 3: Chuyên biệt hóa các Core Skills (`.agents/skills/`)
Dùng `edit_file` cập nhật lệnh build/test và mẫu cấu trúc trong:
- `skills/create/SKILL.md` & `skills/feature-reconstruction/SKILL.md`: Điền cấu trúc tầng của ngôn ngữ đích để ánh xạ từ `Reconstruction Obligation Ledger` sang code thực tế.
- `skills/code-quality/SKILL.md` & `skills/audit/SKILL.md`: Điền bộ quy tắc Linter (`clippy`, `Roslyn/Sonar`, `golangci-lint`, `ruff`, `SonarQube`) và các anti-pattern khi dịch từ pseudocode sang ngôn ngữ đích.
- `skills/test/SKILL.md`: Điền mẫu Golden Input/Output Equivalence Test chuẩn của ngôn ngữ đích.

### Bước 4: Tạo thêm 1–2 Skill Đặc thù Ngôn ngữ (Tuỳ chọn)
Nếu ngôn ngữ đích có các thư viện chuyên dụng cho việc tái tạo giao thức/binary (ví dụ Rust `nom`/`bytes`/`serde`, .NET `Span<byte>`/`BinaryPrimitives`), có thể tạo thêm 1 skill mẫu tương ứng trong `.agents/skills/`. Tuyệt đối không tạo skill alias rỗng.

---

## Tham chiếu Nhanh: Presets Chuẩn Cho Ngôn Ngữ Đích

### 🦀 1. Preset cho **Rust** (`glob: "**/*.rs"`)
- **Fast Compile Check (Lead Agent, 1 lần ở cuối)**: `cargo check --workspace` (hoặc `cargo clippy --all-targets -- -D warnings` khi audit).
- **Scoped / Golden I/O Test Command**: `cargo test <test_name_filter> -- --nocapture`.
- **Idiomatic Reconstruction & Clean Code**:
  - **Binary & Protocol Parsing**: Dùng `bytes::Buf`, `nom`, `winnow`, hoặc `zerocopy`/`bytemuck` kèm kiểm tra độ dài rõ ràng thay vì ép kiểu con trỏ raw `unsafe` như trong giả mã C của Ghidra.
  - **CẤM `unwrap()` / `expect()` trong code tái tạo**: Bắt buộc dùng toán tử `?` với `thiserror` phản ánh đúng các nhánh lỗi quan sát được từ binary/capture gốc.
  - **Borrowing over Cloning**: Ưu tiên `&[u8]`, `&str`, `Cow<'_, [u8]>` khi xử lý gói tin/buffer.

### 🟣 2. Preset cho **.NET / C#** (`glob: "**/*.cs"`)
- **Fast Compile Check (Lead Agent, 1 lần ở cuối)**: `dotnet build --no-restore -v q`.
- **Scoped / Golden I/O Test Command**: `dotnet test --no-build --filter "FullyQualifiedName~<TestClassOrMethod>"`.
- **Idiomatic Reconstruction & Clean Code**:
  - **High-Performance Binary/Buffer Parsing**: Dùng `ReadOnlySpan<byte>`, `BinaryPrimitives` và `System.Text.Json` source generators thay vì mảng byte trung gian tốn GC.
  - **EF Core & Async Discipline**: Dùng `.AsNoTracking()` cho truy vấn đọc, truyền `CancellationToken` xuyên suốt, dùng Primary Constructor và `record` bất biến cho DTO/Message.

### 🐹 3. Preset cho **Go (Golang)** (`glob: "**/*.go"`)
- **Fast Compile Check (Lead Agent, 1 lần ở cuối)**: `go build ./...` (và `go vet ./...`).
- **Scoped Test Command**: `go test -v -run "^Test<Name>$" ./path/to/pkg`.
- **Idiomatic Reconstruction & Clean Code**:
  - Dùng `encoding/binary`, `bytes.Reader`, bọc lỗi tường minh với `fmt.Errorf("...: %w", err)` và truyền `context.Context` ở tham số đầu tiên.

### 🐍 4. Preset cho **Python** (`glob: "**/*.py"`)
- **Fast Compile / Lint Check (Lead Agent, 1 lần ở cuối)**: `ruff check .`.
- **Scoped Test Command**: `pytest -k "<test_name>" -q`.
- **Idiomatic Reconstruction & Clean Code**:
  - Dùng `pydantic` v2 cho API/schema, `struct` / `construct` cho binary parsing, type hints 100% (`bytes`, `memoryview`, `Annotated`).

### ☕ 5. Preset cho **Java 21 / Spring Boot 4** (`glob: "**/*.java"`)
- **Fast Compile Check (Lead Agent, 1 lần ở cuối)**: `.\gradlew.bat compileJava`.
- **Scoped Test Command**: `.\gradlew.bat test --tests "*<TargetTestClass>"`.
- **Idiomatic Reconstruction & Clean Code**:
  - Cấm Inline FQCN; cấm check null thừa sau `@Valid`/`@PathVariable`; dùng `record`, Pattern Matching for `switch`, `ByteBuffer` (với `ByteOrder` tường minh) và `@RequiredArgsConstructor`.
