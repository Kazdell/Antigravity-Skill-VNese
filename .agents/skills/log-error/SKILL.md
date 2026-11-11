---
name: log-error
description: >-
  Ghi nhận lỗi có hệ thống vào ERRORS.md (phân loại mức độ nghiêm trọng, nguyên
  nhân gốc, giải pháp đã áp dụng và biện pháp phòng ngừa tái phát).
---

# /log-error — Systematic Error Logging & Prevention

Ghi chép lại các lỗi kỹ thuật đáng chú ý vào `ERRORS.md` của dự án để tích lũy bài học và ngăn tái phát.

## Quy Trình Thực Hiện
1. **Phân Loại**: Xác định loại lỗi (`Build` / `Runtime` / `Logic` / `Integration`) và mức độ (`Critical` / `High` / `Medium` / `Low`).
2. **Ghi Vào `ERRORS.md`**: Thêm mục mới gồm `[Timestamp]`, `Location`, `Root Cause`, `Fix Applied`, `Prevention` và `Status: Fixed`.
3. **Khắc Phục & Phòng Ngừa**: Sửa nguyên nhân gốc, xác minh hết lỗi; nếu lặp lại `>= 2` lần thì đề xuất thêm test hoặc guardrail trong `.agents/rules/`.
