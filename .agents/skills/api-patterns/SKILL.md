---
name: api-patterns
description: >-
  Thiết kế API RESTful / RPC chuẩn cho dự án — quy ước URI, HTTP status codes,
  Boundary Validation, phân trang PagedData, chuẩn hóa lỗi và tạo tài liệu
  cURL / Postman test.
---

# /api-patterns — REST & API Contract Standards

Tiêu chuẩn thiết kế API thống nhất trên toàn dự án.

## 1. Quy ước đặt URI & HTTP Methods

| Hành động | Method | URI Pattern | Response HTTP |
|---|---|---|---|
| Lấy danh sách phân trang | `GET` / `POST` | `/api/v1/{resources}` (hoặc `/search`) | 200 OK |
| Lấy chi tiết | `GET` | `/api/v1/{resources}/{id}` | 200 OK |
| Tạo mới | `POST` | `/api/v1/{resources}` | 201 Created |
| Cập nhật toàn phần | `PUT` | `/api/v1/{resources}/{id}` | 200 OK |
| Cập nhật một phần | `PATCH` | `/api/v1/{resources}/{id}` | 200 OK |
| Xóa (Soft delete) | `DELETE` | `/api/v1/{resources}/{id}` | 200 OK / 204 No Content |

- URI dùng danh từ số nhiều, lowercase, `kebab-case`.
- Khi đổi tên URI trên endpoint đã phát hành, giữ route alias tương thích ngược cho Frontend cũ.

## 2. Request & Boundary Validation

- Luôn thực hiện kiểm tra hợp lệ dữ liệu ngay tại biên API (Typed Extractor / Validator / Schema).
- Tuyệt đối không nhận trực tiếp Entity CSDL từ Client; bắt buộc đi qua Request DTO / Struct.

## 3. Response Structure & Error Mapping

- Trả về cấu trúc dữ liệu/phân trang chuẩn của dự án (`BaseResponse<T>`, `PagedData<T>`).
- Ánh xạ lỗi miền (Domain Error / Core Exception) nhất quán tại Global Error Handler / Middleware:
  - `400 Bad Request`: Dữ liệu đầu vào hoặc quy tắc nghiệp vụ không hợp lệ.
  - `401 Unauthorized` / `403 Forbidden`: Chưa xác thực hoặc không đủ quyền truy cập tài nguyên.
  - `404 Not Found`: Không tìm thấy bản ghi theo định danh.
  - `409 Conflict`: Trùng lặp mã định danh hoặc xung đột trạng thái (`optimistic lock` / `operation_status`).

## 4. Chuẩn Tài liệu API Test (cURL / Postman Artifact)
- Khi người dùng yêu cầu bộ API test (`cho api test`), luôn khởi tạo thành một **Artifact Markdown** hoàn chỉnh với JSON body chuẩn cho các kịch bản nghiệp vụ, kèm hướng dẫn truyền header xác thực (`Authorization: Bearer <TOKEN>` hoặc Gateway headers).
