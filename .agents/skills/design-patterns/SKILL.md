---
name: design-patterns
description: >-
  Mẫu thiết kế thực dụng (Builder, Factory, Strategy, Template Method,
  Decorator/Middleware, Provider) áp dụng trong dự án. Dùng khi thiết kế thành
  phần dễ mở rộng hoặc refactor code cứng nhắc.
---

# /design-patterns — Pragmatic Design Patterns

## 1. Bảng Chọn Pattern Thực Dụng

| Bài toán | Pattern | Cách áp dụng trong dự án |
|---|---|---|
| Tạo đối tượng nhiều tham số / cấu hình phức tạp | **Builder / Functional Options** | Dùng Builder macro/generator hoặc Options struct cho DTO, `PagedData`, cấu hình xuất báo cáo. |
| Tạo response hoặc lỗi chuẩn | **Factory** | Dùng hàm khởi tạo chuẩn của thư viện Core (`ResponseFactory`, `AppError::not_found(...)`), không tự tạo thủ công mỗi nơi một kiểu. |
| Khung xử lý cố định, bước con tùy biến | **Template Method / Trait Default** | Đặt luồng khung ở Base Service / Trait mặc định (như gửi sự kiện kèm prefix môi trường, xử lý retry), chỉ override phần đặc thù. |
| Hoán đổi thuật toán theo loại nghiệp vụ lúc chạy | **Strategy** | Định nghĩa interface/trait chung (ví dụ `ExportStrategy`, `PaymentProcessor`) và chọn theo điều kiện `supports(type)` để tuân thủ Open-Closed Principle (OCP). |
| Bổ sung hành vi cắt ngang (logging, lock, authz) | **Decorator / Middleware / Aspect** | Tách khỏi logic nghiệp vụ qua Middleware, Tower Layer, Action Filter hoặc AOP (`@DistributedLock`, `@VauthzCheck`). |

## 2. Nguyên tắc Cốt lõi (Karpathy Simplicity First)
- **Không Over-Engineer**: Chỉ dùng pattern khi có nhu cầu mở rộng thực tế ($\ge 2$ nhánh triển khai hoặc tái sử dụng xuyên phân hệ). Nếu logic chỉ dùng 1 nơi, viết hàm phẳng trực tiếp.
- **Tận dụng Pattern Sẵn Có của Core**: Luôn ưu tiên sử dụng các lớp cơ sở, middleware và helper đã có trong thư viện Core của dự án trước khi tự viết mới.
