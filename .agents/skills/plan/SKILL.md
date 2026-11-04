---
name: plan
description: >-
  Thiết kế kiến trúc tái tạo tính năng và lập kế hoạch phân rã theo Hồ sơ Bằng
  chứng (Evidence & Obligation Ledger) thành các bước nguyên tử (< 50 dòng code).
---

# /plan — Evidence-Backed Architecture & Atomic Reconstruction Plan

Lập kế hoạch điều tra dịch ngược hoặc phân rã công việc tái tạo tính năng trước khi viết code.

## Nguyên tắc Cốt lõi
1. **No Code Mutation During Planning**: Chỉ khảo sát tĩnh (`detail: "summary"`) và đọc file (`read_file`), chưa sửa code khi kế hoạch chưa được duyệt.
2. **Evidence-to-Target Mapping**: Mỗi hạng mục tái tạo phải chỉ rõ dựa trên `evidence_id` / địa chỉ / route nào của bản gốc và ánh xạ vào tầng nào của dự án đích.
3. **Atomic Breakdown (`[Step -> Verify]`)**: Chia thành các bước độc lập (`< 50` dòng code/bước) kèm tiêu chí kiểm chứng biên dịch hoặc đối chiếu đầu ra.

## Cấu trúc Kế hoạch Chuẩn

```markdown
# Implementation Plan: [Tên tính năng / Mục tiêu tái tạo]

## 1. Hồ sơ Bằng chứng & Phạm vi Ảnh hưởng (Evidence & Blast Radius)
- **Mục tiêu gốc & Evidence ID**: `[tên file/capture gốc + các evidence_id cốt lõi]`
- **Phân hệ đích**: `[tên-service/module đích]`
- **Database / Schema / Giao thức**: [Các bảng, struct, hoặc định dạng gói tin cần tạo/sửa]
- **Điểm còn Unknown (nếu có)**: [Các giả định cần xác nhận thêm]

## 2. Phân rã Công việc Nguyên tử (Atomic Tasks)
- [ ] **Bước 1 (Data / Protocol Layer)**: Định nghĩa Model / Struct / Parser theo đúng cấu trúc `Observed`.
- [ ] **Bước 2 (Contract & Validation)**: Request/Response DTO + Boundary Validation.
- [ ] **Bước 3 (Core Algorithm / Service)**: Tái tạo thuật toán/luồng nghiệp vụ (idiomatic, Zero Defensive Redundancy).
- [ ] **Bước 4 (API / UI Integration)**: Kết nối vào Handler/Controller hoặc Frontend component.
- [ ] **Bước 5 (Verification)**: Chạy kiểm tra biên dịch nhanh 1 lần ở cuối (chỉ viết và chạy scoped Golden I/O test khi người dùng yêu cầu).
```
