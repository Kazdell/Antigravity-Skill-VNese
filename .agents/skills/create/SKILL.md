---
name: create
description: >-
  Xây dựng tính năng mới hoặc tái tạo module từ hồ sơ dịch ngược theo luồng phân
  tầng chuẩn (Data/Protocol -> Repository/Parser -> DTO/Validator ->
  Service/Algorithm -> Handler/Controller).
---

# /create — Clean-Room Feature & Module Creation

Hướng dẫn xây dựng tính năng mới hoặc hiện thực hóa hồ sơ dịch ngược (`Reconstruction Obligation Ledger`) vào dự án đích.

## Luồng Chuẩn 5 Bước + Kiểm chứng

1. **Data Model / Protocol Struct & Migration**:
   - Khai báo cấu trúc dữ liệu, bảng CSDL hoặc frame gói tin phản ánh chính xác các trường `Observed` từ bằng chứng dịch ngược.
2. **Repository / Parser Layer**:
   - Xây dựng tầng truy vấn dữ liệu (tránh N+1 query) hoặc bộ giải mã nhị phân/giao thức an toàn bộ nhớ (kiểm tra biên độ dài rõ ràng).
3. **Request / Response DTO & Boundary Validation**:
   - Tách biệt cấu trúc giao tiếp API với dữ liệu nội bộ; gắn đầy đủ validation tại biên.
4. **Domain Service / Reconstructed Algorithm**:
   - Viết lại thuật toán và luồng trạng thái bằng phong cách chuẩn mực (idiomatic) của ngôn ngữ đích (tuân thủ `backend.md` — cấm bê nguyên tên biến vô nghĩa của decompiler, cấm viết inline FQCN, cấm check dư thừa).
5. **API Handler / Controller / UI Component**:
   - Kết nối vào endpoint hoặc giao diện người dùng theo chuẩn của dự án đích.
6. **Xác minh Biên dịch & Kiểm thử Theo Yêu cầu (On-Demand Testing)**:
   - **Mặc định**: Chỉ Lead agent chạy lệnh kiểm tra biên dịch nhanh **1 lần duy nhất ở cuối** tại thư mục gốc của phân hệ đích (tuân thủ `testing-mandatory.md` — không tự ý viết/chạy unit test khi chưa được yêu cầu).
   - **Khi người dùng yêu cầu viết/chạy Test**: Viết Unit Test / Golden Input-Output Test đối chiếu với mẫu dữ liệu bắt được từ bản gốc và chỉ chạy scoped test đó.
