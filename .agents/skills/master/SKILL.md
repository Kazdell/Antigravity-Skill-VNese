---
name: master
description: >-
  Siêu kỹ năng điều phối hợp nhất (Unified Master Pipeline) cho Dịch ngược &
  Tái tạo tính năng bắc cầu tương đối qua /reframe + /grill-me -> /plan ->
  /orchestrate + /goal -> /audit với Kim Tự Tháp Đệ Quy (Cấp 1 -> Cấp 5), Kiểm
  soát Tiến độ 25%-50%-75%-100% ETA và Smart Victory Auditor.
---

# /master — Unified Reverse Engineering & Reconstruction Pipeline (`Relative Bridge + Grill-Me + Goal Protocol`)

## Pha 1: Bắc cầu [../reframe/SKILL.md](../reframe/SKILL.md) + [../static-binary-triage/SKILL.md](../static-binary-triage/SKILL.md) + Chế độ `/grill-me`
1. Đọc và áp dụng quy trình tại [../reframe/SKILL.md](../reframe/SKILL.md) và [../static-binary-triage/SKILL.md](../static-binary-triage/SKILL.md) (khảo sát tĩnh `detail: "summary"`, ghi toàn bộ bảng phát hiện và `evidence_id` vào Artifact, khung chat chỉ tóm tắt 3–5 ý cốt lõi).
2. **Kích hoạt `<GRILL_ME>` (Trừ khi User ghi `[skip-interview]` hoặc yêu cầu phân tích đơn lẻ đã rõ 100%)**:
   - Tự kiểm tra nhanh các bằng chứng tĩnh (`<= 3` lệnh summary) nếu câu hỏi có thể trả lời trực tiếp từ binary/artifact.
   - Với mọi nhánh kiến trúc tái tạo, phạm vi mục tiêu hoặc giả thuyết giao thức cần chốt: đi dọc từng nhánh của cây quyết định bằng công cụ `ask_question` (**hỏi từng câu một — `1 question at a time`**), luôn đặt phương án khuyến nghị lên đầu kèm tiền tố `(Recommended)` cho đến khi thống nhất hoàn toàn với User trước khi sang Pha 2.

## Pha 2: Bắc cầu [../plan/SKILL.md](../plan/SKILL.md) + [../feature-reconstruction/SKILL.md](../feature-reconstruction/SKILL.md) — Cây Kim Tự Tháp & Cam kết ETA
1. Đọc và áp dụng quy trình tại [../plan/SKILL.md](../plan/SKILL.md) và [../feature-reconstruction/SKILL.md](../feature-reconstruction/SKILL.md).
2. Lập kế hoạch `implementation_plan.md` ánh xạ từ các mục `Observed` sang kiến trúc dự án đích theo **Cây Kim Tự Tháp (`1 Domain/Layer = 1 pyramid-lead Cấp 2` $\rightarrow$ Cấp 3..5; `Span of Control <= 4` nhánh/Lead, `<= 4` file/Leaf Worker, tối đa Cấp 5)**, **Ma trận Giao thoa Bất đối xứng theo `evidence_id` trên vùng $(i \cap i+1)$** và công bố **Tổng `100% ETA` (Trần tối đa `120% ETA`, kèm đệm động $\Delta \in [5\%, 10\%]$ tỉ lệ nghịch với thời gian: $\le 6\text{p}$ dùng $10\%$, $\ge 60\text{p}$ tối đa $5\%$)** cho User **trước khi** khởi chạy bất kỳ Subagent nào.

## Pha 3: Bắc cầu [../orchestrate/SKILL.md](../orchestrate/SKILL.md) + [../create/SKILL.md](../create/SKILL.md) + Chế độ `/goal`
1. Đọc và thực thi đúng quy trình 4 pha tại [../orchestrate/SKILL.md](../orchestrate/SKILL.md) kết hợp [../create/SKILL.md](../create/SKILL.md):
   - Tác vụ `<= 5` file / `<= 2` binary: Cấp 1 trực tiếp thực thi (`0` Subagent).
   - Tác vụ `> 5` file hoặc dịch ngược/tái tạo đa tầng (cả Read-Only Audit lẫn Reconstruction): Bắt buộc định nghĩa và khởi tạo `pyramid-lead` (1 Lead Cấp 2/Layer, tự đẻ tiếp Cấp 3..5), `pyramid-worker`, `victory-auditor` trên `Workspace: "inherit"` (cấm gọi `research` phẳng); bật hội thoại ngầm 1 bậc (`Max 1 Peer Ping-Pong` + `[50% MILESTONE PUSH]`) và kiểm soát tiến độ chủ động bằng `schedule` tại các mốc có đệm động `25%+Δ - 50%+Δ - 75%+Δ - 100%+Δ` (chỉ cưỡng chế thúc giục / `kill` khi chạm trần `120% ETA`).
2. **Kích hoạt `<GOAL>` (Autonomous Goal Persistence Loop)**:
   - Thiết lập danh sách nghĩa vụ tái tạo (`Obligation Ledger / Goal Checklist`). Không được dừng lại giữa chừng khi mục tiêu chưa đạt 100%; nếu có nhánh chậm/kẹt hoặc lệch giao thức vùng giao, tự động kích hoạt `Work Stealing` (ở 50% ETA), `Fast-Track Takeover` (ở 75% ETA) hoặc vòng sửa bổ sung cho tới khi toàn bộ nghĩa vụ đạt `100% PASS`.

## Pha 4: Bắc cầu [../audit/SKILL.md](../audit/SKILL.md) — Hậu kiểm `Victory Auditor`, Biên dịch & Đóng phiên
1. Đọc và áp dụng quy trình thẩm định tại [../audit/SKILL.md](../audit/SKILL.md) kết hợp báo cáo `VICTORY_VERIFIED` từ `victory-auditor` (Cấp 1–3).
2. **Final Cleanup & Verify**: Đóng toàn bộ phiên REA đang mở (`close_binary`). Nếu có sửa Backend dự án đích, chỉ Cấp 1 chạy **1 lệnh kiểm tra biên dịch nhanh ở bước cuối cùng** (nếu biên dịch báo lỗi, theo chế độ `<GOAL>` phải tự sửa dứt điểm đến khi biên dịch thành công; chỉ chạy scoped Golden I/O test khi User yêu cầu).
