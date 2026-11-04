---
trigger: model_decision
description: "When the user asks to reconstruct features from reverse-engineered evidence, design reconstruction architecture, debug hooks/bridges/rebuilt code, or run verification tests."
---

# reconstruction-and-debug.md — Feature Reconstruction, Architecture & Surgical Debugging

## 1. SIMPLICITY-FIRST FEATURE RECONSTRUCTION (KARPATHY + CLEAN CODE)
1. **Faithful & Minimal Reconstruction**:
   - Khi tái tạo lại tính năng, giao thức hoặc thuật toán từ bằng chứng dịch ngược (`Observed` + `Derived`), chỉ viết lượng code tối thiểu phản ánh đúng hành vi thực tế của bản gốc.
   - **No Speculative Abstractions**: Cấm tự bịa thêm interface, wrapper dùng 1 lần, hoặc cấu hình "dự phòng" không tồn tại trong thiết kế của mục tiêu gốc.
   - **Flat Control Flow & Zero Defensive Redundancy**: Ưu tiên Early Return (tối đa 2 cấp lồng nhau); chỉ kiểm tra các điều kiện biên/lỗi thực sự tồn tại trong luồng dữ liệu hoặc binary gốc.
2. **Goal-Driven Atomic WBS (`[Step -> Verify]`)**:
   - Chia quá trình tái tạo hoặc viết công cụ phân tích thành các bước nguyên tử (`< 50` dòng code/bước).
   - Mỗi bước bắt buộc gắn với tiêu chí kiểm chứng đối chiếu thực tế: `[Bước tái tạo / Viết Hook] -> [Cách kiểm chứng với bản capture / đầu ra của binary gốc]`.

## 2. SYSTEMATIC DEBUGGING FOR HOOKS, BRIDGES & REBUILT CODE (`<= 3` FILES)
1. **Direct Call-Chain Containment (`<= 3` Files)**:
   - Khi debug code tái tạo, script Frida/GDB/CDP hoặc bài test bị lỗi, chỉ đọc chuỗi gọi trực tiếp liên quan đến lỗi (tối đa `3` file).
   - Nếu sau `<= 3` file vẫn thiếu thông tin về đầu vào/môi trường chạy của mục tiêu, dừng gọi tool và hỏi rõ người dùng thay vì đoán mò.
2. **Root-Cause Surgeon & Self-Cleanup**:
   - Sửa đúng nguyên nhân gốc (sai lệch offset, sai calling convention, sai cấu trúc gói tin/state), không vá triệu chứng.
   - Sau khi sửa xong, xóa toàn bộ log/hook tạm thời và các import/biến dư thừa do quá trình sửa tạo ra; không sửa ké code lân cận.

## 3. ON-DEMAND & SCOPED VERIFICATION
1. **Scoped Verification Only**:
   - Không tự ý chạy toàn bộ test suite hoặc build toàn dự án khi không được yêu cầu.
   - Khi cần kiểm chứng code tái tạo, chỉ chạy biên dịch hoặc chạy đúng bài test/script đối chiếu đầu ra (Golden Input/Output comparison) của module vừa sửa.
2. **Reporting Format**:
   - Báo cáo ngắn gọn theo chuẩn: `[Vấn đề / Hành vi gốc (kèm evidence_id)] -> [Nguyên nhân / Cơ chế] -> [Giải pháp / Code tái tạo] -> [Kết quả kiểm chứng]`.
