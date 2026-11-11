---
name: monitor
description: >-
  Thiết lập và kiểm tra hệ thống Observability cho ứng dụng (Structured JSON
  Logging, Error Tracking như Sentry/Datadog, Health Check endpoint và Alerting).
---

# /monitor — DevOps Observability & Health Monitoring

Thiết lập hoặc kiểm tra hệ thống giám sát vận hành Production. *(Để giám sát chống kẹt cho đội hình Swarm, hệ thống dùng built-in `swarm_monitor`).*

## Các Hạng Mục Triển Khai
1. **Structured Logging**: Cấu hình JSON logging kèm `traceId`/`requestId`, ẩn thông tin nhạy cảm (PII/secrets).
2. **Error Tracking**: Cài đặt Sentry / OpenTelemetry / Datadog theo tech stack của dự án.
3. **Health Endpoints**: Bổ sung `/health` (liveness) và `/ready` (kiểm tra kết nối CSDL/Cache).
4. **Alerting**: Đề xuất ngưỡng cảnh báo cho lỗi 5xx, độ trễ p95/p99 và CPU/RAM.
