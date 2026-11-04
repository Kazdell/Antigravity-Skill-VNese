---
name: orchestrate
description: >-
  Điều phối đa tác tử Kim Tự Tháp Đệ Quy (Fractal Pyramid Orchestration kết hợp
  /teamwork và /orchestrate, Cấp 1 -> Cấp 5) cho dịch ngược đa tầng & tái tạo
  tính năng: Hội thoại Ngầm Không Vượt Cấp, Giao thoa Bất đối xứng theo
  evidence_id, Kiểm soát Tiến độ Chủ động (25%-50%-75%-100% ETA) và Smart
  Victory Auditor.
---

# /orchestrate — Fractal Pyramid, Bounded Mesh & Proactive ETA Reverse Engineering Orchestration

## 1. Pha 1: Định tuyến Phân tầng, Ước lượng ETA & Khởi tạo Role (`Lead Pre-Scan <= 30s`)
1. **Ngưỡng Kích hoạt & Nguyên tắc 1 Domain/Layer = 1 Lead Cấp 2 (Áp dụng cho cả Read-Only Audit và Reconstruction)**:
   - Main Agent (Cấp 1) trực tiếp làm rõ yêu cầu với User; **CẤM** đẩy qua hộp đen trung gian để hỏi lại User.
   - `<= 5` file / `<= 2` binary đơn giản: Cấp 1 trực tiếp phân tích & sửa file; **CẤM** gọi Subagent.
   - `> 5` file hoặc dịch ngược đa tầng (VD: Native DLL/SO + Electron `.asar`/APK UI + Network Capture `.har`/`.pcap`): Mỗi Tầng/Domain chỉ tương ứng với **1 Team Lead Cấp 2** (`pyramid-lead`), tự phân rã tiếp xuống **Cấp 3..5** (cấm gọi `research` phẳng thay cho `pyramid-lead`).
2. **Công bố Bắt buộc Sơ đồ Kim Tự Tháp, Đệm ETA Động (`+5%..10%`) & Trần `120% ETA` Trước Khi Chạy**:
   - **TRƯỚC KHI** gọi `invoke_subagent`, Cấp 1 bắt buộc xuất thông báo ngắn gọn cho User gồm: **(a) Sơ đồ phân cấp Kim Tự Tháp (Cấp 1 $\rightarrow$ Cấp 2 $\rightarrow$ Cấp 3..5)** và **(b) Bảng Cam kết `100% ETA` (Trần cưỡng chế tối đa `120% ETA`) kèm biên độ đệm động $\Delta \in [5\%, 10\%]$ giữa các mốc**:
     - Định mức cơ sở ($T_{\text{base}}$): Leaf Worker `~30s–45s/file-hoặc-artifact`; `Victory Auditor` `~30s–45s`; `ETA_Cụm = Max(ETA các nhánh song song) + 45s`.
     - **Đệm ETA động ($\Delta$) tỉ lệ nghịch với $T_{\text{base}}$**: $T_{\text{base}} \le 6\text{ phút} \Rightarrow \Delta = 10\%$; $6\text{ phút} < T_{\text{base}} < 60\text{ phút} \Rightarrow \Delta(\%) = 10 - 5 \times \frac{T_{\text{base(phút)}} - 6}{54}$; $T_{\text{base}} \ge 60\text{ phút} \Rightarrow \Delta = 5\%$ (tối đa).
3. **Định nghĩa 3 Role Chuẩn qua `define_subagent` (`Workspace: "inherit"`)**:
   - `pyramid-lead` (`EnableWriteTools: true`, `EnableSubagentTools: true`): Team Lead Cấp 2, 3, 4. Mỗi Lead quản lý tối đa **`4` nhánh con** (`Span of Control <= 4`). Nếu 1 nhánh con có `> 4` file/artifact hoặc nhiều cụm module con, Lead phân cấp tiếp cho Sub-Team Lead cấp dưới (tối đa tới **Cấp 5**).
   - `pyramid-worker` (`EnableWriteTools: true`, `EnableSubagentTools: false`): Leaf Worker ở tầng lá (tối đa **`4` file/artifact trên mỗi Worker**; khóa cứng trần đệ quy tại Cấp 5).
   - `victory-auditor` (`EnableWriteTools: true`, `EnableSubagentTools: false`): Agent Hậu kiểm thông minh có quyền **Surgical Fix**, **chỉ đặt tại Cấp quản lý 1, 2 và tối đa Cấp 3** (CẤM tạo ở Cấp 4–5 để tránh độ trễ chuỗi).
4. **Báo cáo File-First**: Mọi bảng ánh xạ địa chỉ, danh sách API surface và hồ sơ `evidence_id` chi tiết phải ghi vào file Artifact Markdown; khung chat chỉ trả tóm tắt 3–5 phát hiện cốt lõi kèm link.

## 2. Pha 2: Giao thoa Bất đối xứng & Hội thoại Ngầm Không Vượt Cấp (`Bounded Command & Peer Mesh`)
1. **Quy tắc Liên lạc 1 Bậc (`No Cross-Tier Leapfrogging`)**:
   - Mỗi Agent ở Cấp $K$ chỉ được cấp `conversationId` và liên hệ (`send_message`) với **3 nhóm**: (a) **Cấp trên trực tiếp** (Cấp $K-1$), (b) **Đồng cấp cùng đội có vùng giao thoa với mình** (Cấp $K$), và (c) **Cấp dưới trực tiếp do mình tạo ra** (Cấp $K+1$). **CẤM** nhắn tin vượt cấp hoặc nhắn chéo sang nhánh của đội khác.
2. **Ma trận Giao thoa Bất đối xứng (`Contract-First Asymmetric Overlap`) & Đồng bộ Ngang cấp (`Max 1 Peer Ping-Pong`)**:
   - Khi chia việc cho 2 nhánh liền kề ($i$ và $i+1$ — giữa các tầng Native/JS/Network hoặc các module tái tạo), Lead chốt sẵn **`Boundary Contract`** (IPC bridge, packet struct, function signature, `evidence_id`) và cấp `conversationId` đồng cấp cho nhau:
   - **Nhánh $i$ (`Primary Boundary Owner`)**: Giữ quyền `Primary Write` trên vùng $i$ **CỘNG VỚI toàn bộ vùng giao thoa $(i \cap i+1)$**. Ngay khi hoàn thành **50% khối lượng (bao gồm vùng giao $i \cap i+1$)**, Nhánh $i$ bắt buộc gửi **1 tin `send_message` (`[50% MILESTONE PUSH]`)** cho cả **Team Lead** và **Nhánh $i+1$** (thông báo `evidence_id` và chữ ký hàm/struct vùng giao đã chốt).
   - **Nhánh $i+1$ (`Secondary Cross-Checker`)**: Giữ quyền `Primary Write` trên vùng riêng của $i+1$, và thực hiện **`Read-Only Cross-Check`** trên vùng giao $(i \cap i+1)$ đối chiếu với bằng chứng `Observed` của nhánh $i+1$.
     - Nếu phát hiện lệch hợp đồng/giao thức, Nhánh $i+1$ gửi **tối đa 1 tin `send_message` trực tiếp cho Nhánh $i$** để Nhánh $i$ tự sửa ngay (giới hạn cứng **`Max 1 Peer Ping-Pong`**: nếu sau 1 vòng vẫn chưa thống nhất, dừng tranh luận ngay và ghi vào `[CROSS-CHECK REPORT: CONFLICT]` đẩy lên cấp trên).
3. **Ràng buộc Bắt buộc trong mọi Prompt (`Mandatory Constraints`)**:
   > 1. **Summary-First & Evidence Provenance**: Luôn gọi công cụ REA với `detail: "summary"` trước và phân trang qua `inspect_analysis_view`. Ghi rõ `evidence_id` và phân loại `Observed` / `Inferred` / `Unknown`.
   > 2. **Strict Scope & Surgical `edit_file`**: Khi viết/sửa code tái tạo, chỉ sửa đúng file trong `Primary Write Scope` bằng `edit_file` (CẤM sửa đè vào vùng `Read-Only Cross-Check`, CẤM `write_file(Overwrite: true)` trên file có sẵn).
   > 3. **50% Milestone Push**: Ngay khi xong 50% khối lượng (và vùng giao $i \cap i+1$), phải bắn 1 tin `send_message` ngắn gọn báo tiến độ kèm `evidence_id` cho Team Lead và Peer kế cận.
   > 4. **Zero-Waste (`read_file <= 5` & No Unsolicited Build/Test)**: CẤM tự ý chạy lệnh build/test nặng khi không được giao.

## 3. Pha 3: Kiểm soát Tiến độ Chủ động với Đệm Động (`25%+Δ -> 50%+Δ -> 75%+Δ -> 100%+Δ`, Trần Cưỡng Chế `120% ETA`)
Lead sử dụng công cụ hẹn giờ `schedule(DurationSeconds, TimerCondition: "any")` cộng sẵn biên độ đệm động $\Delta \in [5\%, 10\%]$ giữa các mốc để tránh thúc giục sớm khi Subagent đang xử lý hợp lệ:
1. **Mốc `25% + Δ ETA` (Kiểm tra Gia tốc Khởi đầu — Target $\ge 25\%$ khối lượng)**:
   - Nếu sau thời gian đệm $\Delta$ vẫn có nhánh sa lầy vào decompile/dump quá sâu chưa ra kết quả: Lead gửi `send_message` khóa phạm vi địa chỉ/hàm mục tiêu, hoặc **tách nhỏ nhánh nặng (`Early Split`)**.
2. **Mốc `50% + Δ ETA` (Cổng Tái Cấu Trúc Plan Giữa Giờ — Target $\ge 50\%$ khối lượng & xong vùng giao $i \cap i+1$)**:
   - Nếu quá mốc `50% + Δ` mà nhánh chưa gửi `[50% MILESTONE PUSH]`: Lead thực hiện **`Dynamic Re-Plan & Work Stealing`** — san sẻ các file/module chưa làm sang nhánh đồng cấp đã xong sớm.
3. **Mốc `75% + Δ ETA` (Cổng Hội Tụ & Bọc Lót — Target $\ge 80\%$ khối lượng)**:
   - Nếu quá mốc `75% + Δ` mà nhánh còn chậm: Nhắc nhẹ để nhánh chốt phần đang làm dở và chuẩn bị phương án bọc lót từ Team Lead / `Victory Auditor`.
4. **Mốc `100% + Δ ETA` đến Trần Tối Đa `120% ETA` (Cửa Sổ Đệm Hoàn Tất & Chốt Chặn Cưỡng Chế `120%`)**:
   - **Từ `100%` $\rightarrow$ `100% + Δ` (tối đa `120% ETA`)**: Cửa sổ đệm ân hạn (`Grace Window`) cho phép các nhánh đang hoàn thiện nốt bước cuối về đích tự nhiên mà không bị thúc giục gắt hay cắt ngang.
   - **Tại Trần Cưỡng Chế Tối Đa `120% ETA`**: Chỉ khi chạm mốc **`120% ETA`** mà vẫn còn Subagent treo hoặc chưa hoàn tất mới kích hoạt cưỡng chế thúc giục tối hậu, `Fast-Track Takeover` và `manage_subagents(Action: "kill")` để Lead/Auditor tự chốt phần cuối.

## 4. Pha 4: Hậu kiểm Thông minh (`Smart Victory Auditor` Cấp 1–3) & Đóng phiên
1. **Gom Báo cáo từ Tầng Lá (Cấp 4–5 $\rightarrow$ Cấp 3/2)**: Các nhánh Cấp 4–5 đẩy thẳng danh sách file/bằng chứng + `[CROSS-CHECK REPORT]` lên cấp quản lý 3 hoặc 2.
2. **Cơ chế `Smart Victory Auditor` (Tại Cấp quản lý 3 / Cấp 2)**:
   - **Nếu Peer Sync đã khớp 100% (`[CROSS-CHECK: PASS]`)**: `victory-auditor` quét nhanh `git diff` dự án đích (đảm bảo không sót rác decompiler `v1`, `puVar2`, `goto` và khớp `evidence_id`) rồi đóng dấu `VICTORY_VERIFIED`.
   - **Nếu còn xung đột (`[CROSS-CHECK: CONFLICT]`) hoặc gánh phần `Fast-Track Takeover` ở mốc 75% ETA**: `victory-auditor` dùng quyền **`Surgical Fix`** sửa dứt điểm ngay trên file rồi báo `VICTORY_VERIFIED` về Team Lead.
3. **Chief Final Audit, Session Cleanup & Compile Verify (Cấp 1)**:
   - Đảm bảo mọi phiên `open_binary` / `Frida` đã được đóng sạch (`close_binary`), và chỉ Cấp 1 chạy **1 lệnh kiểm tra biên dịch nhanh ở cuối** nếu có sửa code Backend đích.
