---
trigger: glob
glob: "**/*.{rs,cs,go,py,java,kt,ts,php,rb}"
---

# BACKEND.MD — Backend Clean Code, Idioms & Architecture Rules

> **Lưu ý cấu hình**: Khi chuyên biệt hóa cho dự án cụ thể (qua `/customize-stack`), thu hẹp `glob` ở trên về đúng đuôi file của ngôn ngữ Backend (ví dụ `**/*.rs` cho Rust, `**/*.cs` cho .NET, `**/*.go` cho Go, `**/*.java` cho Java) và cập nhật mục **4. STACK-SPECIFIC IDIOMS** bên dưới.

## 1. CLEAN CODE & FLAT CONTROL FLOW
1. **No Inline Fully-Qualified Names (FQCN / Deep Module Paths)**:
   - Cấm viết đường dẫn module/namespace/package đầy đủ lặp đi lặp lại trong thân hàm hoặc test; bắt buộc khai báo `import` / `use` / `using` tường minh ở đầu file.
2. **Guard Clauses & Flat Control Flow (No Pyramid of Doom)**:
   - Kiểm tra điều kiện biên, quyền truy cập và trạng thái hợp lệ ngay đầu hàm (`if invalid { return/throw }`), giữ luồng xử lý chính (Happy Path) phẳng ở mức thụt lề tối thiểu.
   - Cấm lồng ghép toán tử 3 ngôi (nested ternary) hoặc chuỗi `if-else` sâu; ưu tiên dùng combinators (`map`, `and_then`, `filter`, `coalesce`), pattern matching, hoặc tách thành helper method có tên rõ nghĩa.
3. **Immutability & Exhaustive Branching**:
   - Ưu tiên biến bất biến (`let`, `readonly`, `final`, `const`) và trả về collection rỗng thay vì `null`/`nil` khi danh sách không có phần tử.
   - Không tự ý xóa nhánh xử lý mặc định (`default` / `_`) trong `switch` / `match` nếu hệ thống dùng để bảo vệ tương thích ngược đối với dữ liệu/enum ngoài.

## 2. MANDATORY SHARED UTILITIES (DRY — REUSE OVER REINVENT)
1. **Core & Shared Library First**:
   - Luôn kiểm tra và tái sử dụng các module dùng chung của dự án cho: chuẩn hóa chuỗi, tra cứu Enum, xây dựng bộ lọc truy vấn động (query predicates/specifications), phân trang (`PagedData`), đóng gói response (`BaseResponse`), nén/xuất file, và lấy thông tin người dùng hiện tại (`CurrentUser` / `SecurityContext`).
   - Đóng gói logic biến đổi/validate lặp lại giữa nhiều Service/Handler vào module `*Helper` hoặc domain utility dùng chung.
2. **Constants & Timezone Discipline**:
   - Tập trung các mã lỗi, message key lặp lại vào module hằng số dùng chung; không hardcode chuỗi phân tán.
   - Mọi thao tác lấy thời gian hiện tại phải dùng UTC hoặc hằng số múi giờ chuẩn của hệ thống (tránh gọi hàm lấy giờ hệ thống ngầm định không rõ timezone). Không khởi tạo lặp lại các đối tượng nặng (formatter, regex compiler, HTTP client) bên trong vòng lặp.

## 3. BOUNDARY VALIDATION & STATIC ANALYSIS DISCIPLINE
1. **Zero Defensive Redundancy**:
   - Validate dữ liệu đầu vào ngay tại biên API (Request DTO / Schema Validator / Typed Extractor). Khi dữ liệu đã đi vào tầng Service/Domain với kiểu hợp lệ, không viết lại các câu lệnh kiểm tra `null`/rỗng dư thừa.
   - Không truy vấn DB lặp lại ở tầng điều phối (Handler/Action) những gì tầng Domain Service bên dưới đã lock và kiểm tra.
2. **Surgical Static Analysis / Linter Fixes**:
   - Khi sửa cảnh báo từ trình biên dịch hoặc linter (`clippy`, `Roslyn/Sonar`, `golangci-lint`, `ruff`, `SonarQube`), chỉ sửa đúng vùng lỗi bằng `edit_file` (giữ nguyên `CRLF`).
   - Khi hàm có quá nhiều tham số (>7), ưu tiên loại bỏ các tham số có thể suy diễn từ tham số khác trước, sau đó mới gom nhóm các trường cùng ngữ cảnh vào `struct` / `record` / `options object`.

## 4. STACK-SPECIFIC IDIOMS (CẬP NHẬT THEO NGÔN NGỮ DỰ ÁN)
- **Ngôn ngữ / Framework hiện tại**: `[Chưa chuyên biệt hóa — Chạy /customize-stack để điền chuẩn ngôn ngữ như Rust / .NET / Go / Python / Java]`
- **Lệnh kiểm tra biên dịch nhanh (Lead Agent)**: `[Ví dụ: cargo check | dotnet build --no-restore | go build ./... | ./gradlew compileJava]`
- **Quy ước đặc thù của ngôn ngữ**:
  - *Quản lý lỗi*: Dùng cơ chế lỗi chuẩn của dự án (Domain Error / `Result<T, AppError>` / Custom Exception được Global Handler bắt tự động); tuyệt đối không nuốt lỗi im lặng hoặc dùng `unwrap()`/`panic!` trong code nghiệp vụ.
  - *Truy cập dữ liệu & Transaction*: Tránh truy vấn N+1 trong vòng lặp; sử dụng batch query (`IN` / `JOIN`) và quản lý phạm vi transaction rõ ràng (chỉ đọc vs. ghi).
