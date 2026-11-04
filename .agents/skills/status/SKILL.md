---
name: status
description: >-
  Kiểm tra nhanh trạng thái không gian làm việc: nhánh Git của dự án tái tạo,
  các phiên phân tích REA đang mở (binary_session), sổ cái bằng chứng và tiến độ
  trong tài liệu handover.
---

# /status — Workspace, REA Session & Reconstruction Status

Tổng hợp nhanh bức tranh hiện tại của phiên dịch ngược và dự án tái tạo.

## Các Bước Kiểm tra

1. **Trạng thái Git & File trong Workspace**:
   - Kiểm tra nhánh hiện tại và các file đang thay đổi (`git status --short`, `git log -n 3 --oneline`).
2. **Tiến độ Dịch ngược & Handover**:
   - Dùng `read_file` kiểm tra tài liệu `handover-*.md` hoặc bảng `Evidence Ledger` gần nhất trong phiên.
3. **Báo cáo Tóm tắt**:
   - **Mục tiêu dịch ngược & Phân hệ đích đang mở**.
   - **Trạng thái phiên REA**: Còn phiên native/runtime nào đang mở cần lưu ý đóng hay không.
   - **Hạng mục vừa hoàn thành & Việc tiếp theo (Next Steps)**.
