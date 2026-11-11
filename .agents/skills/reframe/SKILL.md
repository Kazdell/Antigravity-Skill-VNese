---
name: reframe
description: >-
  Quét điểm mù kỹ thuật (Tech stack, Version, Kiến trúc, Ràng buộc) và tái cấu
  trúc yêu cầu thành Sandwich Technical Prompt kèm phân tầng nhiệt độ tối ưu.
---

# /reframe — Technical Blind-Spot Scan & Sandwich Prompt Architecture

Chuẩn hóa các yêu cầu phức tạp hoặc còn mơ hồ thành đặc tả kỹ thuật chặt chẽ theo cấu trúc **Sandwich (Top Guardrails -> Middle Chain -> Bottom Constraints)**.

## Quy Trình 4 Bước

1. **Quét Điểm Mù Kỹ Thuật (Blind-Spot Scan)**:
   - Kiểm tra các thông số còn thiếu: Phiên bản ngôn ngữ/framework, kiến trúc đích, môi trường chạy, ràng buộc hiệu năng/bảo mật.
   - Phát hiện rủi ro over-engineering để giữ đúng tinh thần YAGNI.
2. **Khảo Sát Ngữ Cảnh Dự Án**:
   - Tra cứu nhanh cấu hình manifest (`package.json`, `Cargo.toml`, `*.csproj`, `pom.xml`, v.v.) trong workspace.
3. **Đặt Câu Hỏi Làm Rõ (2–4 Câu Ngắn Gọn)**:
   - Nêu trực diện các điểm mù chưa có trong codebase để User chốt.
4. **Xuất Sandwich Technical Prompt & Gợi Ý Tầng Điều Phối**:
   - Đề xuất mức độ chặt chẽ (`Ultra-Strict` cho DB migration/binary/security, `Baseline` cho code chuẩn, `Creative` cho UI/brainstorm) và tầng thực thi (`Direct` vs `Native Swarm`).
