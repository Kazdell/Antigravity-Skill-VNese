---
trigger: always_on
---

# SECURITY.MD - Security Guardrails

> **Mục tiêu**: Bảo vệ hệ thống khỏi các lỗ hổng phổ biến và sai sót của con người.

---

##  1. FORBIDDEN ACTIONS (Cấm tuyệt đối)

1. **Hardcode Secrets**:
   - Không bao giờ viết API Key, Password, Token trực tiếp vào code.
   - Luôn sử dụng `process.env` hoặc biến môi trường.
2. **Commit Token**:
   - Kiểm tra file `.gitignore` trước khi commit.
   - Đảm bảo `.env` nằm trong `.gitignore`.
3. **Delete Database & Destructive Commands**:
   - CẤM TUYỆT ĐỐI KHÔNG BAO GIỜ tự ý chạy các lệnh phá hủy dữ liệu như: `dotnet ef database drop`, `DROP DATABASE`, `DROP TABLE`, hay xóa file `.sqlite` nếu không có lệnh RÕ RÀNG từ người dùng và BA bước xác nhận.
   - Đặc biệt cấm sử dụng cờ `--force` cho các lệnh xoá DB/xoá Migration trên production trừ khi có sự xác nhận bằng văn bản của Leader.

---

## ️ 2. CODING STANDARDS (Tiêu chuẩn Code An toàn)

1. **SQL Injection**:
   - Luôn sử dụng Parameterized Queries (hoặc ORM như Prisma/TypeORM).
   - Cấm nối chuỗi trực tiếp vào câu lệnh SQL.
2. **XSS (Cross-Site Scripting)**:
   - Sanitize mọi dữ liệu đầu vào từ người dùng hoặc API.
   - Sử dụng các thư viện như `dompurify` khi render HTML.
3. **Authentication**:
   - Luôn hash mật khẩu (Bcrypt/Argon2).
4. **Null Pointer Exceptions (NPE) & Safe Checking**:
   - Luôn kiểm tra tính hợp lệ (`null`/`undefined`) trước khi truy cập thuộc tính hoặc hàm của đối tượng.
   - Ưu tiên sử dụng optional chaining (`?.`) hoặc nullish coalescing (`??`) trong Javascript/Typescript, hoặc kiểm tra `if` tường minh.
5. **Thread-Safety & Concurrency**:
   - Tránh chia sẻ trạng thái có thể biến đổi (Shared Mutable State) giữa các luồng hoặc tiến trình bất đồng bộ mà không có cơ chế khoá (Lock) hoặc đồng bộ thích hợp.
   - Đảm bảo các hàm chạy song song không tranh chấp dữ liệu khi ghi vào Database hoặc Cache.
6. **Resource Cleanup & Leak Prevention**:
   - Luôn đóng hoặc giải phóng tài nguyên (file streams, database connections, event subscriptions) ngay sau khi dùng bằng cấu trúc an toàn như `try-finally` hoặc `using` (C#), clean-up function trong `useEffect` (React).

---

##  3. INCIDENT PROTOCOL (Quy trình sự cố)

Khi phát hiện lỗ hổng hoặc nghi ngờ lộ secret:
1. **DỪNG**: Ngừng mọi tác vụ hiện tại.
2. **BÁO CÁO**: Thông báo ngay cho người dùng bằng cảnh báo đ (RED ALERT).
3. **KHẮC PHỤC**: Đề xuất phương án xoay key (rotation) hoặc vá lỗi.

---


