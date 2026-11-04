---
trigger: glob
glob: "**/*.{ts,tsx,js,jsx,html,vue,svelte,scss,css}"
---

# FRONTEND.MD — Frontend Clean Code & Zero-Waste Rules

## 1. ZERO-WASTE NAVIGATION & VERIFICATION
1. **Scoped Component Discovery**:
   - **CẤM** đọc các file cấu hình định tuyến tổng (`*.routes.*`, `router.*`) hoặc quét đệ quy vào `node_modules`, `dist`, `build`, `.next`, `.angular`.
   - Khi sửa màn hình/component cụ thể, dùng `Select-String` tìm theo mã màn hình, selector, hoặc i18n key trong thư mục nguồn (`src/`) rồi gọi `read_file` trực tiếp vào các file của component đó.
2. **No Unsolicited Build / Typecheck**:
   - **CẤM** tự ý chạy lệnh build/typecheck (`npx.cmd tsc`, `npm run build`, `ng build`, `next build`) sau khi sửa file Frontend khi dev server / live watcher đang chạy, trừ khi người dùng yêu cầu tường minh.

## 2. ZERO DEFENSIVE REDUNDANCY & SHARED UTILITIES (DRY)
1. **Trust Shared Form Controls & API Contracts**:
   - Tin tưởng cơ chế đồng bộ giá trị/trạng thái của các shared form controls và cấu trúc dữ liệu từ API response chuẩn.
   - **Single Validation Point**: Thực hiện validate dữ liệu tập trung **1 lần duy nhất** tại hàm `validate()` / `onSubmit()` dùng chung trước khi gửi request; cấm rải check điều kiện trùng lặp ở nhiều sự kiện UI trung gian.
2. **Reuse Shared User / Permission & Format Helpers**:
   - Bắt buộc tái sử dụng các hàm utility kiểm tra người dùng hiện tại, quyền thao tác (`canEdit`, `canApprove`, actor check), định dạng ngày tháng/tiền tệ và xử lý tải file sẵn có trong bộ `shared/utils` hoặc `core` của dự án.
   - **CẤM** nhân bản logic parse token/user info hoặc viết lại hàm kiểm tra quyền cục bộ trong từng component.
3. **Clean Reactive State & Resource Cleanup**:
   - Làm phẳng logic xử lý dữ liệu và tránh lồng ghép `if-else` / ternary nhiều tầng trong cả code logic lẫn template.
   - Quản lý vòng đời bất đồng bộ an toàn (hủy subscription/listener/timer khi component unmount; tránh lồng callback/subscribe nhiều cấp).

## 3. SURGICAL TEMPLATE & COMPONENT EDITS
1. **Minimal Diff (`CRLF` Safe)**:
   - Dùng `edit_file` chỉ sửa đúng block UI/logic mục tiêu; không tự ý format lại toàn bộ file template, không đổi tên props/input/output đang hoạt động ổn định và không xóa comment nghiệp vụ sẵn có.
2. **i18n & Constants Consistency**:
   - Không hardcode chuỗi thông báo/nhãn giao diện nếu dự án đang dùng i18n; tái sử dụng các hằng số trạng thái/enum dùng chung giữa các màn hình cùng nghiệp vụ.
