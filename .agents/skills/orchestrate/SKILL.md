---
name: orchestrate
description: >-
  Điều phối đa tác tử Hợp nhất 3 Tầng (Direct Scalpel -> Parallel Scouts ->
  Built-in Native Swarm) cho phát triển đa phân hệ & dịch ngược tái tạo tính
  năng: Contract-First Brief, Evidence Provenance, workspace "share" chống đụng
  file, Multi-Perspective Verifier và Stall Monitor tự động.
---

# /orchestrate — Unified 3-Tier & Native Swarm Orchestration

Chuẩn điều phối làm việc nhóm hợp nhất giữa kỷ luật **Clean-Room / Evidence Provenance** và động cơ **Built-in Native Swarm** (`swarm`, `swarm_monitor`, `swarm_narrator`).

## 1. Phễu Phân Loại 3 Tầng (3-Tier Triage Gate)

- **Tầng 0 — Direct Scalpel (`<= 5` files / `<= 2` binaries nhỏ)**: Jarvis trực tiếp phân tích & sửa file song song; **CẤM** gọi Subagent hay Swarm.
- **Tầng 1 — Parallel Scouts (`invoke_subagent` | Read-Only 2–3 nhánh)**: Dùng cho tra cứu độc lập một chiều không sửa file chéo và không cần thảo luận ngang hàng.
- **Tầng 2 — Native Swarm (`> 5` files / Đa phân hệ / Dịch ngược đa tầng DLL + UI + Network)**: Bắt buộc đọc `native_swarm/SKILL.md` và khởi chạy `swarm`.

---

## 2. Quy Trình Native Swarm 4 Pha (Tầng 2)

### Pha 1: Chốt `Boundary Contract` & Khởi Tạo Đội Hình (`Start Turn`)
1. Đọc `native_swarm/SKILL.md` nếu chưa nạp trong phiên. Đặt tên Swarm qua `swarm{action: "title", title: "..."}`.
2. Tạo kênh riêng (`#native-re`, `#protocol`, `#backend`, `#ui`) nếu có `> 6` workers hoặc nhiều mảng độc lập.
3. Khởi tạo các **Domain Investigators / Authors** (`swarm{action: "add"}`):
   - Dùng `workspace: "share"` cho các Author cùng sửa code/biên dịch song song; giữ `workspace: "inherit"` cho các worker chỉ đọc binary/capture và `verifier`.
   - Trong `brief` công khai trên bảng tin, chỉ định rõ:
     - **File/Layer Ownership**: Phạm vi sở hữu độc quyền (`You own ... Do NOT touch ...`).
     - **Boundary Contract**: Chữ ký IPC/API, cấu trúc gói tin, hoặc `evidence_id` biên.
     - **Skill & Evidence Discipline**: Chỉ rõ `.agents/skills/<name>/SKILL.md` cần đọc; gắn nhãn `Observed` / `Inferred` / `Unknown` kèm `evidence_id`; cấm để sót rác giả mã (`v1`, `puVar2`, `goto`).
     - **Pipeline Handoff**: Xong việc đăng báo cáo lên bảng và ping `@verifier` kèm mã `#seq`.
4. Khởi tạo **`verifier`** (`1` verifier cho mỗi `2–4` workers, `workspace: "inherit"`):
   - Đối chiếu chéo `Boundary Contract`, kiểm tra chất lượng code/rác decompiler và chạy scoped check; nếu lệch contract thì ping thẳng `@<author> xem #<seq>` để tác giả tự sửa (cấm 2 người sửa chung 1 file).
5. Khởi tạo **`monitor`** (`swarm_monitor` — tự động canh kẹt mỗi 3 phút, thay thế hoàn toàn hẹn giờ `schedule` ETA thủ công) và **`narrator`** (nếu có trong danh sách skills). Hiển thị link Swarm Board cho User và kết thúc lượt khởi tạo.

### Pha 2: Điều Phối Qua `#seq`, Đổi Model Nóng & `fork`
- Coordinator giữ lượt cực ngắn để luôn sẵn sàng trả lời User; không tự ôm tác vụ build/decompile dài.
- Mọi bàn giao trên bảng tin đều trỏ theo số thứ tự bài đăng (`#4`, `#7`), cấm copy-paste lặp lại nội dung.
- Dùng `swarm_wait{since: LAST_SEQ}` khi chờ kết quả; dùng `swarm{action: "model"}` nếu có thành viên chạm trần quota; dùng `fork: "<member>"` khi cần tách nhánh từ một ngữ cảnh điều tra đã ấm.

### Pha 3: Hợp Nhất Code (`Land Work`) & Thẩm Định Cuối
1. Sau khi `@verifier` đóng dấu PASS, gộp thay đổi từ các nhánh `workspace: "share"` về workspace chính trước khi `kill` worker đó.
2. Đảm bảo mọi phiên phân tích nhị phân/runtime (`close_binary`, Frida) đã đóng sạch; Coordinator chạy **1 lệnh kiểm tra biên dịch nhanh ở cuối** tại phân hệ đích.
3. Đăng `[done] <summary>` (không có `@`) lên bảng tin để tắt `monitor`, ghi chi tiết bảng ánh xạ dài vào Artifact Markdown và tóm tắt 3–5 ý chính kèm link cho User.
