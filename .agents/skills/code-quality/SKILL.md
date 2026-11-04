---
name: code-quality
description: >-
  Review và dọn dẹp chất lượng code tái tạo hoặc script phân tích (Clean Code,
  loại bỏ rác giả mã decompiler, DRY/KISS/YAGNI, Zero Defensive Redundancy,
  Linter rules).
---

# /code-quality — Clean Code & Decompiler-Artifact Elimination

Quy trình làm sạch code tái tạo và công cụ phân tích: **(1) Đọc file bằng `read_file` ➔ (2) Đối chiếu bộ quy tắc Clean Code & Anti-Patterns ➔ (3) Sửa trực tiếp bằng `edit_file` ➔ (4) Xác minh biên dịch nhanh một lần ở cuối**.

## 1. Bộ Quy tắc Làm Sạch Code Tái Tạo
- **Xóa sạch Dấu vết Trình Dịch Ngược (Zero Decompiler Artifacts)**:
  - Đổi toàn bộ tên biến/hàm kiểu `uVar1`, `local_28`, `sub_140001000`, `a`, `b`, `e`, `t` thành tên có ngữ nghĩa rõ ràng dựa trên hành vi đã xác minh.
  - Thay thế cấu trúc điều khiển rối rắm của decompiler (vòng lặp `while(true)` kèm `break` lồng nhau, chuỗi `if` đảo ngược) bằng Guard Clauses phẳng và các hàm combinator chuẩn của ngôn ngữ đích.
- **DRY & Tái sử dụng**:
  - Gom các bảng hằng số, magic bytes, opcodes, helper đọc/ghi byte và utility chung vào module dùng chung.
  - Không khởi tạo lặp lại các đối tượng nặng (cipher, regex, parser) bên trong vòng lặp.

## 2. 5 Anti-Patterns Cấm Tuyệt Đối (Zero Defensive Redundancy)
1. **CẤM Inline FQCN / Deep Module Paths**: Bắt buộc khai báo `import` / `use` / `using` ở đầu file.
2. **CẤM check dư thừa sau Boundary Validation**: Khi dữ liệu đã qua bước parse/validate ở biên thành kiểu an toàn, không viết lại các câu lệnh kiểm tra `null`/rỗng lặp lại ở tầng dưới.
3. **CẤM bịa đặt hành vi không có trong Bằng chứng (`No Speculative Features`)**: Không tự thêm các tham số cấu hình hoặc nhánh xử lý "dự phòng" không tồn tại trong yêu cầu hoặc bằng chứng `Observed`.
4. **CẤM nuốt lỗi hoặc dùng `unwrap()`/`panic!` bừa bãi**: Mọi lỗi giải mã/nghiệp vụ phải trả về lỗi miền chuẩn.
5. **CẤM bẻ cong Production Code vì Test sai**: Nếu bài test viết sai với dữ liệu `Observed` thực tế, phải sửa lại test vector cho khớp với bằng chứng.
