---
name: feature-reconstruction
description: >-
  Dịch ngược một tính năng từ ứng dụng mẫu (Web, Electron/JS, Desktop, Mobile APK,
  hoặc Binary), lập hồ sơ bằng chứng (Evidence & Obligation Ledger) và xây dựng
  lại tính năng tương đương cho dự án đích.
---

# /feature-reconstruction — Evidence-Driven Feature Reconstruction

Quy trình 4 giai đoạn để giải phẫu một tính năng từ ứng dụng/bản build bên ngoài bằng REA và tái tạo sạch sẽ (clean-room / evidence-backed implementation) vào dự án của bạn.

> **Instruction truy cập tài liệu chuyên sâu**:
> Sử dụng `read_file` để đọc `javascript-artifact-reconstruction.md`, `javascript-application-workflows.md`, `native-function-matching.md`, `reconstruction-coverage.md`, `reconstruction-obligation-ledgers.md`, và `reconstruction-readiness.md` trong thư mục `guides/` của skill `reverse-engineer-anything`.

## Giai đoạn 1: Khảo sát & Định vị Hạt giống Tính năng (Feature Seed Discovery)

1. **Xác định Điểm vào (Entrypoint)**:
   - **Ứng dụng Web / HAR capture**: Dùng `inspect_web_network_capture` (`view: {kind: "api-surface"}`) hoặc `inspect_web_event_listeners` để tìm route, API endpoint và DOM handler của tính năng.
   - **Ứng dụng Electron / JavaScript (`.asar` / bundle)**: Dùng `analyze_javascript_application` (`detail: "summary"`) và `view: {kind: "javascript-routes"}` hoặc tìm theo chuỗi/IPC channel.
   - **Ứng dụng Android (`.apk`)**: Dùng `inspect_android_package` và `search_android_classes` để định vị Activity/ViewModel/Service/Repository xử lý tính năng.
   - **Ứng dụng Native (`.exe`, `.dll`, `.so`, Mach-O)**: Dùng `open_binary` $\rightarrow$ `search_strings` / `list_procedures` để tìm hàm xử lý.

## Giai đoạn 2: Truy vết Luồng Dữ liệu & Trích xuất Thuật toán (Feature Tracing)

1. **Truy vết Đồ thị Phụ thuộc**:
   - Trên JS/Electron: Gọi `trace_application_feature` (chọn `direction: "outgoing"` hoặc `"both"` quanh hạt giống vừa tìm được) để lấy danh sách module, IPC boundary và hàm liên quan.
   - Trên Android: Gọi `inspect_android_method` và `trace_android_references`.
   - Trên Native: Gọi `analyze_function` và `build_call_path` để dựng luồng gọi hàm và đọc `pseudocode` + `value_flow_operations`.
2. **Ghi nhận Hồ sơ Nghĩa vụ Tái tạo (Reconstruction Obligation Ledger)**:
   - Liệt kê rõ: **(a) Cấu trúc dữ liệu / Schema / SQLite / API payload**, **(b) Quy tắc biến đổi & thuật toán cốt lõi**, **(c) Các trường hợp biên & xử lý lỗi**, và **(d) Các phần còn `Unknown`** (dùng `record_unknown` nếu cần theo dõi trong phiên).

## Giai đoạn 3: Thiết kế & Tái tạo vào Dự án Đích (Clean Implementation)

1. Tách biệt hoàn toàn giữa **Hành vi quan sát được từ bằng chứng (`Observed Behavior`)** và **Quyết định thiết kế mới trên kiến trúc của dự án đích (`Target Architecture Design`)**.
2. Lập kế hoạch ngắn gọn (Artifact Markdown) ánh xạ từng hành vi đã trích xuất (kèm `evidence_id`) sang các thành phần tương ứng của dự án mới, sau đó viết code tuân thủ `backend.md` / `frontend.md`.
3. **Kiểm chứng Theo Yêu cầu (On-Demand Verification)**: Mặc định chỉ chạy lệnh kiểm tra biên dịch nhanh 1 lần ở cuối tại thư mục dự án đích; chỉ viết và chạy scoped unit test / golden I/O test khi người dùng yêu cầu (tuân thủ `testing-mandatory.md`).

## Giai đoạn 4: Kiểm chứng Tương đương (`verify_reconstruction` & Diff Check)

1. Nếu cần so sánh giữa 2 phiên bản hoặc kiểm chứng bản dựng lại với đặc tả hữu hạn, sử dụng `compare_application_versions`, `compare_javascript_export_shapes`, `compare_functions`, hoặc `verify_reconstruction`.
2. Đóng toàn bộ phiên phân tích (`close_binary`) sau khi hoàn tất bàn giao.
