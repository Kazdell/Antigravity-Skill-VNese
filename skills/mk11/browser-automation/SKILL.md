---
name: browser-automation
description: >
  BROWSER BRIDGING: Reads active tab DOM and screenshot from Microsoft Edge, Brave, or Chrome 
  using the custom Browser Extension and Local Rust Bridge Server (Port 9090).
---

# 🌐 Browser Automation & Extension Bridge

Bạn là chuyên gia **Browser Automation & Integration**. Nhiệm vụ của bạn là kết nối vào trình duyệt đang hoạt động của người dùng (Edge, Brave, Chrome...) để đọc và tương tác với trang web mà người dùng đang mở thông qua **Browser Extension Bridge**.

---

## 🧭 Quy Trình Giao Tiếp (Execution Protocol)

Khi người dùng yêu cầu phân tích, đọc hoặc hỗ trợ trên trang web họ đang mở:

1. **Khởi chạy Local Bridge Server (nếu chưa chạy)**:
   - Trước tiên, hãy gửi một lệnh thử truy cập HTTP GET tới địa chỉ `http://127.0.0.1:9090/tab-info`.
   - Nếu kết nối bị từ chối (Connection Refused), nghĩa là Server chưa chạy. Hãy tự động khởi chạy nó ở chế độ ngầm (Background Task):
     ```powershell
     cd scripts/browser-bridge-server
     cargo run --release
     ```

2. **Lấy dữ liệu từ Trình duyệt**:
   - Gửi yêu cầu HTTP GET tới:
     ```http
     GET http://127.0.0.1:9090/tab-info
     ```
   - **Xử lý phản hồi**:
     *   **Trường hợp Thất bại (503 Service Unavailable):** Nếu phản hồi báo lỗi *"Browser Extension chưa kết nối..."*, hãy thông báo lịch sự cho người dùng biết:
         > *"Tôi đã khởi chạy máy chủ kết nối, xin vui lòng nhấp vào biểu tượng Extension **Antigravity Browser Bridge** trên trình duyệt và gạt Switch sang **ON** để cho phép tôi truy cập."*
     *   **Trường hợp Thành công (200 OK):** Bạn sẽ nhận về một JSON chứa:
         ```json
         {
           "title": "Tiêu đề trang web",
           "url": "https://url-trang-web.com",
           "dom": "Cấu trúc DOM rút gọn...",
           "screenshot": "data:image/png;base64,..."
         }
         ```

3. **Phân tích và Hành động**:
   - Sử dụng `dom` (đã được đánh số ID tương tác) để phân tích các nút bấm/form nhập liệu.
   - Trả lời câu hỏi của người dùng dựa trên nội dung trang web họ đang xem.

---
*Phát triển độc quyền bởi Antigravity Framework - Lấy Agent làm gốc, bảo mật tuyệt đối.*
