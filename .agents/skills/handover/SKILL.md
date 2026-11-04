---
name: handover
description: >-
  Tạo tài liệu bàn giao phiên dịch ngược & tái tạo (Session Handover Document)
  kèm sổ cái bằng chứng (Evidence IDs, SHA-256 digests, VA/RVA coordinates),
  trạng thái code tái tạo và prompt khởi tạo phiên mới.
---

# /handover — Reverse Engineering & Reconstruction Session Handover

Đóng gói toàn bộ kết quả điều tra dịch ngược (`evidence_id`, cấu trúc gói tin, địa chỉ hàm) và tiến độ tái tạo tính năng của phiên hiện tại thành tài liệu bàn giao chuẩn (`handover-YYYY-MM-DD.md`).

## 1. Quy trình Thực hiện (3 Bước)

1. **Tổng hợp Hiện trạng & Đóng phiên**:
   - Xuất gói bằng chứng nếu cần (`export_evidence_bundle`) và đóng sạch các phiên đang mở (`close_binary`).
   - Kiểm tra trạng thái Git của dự án tái tạo (`git status --short`).
2. **Tạo File Handover**:
   - Dùng `write_file` tạo artifact `handover-YYYY-MM-DD.md` trong thư mục Artifact của phiên hội thoại.
3. **Xuất Prompt Khởi tạo Phiên Mới**:
   - Cung cấp đoạn prompt ngắn gọn kèm đường dẫn link tới file handover để người dùng copy sang phiên mới.

---

## 2. Mẫu Tài liệu Chuẩn (`handover-YYYY-MM-DD.md`)

```markdown
# handover-YYYY-MM-DD.md — [Tên Mục tiêu Dịch ngược / Dự án Tái tạo]

## 1. Target & Reconstruction Context
- **Target Artifacts**: `[đường dẫn file binary / APK / ASAR / capture + SHA-256 digest]`
- **Target Tech Stack (Reconstruction)**: `[Ngôn ngữ + Framework đích]`
- **REA Session Status**: `CLOSED & CLEAN` (hoặc đường dẫn Evidence Bundle đã xuất)
- **Last Compile / Verification**: `[PASS / BUILD SUCCESSFUL]`

---

## 2. Key Reverse Engineered Findings (Evidence Ledger)
| Hạng mục / Hàm / Endpoint | Tọa độ (VA/RVA / Route / File) | Phân loại (`Observed` / `Inferred`) | Evidence ID & Mô tả cơ chế |
| :--- | :--- | :--- | :--- |
| `[Tên hàm / API]` | `0x140001230` / `/api/v1/...` | `Observed` | `ev_...` — [Mô tả ngắn] |

---

## 3. Reconstructed Modules & Verification Status
| File Dự Án Đích | Ánh xạ từ Hành vi Gốc | Trạng thái |
| :--- | :--- | :--- |
| `FileName` | `[Hành vi / Thuật toán tương ứng]` | ✅ Verified / ⏳ In Progress |

---

## 4. Open Unknowns & Next Steps Queue
1. **[PRIORITY]** [Hạng mục cần giải mã / tái tạo ngay đầu phiên tới]
2. **[MEDIUM]** [Hạng mục tiếp theo]
3. **[UNKNOWN]** [Các nhánh dữ liệu chưa có mẫu capture cần bổ sung]
```

---

## 3. Mẫu Prompt Khởi tạo Phiên Mới (In ra cuối phản hồi)

```markdown
### Prompt Khởi tạo Phiên Mới (Copy sang New Chat)

> Tiếp tục dịch ngược và tái tạo dự án **[Tên Project]**. Hãy dùng `read_file` đọc tài liệu bàn giao kèm sổ cái bằng chứng tại link file `handover-YYYY-MM-DD.md` của phiên trước và thực hiện hạng mục số 1 trong phần **Next Steps Queue**.
```
