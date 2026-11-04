---
name: test
description: >-
  Chạy bộ kiểm thử có chọn lọc (Scoped Test Runner) và kiểm thử tương đương
  Đầu vào - Đầu ra chuẩn (Golden Input/Output Equivalence Testing) dựa trên bằng
  chứng dịch ngược.
---

# /test — Scoped Automated Testing & Golden I/O Verification

Điều phối chạy kiểm thử có chọn lọc và đối chiếu tính tương đương giữa code tái tạo với bằng chứng thu được từ bản gốc.

## 1. Thực thi Kiểm thử Có Chọn Lọc (Scoped Execution)

Khi được yêu cầu chạy kiểm thử, di chuyển vào thư mục gốc của phân hệ đích và **chỉ chạy test mục tiêu**:
- **Rust**: `cargo test <test_name_filter> -- --nocapture`
- **.NET**: `dotnet test --no-build --filter "FullyQualifiedName~<TestClassOrMethod>"`
- **Go**: `go test -v -run "^Test<Name>$" ./path/to/pkg`
- **Python**: `pytest -k "<test_name>" -q`
- **Java (Gradle)**: `.\gradlew.bat test --tests "*<TestClassName>.<testMethodName>"`
- **Node.js / TypeScript**: `npx.cmd vitest run <path>` hoặc `npx.cmd jest <path> -t "<test_name>"`

> **Lưu ý**: Không tự ý chạy toàn bộ test suite của dự án trừ khi người dùng yêu cầu rõ.

---

## 2. Nguyên tắc Viết Golden Input/Output Tests Khi Tái Tạo Tính Năng

1. **Vector Kiểm thử Từ Bằng chứng Thực tế (`Observed Evidence Vectors`)**:
   - Trích xuất trực tiếp các cặp `(Input, Expected Output)` từ file capture (`.har`, `.saz`, `.pcap`), cơ sở dữ liệu `.sqlite`, hoặc kết quả trace của REA để làm dữ liệu test chuẩn (Golden Vectors).
   - Ghi chú rõ `evidence_id` nguồn gốc của vector kiểm thử trong comment của hàm test.
2. **Kiểm chứng Tương đương bằng Công cụ REA (Khi áp dụng)**:
   - Tận dụng `verify_reconstruction`, `compare_functions`, hoặc `compare_javascript_export_shapes` khi cần chứng minh tính tương đương về cấu trúc và hành vi.
3. **Không Bẻ Cong Code Tái Tạo Vì Test Giả Định Sai**:
   - Nếu một bài test tự suy diễn mâu thuẫn với dữ liệu `Observed` thực tế từ bản gốc, phải sửa bài test theo đúng bằng chứng `Observed`.
