---
name: debug
description: >-
  Phân tích lỗi trong quá trình dịch ngược, sửa lỗi script hook/bridge
  (Frida/GDB/CDP) hoặc debug sai lệch hành vi trong code tái tạo (Zero
  Regression, giới hạn chuỗi gọi <= 3 file).
---

# /debug — Systematic Debugging for Hooks, Bridges & Reconstructed Code

Quy trình điều tra và khắc phục lỗi trong quá trình phân tích hoặc tái tạo tính năng.

## Quy trình 3 Bước

### Bước 1: Khoanh vùng & Đối chiếu Bằng chứng (Sherlock Mode `<= 3` Files)
- Đọc stack trace / log lỗi và chỉ kiểm tra chuỗi gọi trực tiếp liên quan (tối đa `3` file).
- Đối chiếu hành vi lỗi của code tái tạo với bằng chứng `Observed` từ bản gốc (`evidence_id`, payload mẫu, thứ tự byte endianness, hoặc trạng thái thanh ghi/bộ nhớ).
- Xác định nguyên nhân:
  - **Sai lệch Giao thức / Giải mã**: Sai endianness (Little vs. Big Endian), sai độ lệch (offset/padding), hoặc thiếu bước giải nén/giải mã.
  - **Sai lệch Logic / Trạng thái**: Điều kiện biên trong giả mã decompiler bị dịch ngược chiều (`!=` vs `==`, signed vs. unsigned).
  - **Lỗi Hook / Bridge / MCP**: Sai địa chỉ RVA/VA (do ASLR base address), sai chữ ký hàm (calling convention), hoặc phiên phân tích bị kẹt.

### Bước 2: Sửa Nguyên Nhân Gốc (Surgeon Mode)
- Dùng `edit_file` sửa đúng vị trí phát sinh sai lệch; tuyệt đối không chèn code nuốt lỗi im lặng để che giấu gói tin lỗi.
- Tự dọn dẹp các dòng log/hook tạm thời phát sinh trong lúc gỡ lỗi.

### Bước 3: Xác minh Tối thiểu (Zero Regression)
- Chạy lệnh kiểm tra biên dịch nhanh 1 lần ở cuối (hoặc chạy riêng scoped test đối chiếu Golden I/O nếu được yêu cầu).
- Báo cáo ngắn gọn: `[Triệu chứng lỗi] ➔ [Nguyên nhân gốc (kèm đối chiếu evidence_id)] ➔ [Giải pháp] ➔ [Kết quả Xác minh]`.
