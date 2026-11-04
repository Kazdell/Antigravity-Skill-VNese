---
name: workflow-simplified
description: >-
  Quy trình làm việc 3 bước tinh gọn (Yêu cầu -> Đề xuất kèm Cảnh báo thay đổi ->
  Thực thi & Kiểm chứng), tích hợp sẵn Change Policy (change-policy), mẫu đề
  xuất phản hồi (response-template) và mẫu theo dõi công việc (task-template).
---

# /workflow-simplified — Quy trình 3 Bước & Chính sách Thay đổi (REA + Reconstruction)

Quy trình phối hợp nhanh, minh bạch giữa Người dùng và Agent:

```text
Bước 1: Yêu cầu ➔ Bước 2: Đề xuất (kèm Cảnh báo thay đổi) ➔ Bước 3: Thực thi & Kiểm chứng
```

## 1. Phân loại Mức độ Phê duyệt (Change Policy)

| Mức độ | Phạm vi áp dụng | Hành động của Agent |
| :--- | :--- | :--- |
| **Mức 1: Thực thi ngay** | Phân tích tĩnh (`static-binary-triage`), đọc capture offline (`.har`/`.saz`/`.pcap`/`.sqlite`), quan sát trình duyệt thụ động, sửa lỗi nhỏ trong code tái tạo, clean code cục bộ. | Thực hiện ngay bằng công cụ REA (chế độ `summary`) và `read_file` / `edit_file`, đóng phiên `close_binary` và báo cáo kết quả kèm `evidence_id`. |
| **Mức 2: Đề xuất & Chờ "OK"** | Tái tạo tính năng mới qua nhiều module, thay đổi schema DB / API contract của dự án đích, chạy kịch bản tương tác trình duyệt chủ động (`capture_browser_scenario`). | Trình bày **Proposal ngắn gọn** (mục tiêu, file dự kiến tạo/sửa, rủi ro) và chờ người dùng xác nhận `"OK"` rồi tự động thực thi từ A–Z. |
| **Mức 3: Chặn tuyệt đối cho tới khi được phép** | Khởi chạy binary lạ trên máy host, gắn hook động (`Frida`/`GDB`/`Rizin`), patch file binary gốc, xóa file, lệnh Git thay đổi lịch sử/nhánh (`git reset`, `git checkout`, `git clean`, `git push`). | Luôn hỏi xin phép tường minh trước khi thực hiện. |

---

## 2. Mẫu Đề xuất Ngắn gọn (Khi ở Mức 2)

```markdown
## Đề xuất thực hiện
- [Đầu mục phân tích / tái tạo 1 (kèm evidence_id nếu có)]
- [Đầu mục 2]

## Phạm vi & Cảnh báo thay đổi
- **Mục tiêu / Phân hệ đích**: `[tên-target / tên-service]`
- **File dự kiến sửa/tạo**: [Danh sách tên file kèm link tới vị trí trong workspace]
- **Ảnh hưởng API / DB / Runtime**: [Không / Có — mô tả ngắn]

## Kiểm chứng dự kiến
- Chạy kiểm tra biên dịch nhanh (và đối chiếu Golden I/O hoặc scoped test nếu được yêu cầu) -> PASS
```

*(Khi người dùng phản hồi **"OK" / "Được" / "Làm đi"**, thực thi toàn bộ + chạy kiểm chứng ngay mà không hỏi lại).*
