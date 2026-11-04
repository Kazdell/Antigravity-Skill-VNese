---
name: docker-deploy
description: >-
  Đóng gói và triển khai (deploy) phân hệ bằng Docker / Docker Compose trên
  local — multi-stage Dockerfile, docker-compose (app + DB + Cache + Secret
  Store), healthcheck và giải thích từng lệnh.
---

# /docker-deploy — Containerization & Local Stack Deployment

Đóng gói phân hệ ứng dụng cùng các dịch vụ hạ tầng (Database, Redis, Vault, Message Broker) qua Docker Compose với cơ chế kiểm tra sức khỏe (`healthcheck`) tự động.

## 1. Nguyên tắc Multi-Stage Dockerfile Theo Ngôn Ngữ

- **Rust**: Stage 1 dùng `rust:<version>-alpine` hoặc `cargo-chef` để cache dependencies và build `--release`; Stage 2 dùng `debian:bookworm-slim` hoặc `distroless/cc` chỉ chứa file binary và chạy bằng non-root user.
- **.NET (C#)**: Stage 1 dùng `mcr.microsoft.com/dotnet/sdk:<version>` chạy `dotnet publish -c Release -o /app/publish`; Stage 2 dùng `mcr.microsoft.com/dotnet/aspnet:<version>-alpine` chạy non-root.
- **Go**: Stage 1 dùng `golang:<version>-alpine` build với `CGO_ENABLED=0 GOOS=linux`; Stage 2 dùng `alpine` hoặc `gcr.io/distroless/static-debian12:nonroot`.
- **Java (Spring Boot)**: Build fat-jar hoặc layered-jar và chạy trên `eclipse-temurin:21-jre-alpine` với non-root user.
- **Python / Node.js**: Tách bước cài đặt dependencies (`uv` / `pnpm`) vào layer riêng trước khi copy mã nguồn để tối ưu Docker layer cache.

## 2. Các Lệnh Vận Hành Docker Compose Thường Dùng

```bash
# Khởi tạo & build lại image ứng dụng chạy nền
docker compose up -d --build

# Kiểm tra trạng thái container & healthcheck
docker compose ps

# Theo dõi log thời gian thực của service 'app'
docker compose logs -f app

# Rebuild & khởi động lại riêng service 'app' (giữ nguyên DB/Redis/Vault đang chạy)
docker compose up -d --build app

# Dừng và dọn dẹp container + network
docker compose down
```

## 3. Lưu ý Quan Trọng
- Các container giao tiếp với nhau qua **tên service** trong `docker-compose.yml` (ví dụ `postgres`/`mariadb`, `redis`, `vault`), không dùng `localhost` cho kết nối liên container.
- Luôn khai báo `healthcheck` và `depends_on: condition: service_healthy` để ứng dụng chỉ khởi động sau khi Database và Secret Store đã sẵn sàng.
