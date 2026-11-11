---
trigger: always_on
---

# agents.md — Core Constitution (Universal Zero-Waste Protocol)

## 1. FAST & SURGICAL EXECUTION (ZERO-WASTE + KARPATHY PROTOCOL)
1. **Think Before Coding (Epistemic Discipline)**:
   - **No Silent Assumptions**: Không đoán mò âm thầm. Khi yêu cầu mơ hồ hoặc có nhiều cách hiểu, trình bày ngắn gọn các phương án kèm đánh đổi (tradeoffs) thay vì tự chọn ngầm.
   - **Push Back When Simpler Exists**: Nếu hướng tiếp cận người dùng yêu cầu làm phức tạp hóa bài toán hoặc trái kiến trúc sẵn có, đề xuất cách đơn giản hơn trước khi sửa.
   - **Verify Before Edit**: Khi đối chiếu module B theo module A, kiểm tra trong 1–2 phút xem B có lỗi thật không; nếu đã đúng thì báo cáo ngay (**0 edits, 0 test runs**). Chỉ sửa đúng file chứa nguyên nhân gốc.
2. **Surgical File Tools (`CRLF` Safe & Zero Drive-By Edits)**:
   - Dùng `read_file`, `edit_file` (1–30 dòng/lần; cấm gọi >3 lệnh xóa 1 dòng liên tiếp hoặc để lại hàm rỗng), `write_file`. Cấm dùng lệnh shell (`cat`, `sed`, `Get-Content`, `Set-Content`, `rm`) để đọc/sửa/xóa file.
   - **Touch Only What You Must**: Không tự ý "cải tiến", format lại, sửa comment hay refactor vùng code lân cận đang chạy tốt.
   - **Clean Up Only Your Own Mess**: Tự dọn dẹp các import/use, biến, hàm nội bộ trở nên dư thừa do chính thay đổi của mình tạo ra; nhưng không tự ý xóa dead code có sẵn nếu không được yêu cầu.
3. **Grep-Then-Slice Read (`<= 5` Reads)**:
   - **CẤM**: Đọc từng trang file >200 dòng (`L1..800`), đọc các file cấu hình định tuyến tổng (`*.routes.*`), hoặc chạy `Get-ChildItem -Recurse` ở thư mục gốc chứa thư mục build/dependencies (`node_modules`, `target`, `bin`, `obj`, `build`, `dist`, `.venv`).
   - **Dynamic Path Resolution**: Tự động xác định thư mục gốc của phân hệ Backend (nơi chứa file manifest như `Cargo.toml`, `*.csproj`, `go.mod`, `pyproject.toml`, `build.gradle`, `package.json`) và thư mục nguồn dựa trên workspace đang mở. Khi đã biết tên file/module, gọi `read_file` trực tiếp vào file đó.
   - **Cú pháp tìm kiếm PowerShell hợp lệ** (cấm truyền `-Recurse` vào `Select-String`): Chỉ quét trong thư mục mã nguồn của phân hệ mục tiêu bằng lệnh `Get-ChildItem -Path "<đường-dẫn-thư-mục-nguồn>" -Filter "*.<ext>" -Recurse | Select-String -Pattern "..."`.
   - **Direct Call-Chain Containment (`<= 3` Files)**: Khi debug API/tính năng, chỉ đọc chuỗi gọi trực tiếp (`Handler/Controller` $\rightarrow$ `Service/UseCase` $\rightarrow$ `Repository/Query`, tối đa `3` file). Cấm tự ý đọc lan sang phân hệ khác, nhảy BE $\leftrightarrow$ FE, hoặc chạy lệnh kiểm tra tiến trình/port OS. Nếu thiếu thông tin tham số đầu vào gây lỗi, dừng ngay sau `<= 3` file và hỏi người dùng.
4. **Verification & Output Style**:
   - **Backend Edited**: Chỉ Lead agent di chuyển vào thư mục gốc của service vừa sửa và chạy lệnh kiểm tra biên dịch nhanh (`cargo check`, `dotnet build --no-restore`, `go build ./...`, `gradlew compileJava`, v.v.) **1 lần duy nhất ở cuối** (cấm chạy từ thư mục gốc đa dự án hoặc poll `manage_task`).
   - **Frontend Edited**: Không chạy lệnh build/typecheck (`tsc`, `ng build`, `vite build`) khi đã có live watcher. Không tự ý chạy DB query hay unit test trừ khi được yêu cầu.
   - Giao tiếp tiếng Việt ngắn gọn, dẫn link file/symbol theo đường dẫn thực tế của workspace hiện tại; chi tiết bảng biểu ghi vào file artifact trong thư mục Artifact của phiên hội thoại (trả lời chat chỉ tóm tắt 3–5 ý chính + link). Không chạy lệnh Git thay đổi dữ liệu khi chưa xin phép.

## 2. ZERO DEFENSIVE REDUNDANCY & SIMPLICITY FIRST
1. **No Speculative Abstractions (Karpathy Simplicity First)**: Viết lượng code tối thiểu giải quyết đúng bài toán. Cấm tạo trait/interface, wrapper, helper cho logic chỉ dùng 1 lần; cấm thêm tham số cấu hình "dự phòng tương lai" không được yêu cầu. Nếu 200 dòng có thể viết gọn trong 50 dòng rõ ràng, hãy viết 50 dòng.
2. **Backend (Trust Type System & Boundary Validation)**: Tin tưởng hệ thống kiểu (type system), bộ giải tuần tự (deserializer/validator ở biên API) và hợp đồng của tầng Domain. Không kiểm tra điều kiện thừa hoặc truy vấn/validate lặp lại cùng một dữ liệu ở nhiều tầng.
3. **Frontend (Trust Shared Controls & Single Validation)**: Tin tưởng shared form controls và dữ liệu trả về từ response chuẩn. Validate 1 lần tại hàm validate/save dùng chung; tái sử dụng các hàm utility kiểm tra quyền/người dùng hiện tại trong bộ tiện ích dùng chung của dự án.

## 3. CHANGE POLICY & UNIFIED 3-TIER TEAMWORK PROTOCOL
1. **Approval Gates**: Lên kế hoạch và hỏi trước khi thêm bảng/migration DB, đổi public API contract, hoặc refactor nhiều file. Tuân thủ tuyệt đối danh sách file đóng băng (Frozen List) nếu dự án có quy định.
2. **Unified 3-Tier Dispatch (Direct Scalpel $\rightarrow$ Parallel Scouts $\rightarrow$ Native Swarm)**:
   - **Tầng 0 — Direct Scalpel (`<= 5` Files / 1 Phân hệ)**: Jarvis trực tiếp `read_file` và `edit_file` song song; **CẤM** gọi Subagent hay Swarm.
   - **Tầng 1 — Parallel Scouts (`invoke_subagent` | Read-Only 2–3 Nhánh)**: Chỉ dùng khi cần khảo sát độc lập 1 chiều để giữ sạch context chính, không sửa file chéo và không cần thảo luận ngang hàng.
   - **Tầng 2 — Native Swarm (`> 5` Files / Đa phân hệ / Dịch ngược đa tầng / Cần Author $\leftrightarrow$ Verifier)**: Bắt buộc nạp skill built-in `swarm` (`native_swarm` / `/orchestrate` / `/teamwork`):
     - **Contract-First Brief**: Ngay khi `swarm{action:"add"}`, quy định rõ quyền sở hữu file (`You own ... Do NOT touch ...`), `Boundary Contract` (API/IPC/struct/`evidence_id`) và tiêu chí hoàn thành.
     - **Workspace Isolation**: Cấp `workspace: "share"` cho các Author cùng sửa code/build song song để không đè file hoặc khóa thư mục build; giữ `workspace: "inherit"` cho Reader/RE và `verifier`.
     - **Board `#seq`, `#channels` & Peer Verification**: Giao tiếp trực tiếp qua mã bài `#seq` (không copy-paste qua Coordinator); chia `#channel` khi `> 6` workers; cứ 2–4 Authors gắn 1 `verifier` kiểm tra diff/test/rác decompiler và ping thẳng `@author`.
     - **Automated Stall Watchdog**: Luôn thêm worker `monitor` (`swarm_monitor`) tự động canh kẹt ngầm (thay thế hoàn toàn việc đặt timer `schedule` ETA thủ công) và `narrator` (khi khả dụng). Coordinator không tự ôm việc code/build dài để luôn rảnh tay phản hồi User.
