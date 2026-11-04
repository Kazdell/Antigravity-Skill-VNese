---
name: dynamic-runtime-tracing
description: >-
  Quan sát hành vi lúc chạy (Runtime Observation), phân tích lưu lượng mạng
  (.har/.saz/.pcap/qlog), gắn hook động (Frida/GDB/LLDB/Browser CDP/Electron) và
  đóng gói môi trường giả lập AgentEnv từ ứng dụng thực tế.
---

# /dynamic-runtime-tracing — Runtime Observation, Network Captures & AgentEnv

Quy trình phân tích động, điều tra gói tin mạng và ghi nhận kịch bản tương tác chuẩn REA.

> **Instruction truy cập tài liệu chuyên sâu**:
> Sử dụng `read_file` mở trực tiếp các file trong thư mục `references/` (`runtime-observation.md`, `environments.md`, `evidence-workflows.md`) và `guides/` (`web-network-captures.md`, `pcap-analysis.md`, `browser-observation.md`, `browser-scenario-contract.md`, `electron-observation.md`, `frida-instrumentation.md`, `agent-environments.md`, `reverse-engineering-tools.md`) thuộc thư mục của skill `reverse-engineer-anything`.

## 1. Phân tích Lưu lượng Mạng Offline (`.har`, `.saz`, `qlog`, `.pcap`)

1. **Web Network Captures (`.har`, Fiddler `.saz`, Mitmproxy, HTTP/3 `qlog`)**:
   - Gọi `inspect_web_network_capture` với `detail: "summary"`.
   - Trích xuất bề mặt API tự động bằng `inspect_analysis_view` với `view: {kind: "api-surface", include_query_parameter_names: true}` để gom nhóm toàn bộ endpoint theo HTTP method, origin và path template.
   - Với payload gRPC/Protobuf, dùng `inspect_analysis_view` với `view: {kind: "protobuf", collection: "bodies" | "frames" | "fields"}`.
2. **Packet Captures (`.pcap`, `.pcapng`)**:
   - Dùng `inspect_pcap_capture` (`detail: "summary"`) để thống kê địa chỉ IP, truy vấn DNS, TLS SNI và phân bổ giao thức.
   - Dùng `inspect_packet_fields` và `inspect_packet_capture` khi cần lọc sâu theo trường TShark.

## 2. Quan sát Trình duyệt (Browser CDP) & Ứng dụng Electron

1. **Quan sát Thụ động (Passive Observation)**:
   - Dùng `list_browser_targets` / `list_electron_targets` kết nối tới cổng CDP loopback của trình duyệt hoặc app Electron đang mở.
   - Dùng `inspect_web_page`, `observe_web_session`, `observe_web_execution`, `inspect_web_event_listeners`, hoặc `capture_web_screenshot`.
   - Đối chiếu mã tĩnh và script lúc chạy bằng `reconcile_javascript_runtime`.
2. **Tự động hóa Kịch bản & Dựng `AgentEnv` (`capture_browser_scenario`)**:
   - Khai báo `timeout_ms` và `expected_status` cho mọi bước trong `actions`.
   - Truyền thông tin đăng nhập qua biến môi trường (`{source: "secret", secret_id: "..."}`), tuyệt đối không hỏi mật khẩu明文 (plaintext) trên chat.
   - Kiểm tra độ phủ bằng `inspect_analysis_view` (`view: {kind: "environment-coverage"}`) và xuất gói bằng `export_environment_bundle`.

## 3. Hook Động & Debug Native (`Frida`, `GDB`, `Rizin`, `LLDB`)

- Chỉ khởi chạy hoặc gắn (attach) vào tiến trình khi người dùng đã xác nhận rõ mục tiêu.
- Với **Frida**: `list_frida_processes` $\rightarrow$ `start_frida_session` $\rightarrow$ `load_frida_script` $\rightarrow$ thu thập kết quả $\rightarrow$ luôn đóng bằng `close_frida_session`.
- Với **GDB / Rizin**: Đóng phiên Ghidra (`close_binary`) trước khi chạy lệnh điều khiển debugger để tránh xung đột tài nguyên.
