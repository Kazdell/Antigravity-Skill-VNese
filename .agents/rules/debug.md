---
trigger: model_decision
description: "When the user asks to fix bugs, analyze errors, investigate issues, run tests, or troubleshoot code."
---

# DEBUG.MD — Systematic QA & Fix Protocol

> **Mục tiêu**: Điều tra, Sửa lỗi và Kiểm chứng trong một luồng thống nhất, không đọc lan man.

---

## 1. INVESTIGATION (Sherlock Mode — Direct Call-Chain `<= 3` Files)

1. **Stack Trace & Direct Call-Chain Only (`<= 3` Files)**: Đừng đoán. Chỉ đọc đúng chuỗi gọi trực tiếp `Handler/Controller` $\rightarrow$ `Service/UseCase` $\rightarrow$ `Repository/Query` (tối đa `3` file). **CẤM TUYỆT ĐỐI** tự ý grep/đọc sang phân hệ khác, nhảy BE $\leftrightarrow$ FE, hoặc chạy lệnh OS (`Get-Process`, `Get-CimInstance`, `Get-NetTCPConnection`).
2. **Stop & Ask on Ambiguous Input**: Nếu sau khi đọc `<= 3` file trong Call-Chain mà chưa rõ người dùng đang truyền tham số nào, dừng gọi tool ngay lập tức, báo cáo logic tìm thấy và hỏi rõ tham số.
3. **Isolate**: Khoanh vùng đúng file và dòng chứa nguyên nhân gốc.

---

## 2. TESTING STRATEGY (The Guard)

1. **On-Demand Testing**: Tuân thủ quy tắc kiểm thử theo yêu cầu trong `testing-mandatory.md` — không tự ý viết/chạy unit test nếu không được yêu cầu; chỉ chạy lệnh kiểm tra biên dịch nhanh tại thư mục phân hệ Backend một lần ở cuối khi sửa BE.

---

## 3. FIXING PROTOCOL (Surgeon Mode & Goal-Driven Loop)

1. **Goal-Driven Verification Loop (`Isolate -> Root-Cause Fix -> Verify`)**: Chuyển yêu cầu sửa lỗi thành mục tiêu kiểm chứng rõ ràng: xác định đúng điều kiện gây lỗi trong `<= 3` file $\rightarrow$ sửa đúng nguyên nhân gốc (không vá triệu chứng hay chèn check phòng thủ mù quáng) $\rightarrow$ kiểm chứng bằng biên dịch hoặc chạy scoped test liên quan.
2. **Surgical Scope & Self-Cleanup**: Chỉ chạm vào những dòng gây lỗi. Xóa sạch log debug tạm thời và các import/biến dư thừa phát sinh từ bản vá của mình; tuyệt đối không refactor ké vùng code lân cận.

---

## 4. REPORTING

- Format: `[Lỗi] -> [Nguyên nhân gốc] -> [Giải pháp] -> [Phòng ngừa]`.
