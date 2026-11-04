---
name: logging-patterns
description: >-
  Chuẩn ghi log có cấu trúc (Structured Logging), truyền dẫn Correlation/Trace
  ID xuyên suốt và che giấu dữ liệu nhạy cảm (Sensitive Data Masking).
---

# /logging-patterns — Structured Logging & Traceability

## 1. Nguyên tắc Ghi Log Có Cấu Trúc
- Sử dụng thư viện structured logger chuẩn của ngôn ngữ (`tracing` trong Rust, `ILogger<T>` / `Serilog` trong .NET, `slog` / `zap` trong Go, `SLF4J` trong Java, `structlog` trong Python). Tuyệt đối không dùng `println!`, `Console.WriteLine`, `System.out.println` hay `fmt.Println` trong code nghiệp vụ.
- Dùng tham số hóa / structured key-value (không tự nối chuỗi thủ công):
  - Ghi rõ mốc nghiệp vụ (`INFO`), cảnh báo bất thường có thể phục hồi (`WARN`), lỗi cần can thiệp kèm nguyên nhân/stack trace (`ERROR`), và chi tiết gỡ lỗi (`DEBUG`).

## 2. Correlation ID / Trace ID Xuyên Suốt
- Tận dụng middleware của thư viện Core để tự động gán và truyền dẫn `correlation_id` / `trace_id` qua context (MDC / Tracing Span / Activity / `context.Context`) và đính kèm khi gọi sang dịch vụ ngoài.
- Không tự viết lại filter ghi log request/response nếu thư viện Core của dự án đã cung cấp sẵn.

## 3. Bảo mật & Mask Dữ liệu Nhạy cảm (BẮT BUỘC)
- **Tuyệt đối không ghi log plaintext** cho các trường nhạy cảm: `password`, `token`, `secret`, `apiKey`, `otp`, `pin`, số thẻ/CCCD.
- Khai báo bổ sung các trường nhạy cảm mới của nghiệp vụ vào danh sách `sensitive-fields` / custom serializer masker của hệ thống.
