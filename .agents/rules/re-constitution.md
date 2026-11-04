---
trigger: always_on
---

# re-constitution.md — Core Reverse Engineering Constitution (REA Protocol)

## 1. EVIDENCE PROVENANCE &EPISTEMIC DISCIPLINE
1. **Strict Separation of Facts (`Observed` vs `Derived` vs `Inferred` vs `Unknown`)**:
   - **Observed**: Byte ranges, SHA-256 digests, headers, import/export tables, static AST nodes, or recorded runtime events directly returned by REA tools.
   - **Derived / Inferred**: Decompiler pseudocode, heuristic route patterns, selector candidates, or cross-layer IPC pairings. Never claim decompiled pseudocode is original source code or that static presence proves runtime execution.
   - **Unknown**: Unexamined pages, encrypted/stripped sections, dynamic dispatch targets, or uncaptured network flows. Missing coverage is `unknown`, never proof of absence.
2. **Evidence Citation**:
   - Always cite the exact `evidence_id` (and `parent_evidence_id` / `node_id` / `source_index` / virtual address / file offset) returned by REA MCP or CLI tools when explaining findings.
   - Never fabricate or guess an `evidence_id`, function address, or runtime observation that was not actually executed.

## 2. SUMMARY-FIRST & CONTEXT EFFICIENCY
1. **Summary-First Exploration**:
   - Always request `detail: "summary"` first for large or unknown-size targets (`analyze_javascript_application`, `inspect_artifact`, `inspect_binary_layout`, `inspect_managed_artifact`, `inspect_wasm_artifact`, `inspect_macho`, `inspect_web_network_capture`, `inspect_native_exports`).
   - Use the retained `parent_evidence_id` with `inspect_analysis_view` to page specific collections (`modules`, `routes`, `exports`, `functions`, `occurrences`) and select only necessary fields.
   - Over MCP, `analyze_function` is summary-first by default; page specific facets (`pseudocode`, `assembly`, `callers`, `callees`, `value_flow_operations`) via `inspect_analysis_view` instead of re-running decompilation.
2. **Zero Redundant Re-Analysis**:
   - Check `binary_session` and retained Evidence before opening a target or repeating an analysis call.
   - For CLI outputs on large targets, redirect JSON output to a file inside the conversation's `scratch/` directory (or `$env:TEMP` on Windows) and query it via `rea inspect-analysis-view` rather than dumping megabytes of raw JSON into chat context.

## 3. DYNAMIC PATH & SKILL RESOLUTION (ZERO HARDCODED PATHS)
1. **Target & Host Path Resolution**:
   - Resolve target artifact paths dynamically from the user's prompt or active workspace. Never assume a sample app or fixture is the user's target unless explicitly requested.
   - Resolve skill reference files (`references/*.md`, `guides/*.md`, `resources/...`) relative to the active `reverse-engineer-anything` skill directory reported in `Available skills` using native `read_file`.
2. **Native File Tools & Report-First Output**:
   - Use `read_file`, `edit_file`, `write_file` instead of shell text commands (`cat`, `sed`, `Get-Content`, `Set-Content`).
   - Record comprehensive finding ledgers, address tables, and API surface maps into Markdown Artifacts; keep chat replies concise (3–5 core findings with links).

## 4. FAST & SURGICAL EXECUTION (ZERO-WASTE + KARPATHY PROTOCOL)
1. **Think Before Reversing / Coding**:
   - **No Silent Assumptions**: Không đoán mò giao thức, địa chỉ hàm hay hành vi ẩn. Khi dữ liệu tĩnh/động có nhiều cách giải thích, nêu rõ các giả thuyết kèm mức độ tin cậy (`Observed` vs `Inferred`) và hướng kiểm chứng tiếp theo.
   - **Verify Before Edit**: Trước khi sửa script hook, bridge hoặc code tái tạo, xác minh rõ nguyên nhân gốc; không sửa mò hoặc chạy lại phân tích nặng không cần thiết.
2. **Surgical File Tools & Grep-Then-Slice (`<= 5` Reads)**:
   - Dùng `edit_file` trong phạm vi 1–30 dòng/lần (`CRLF` safe); cấm gọi >3 lệnh xóa 1 dòng liên tiếp hoặc dùng lệnh shell (`rm`, `del`, `Set-Content`) để sửa/xóa file.
   - **Touch Only What You Must & Clean Up Own Mess**: Khi sửa code/hook/tài liệu, không tự ý format lại hay refactor code lân cận; tự dọn dẹp import/biến mồ côi do chính thay đổi của mình tạo ra.
   - **Cú pháp tìm kiếm PowerShell 5.1 hợp lệ**: Chỉ quét trong thư mục đích bằng `Get-ChildItem -Path "<đường-dẫn-thư-mục-đích>" -Filter "*.*" -Recurse | Select-String -Pattern "..."` (cấm truyền `-Recurse` vào `Select-String` hoặc quét tràn vào `node_modules`/`build`/`target`).
3. **Change Policy & Fractal Pyramid Subagent Protocol (`<= 5` Files = Direct; `> 5` Files = Pyramid Tier 1..5)**:
   - **Approval Gates**: Xin phép người dùng trước khi patch binary gốc, thay đổi kiến trúc dự án tái tạo, hoặc sửa hàng loạt file.
   - **Fractal Pyramid Threshold**: Tác vụ `<= 5` file / `<= 2` binary: Cấp 1 trực tiếp thực thi, **CẤM** gọi Subagent. Tác vụ `> 5` file hoặc dịch ngược/tái tạo đa tầng (Native Binary + JS Bundle + Network Capture, áp dụng cho cả Read-Only Audit lẫn Reconstruction): Bắt buộc kích hoạt **Kim tự tháp đệ quy (`/orchestrate`)** trên `Workspace: "inherit"` với 3 role (`pyramid-lead` bật `EnableSubagentTools`, `pyramid-worker` & `victory-auditor` tắt `EnableSubagentTools`; `Span of Control <= 4` nhánh/Lead, `<= 4` file/Worker, tối đa **Cấp 5**; **1 Domain/Layer = 1 `pyramid-lead` Cấp 2**, cấm dùng `research` phẳng thay thế `pyramid-lead`).
   - **Bounded 1-Hop Mesh & Asymmetric Overlap**: Chỉ liên lạc (`send_message`) với Cấp trên trực tiếp, Đồng cấp có vùng giao thoa (`Max 1 Peer Ping-Pong`), và Cấp dưới trực tiếp (cấm vượt cấp). Nhánh $i$ giữ quyền sửa vùng giao $(i \cap i+1)$ và gửi `[50% MILESTONE PUSH]`; Nhánh $i+1$ kiểm chéo `Read-Only Cross-Check` theo `evidence_id`. `victory-auditor` (chỉ đặt ở Cấp 1–3) có quyền `Surgical Fix`.
   - **Mandatory Upfront ETA, Dynamic Buffer (`+5%..10%`) & `120%` Enforcement Ceiling**: **TRƯỚC KHI** gọi bất kỳ Subagent nào, Cấp 1 bắt buộc công bố rõ với User **Sơ đồ Cây Kim Tự Tháp (Cấp 1 $\rightarrow$ Cấp $N$)** và **Bảng Cam kết `100% ETA` (Trần tối đa `120% ETA`)** kèm biên độ đệm động $\Delta \in [5\%, 10\%]$ giữa các mốc tỉ lệ nghịch với thời gian tổng $T_{\text{base}}$ ($\le 6\text{p}: +10\%$; $6\text{p}..60\text{p}$: giảm dần $10\% \rightarrow 5\%$; $\ge 60\text{p}$: tối đa $+5\%$). Đặt `schedule` tại các mốc đã cộng đệm (`25%+Δ` tách sớm, `50%+Δ` `Work Stealing`, `75%+Δ` hỗ trợ bọc lót, `100%+Δ` gom kết quả) và chỉ cưỡng chế thúc giục / `Fast-Track Takeover` / `manage_subagents(kill)` khi vượt trần tối đa **`120% ETA`**.
