---
name: reframe
description: >-
  Làm rõ yêu cầu dịch ngược hoặc tái tạo tính năng mơ hồ, phát hiện điểm mù
  (blind spots / Unknowns) về giao thức / định dạng binary / kiến trúc đích và
  chuyển thành đặc tả thực thi chính xác.
---

# /reframe — Requirement Clarification & Blind-Spot Check

Làm sắc nét các yêu cầu dịch ngược hoặc tái tạo tính năng trước khi lập kế hoạch (`/plan`) hoặc viết code:

1. **Quét Điểm mù & Bằng chứng Sẵn có (Blind-Spot & Evidence Scan)**:
   - Kiểm tra nhanh xem mục tiêu người dùng cung cấp thuộc loại nào (Native PE/ELF/Mach-O, APK, Electron ASAR, `.har`/`.pcap`, hay source code dự án đích).
   - Xác định rõ những gì đã `Observed` (đã có bằng chứng), những gì đang `Inferred` (giả thuyết), và những gì còn `Unknown` (chưa có capture, thiếu khóa giải mã, chưa rõ tham số đầu vào).
2. **Đặt Câu hỏi Ngắn gọn (nếu cần)**:
   - Chỉ hỏi 1–3 câu trực diện vào mục tiêu phân tích hoặc quyết định kiến trúc của dự án đích (hoặc dùng công cụ `ask_question`).
3. **Đóng gói Đặc tả Thực thi (Reframed Spec)**:
   - Mục tiêu phân tích/tái tạo + Danh sách artifact/file liên quan + Ràng buộc Evidence Provenance & Zero Defensive Redundancy.
