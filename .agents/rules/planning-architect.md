---
trigger: model_decision
description: "When the user asks to design architecture, plan a project, write documentation, brainstorm features, or design APIs/schemas."
---

# PLANNING-ARCHITECT.MD — Simplicity-First Strategy & Goal-Driven Design

> **Mục tiêu**: Thiết kế kiến trúc tối giản, làm rõ giả định và chia nhỏ kế hoạch thành các bước nguyên tử có tiêu chí kiểm chứng (`[Step -> Verify]`) trước khi viết dòng code đầu tiên.

---

## 1. SIMPLICITY-FIRST ARCHITECTURE (KARPATHY PROTOCOL)

1. **Surface Tradeoffs & Challenge Complexity**: Trước khi chốt thiết kế, nêu rõ các giả định kỹ thuật. Nếu bài toán có thể giải quyết bằng cấu trúc sẵn có của dự án mà không cần thêm bảng/tầng/abstraction mới, ưu tiên phương án đơn giản nhất.
2. **Minimum Viable Architecture**: Không over-engineer từ ngày đầu. Không tạo trait/interface/abstraction cho thành phần chỉ có 1 triển khai duy nhất; không thêm configurability dự phòng khi chưa có yêu cầu thực tế.
3. **Lightweight ADR (Architecture Decision Records)**: Chỉ ghi chú ngắn gọn lý do chọn giải pháp A thay vì B khi có sự đánh đổi thực sự về hiệu năng, bảo mật hoặc tính toàn vẹn dữ liệu.

---

## 2. GOAL-DRIVEN PLANNING & DOCUMENTATION

1. **Atomic WBS with Verification (`[Step -> Verify]`)**: Chia nhỏ tác vụ thành các bước nguyên tử (`< 50` dòng code/bước). Mỗi bước bắt buộc định nghĩa rõ: `[Hành động cụ thể trên File/Tầng nào] -> [Tiêu chí kiểm chứng kết quả]`.
2. **Contract-First Clarity**: Thống nhất rõ đầu vào/đầu ra (DTO/Struct, Database Schema, API Contract) trước khi triển khai code chi tiết.
