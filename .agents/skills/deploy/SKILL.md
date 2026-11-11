---
name: deploy
description: >-
  Quy trình triển khai ứng dụng lên Staging/Production (Pre-flight Checklist,
  Build, Deploy lên Vercel/Railway/Fly.io/Docker, Health Check và Rollback an toàn).
---

# /deploy — Production & Preview Deployment

Điều phối kiểm tra trước triển khai (Pre-flight), đóng gói build, đẩy lên hạ tầng đích và xác minh sức khỏe dịch vụ.

## Các Chế Độ Hỗ Trợ
- `/deploy check` — Chỉ chạy Pre-deployment Checklist, chưa đẩy lên server.
- `/deploy preview` — Triển khai bản thử nghiệm / staging.
- `/deploy production` — Triển khai bản chính thức (yêu cầu User xác nhận trước khi thực thi).
- `/deploy rollback` — Khôi phục nhanh về phiên bản ổn định liền trước.

## Quy Trình 4 Bước
1. **Pre-Deployment Checklist**: Typecheck / Lint / Build cục bộ `0 errors`; không lộ secret cứng trong code; biến môi trường khớp cấu hình đích.
2. **Build & Deploy**: Triển khai qua CLI chuẩn của nền tảng (`vercel --prod`, `railway up`, `fly deploy`, `docker compose up -d`).
3. **Health Check**: Xác minh endpoint `/health`, kết nối CSDL và log khởi động.
4. **Báo Cáo & Phương Án Rollback**: Xuất tóm tắt phiên bản, URL và lệnh rollback dự phòng.
