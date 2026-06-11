# 🗺️ PLAN: Cải tiến Reader (Giai đoạn 3 & 4)

Tài liệu này tổng hợp các hạng mục nâng cấp trải nghiệm đọc truyện trên hệ thống MLNDEX, đặc biệt lấy cảm hứng từ các nền tảng đọc truyện chuẩn doanh nghiệp (Enterprise) như MangaDex, Webnovel.

---

## 🚀 GIAI ĐOẠN 3: ĐÃ HOÀN THÀNH (Lưu trữ cấu hình & Hướng đọc)
- Cấu hình đọc (Chế độ cuộn, Màu nền) đã được đồng bộ hóa với `localStorage`.
- Tính năng **Hướng đọc (Reading Direction)**: LTR/RTL đã áp dụng linh hoạt cho nút chuyển trang và chuyển chương. Tự ẩn ở chế độ "Cuộn dọc".

---

## 🎯 GIAI ĐOẠN 4: ĐANG TRIỂN KHAI (Triệt để Bug UX & Bổ sung tính năng)

Dành cho **Cả Truyện Tranh & Truyện Chữ**:
- [x] **Lỗi 7 tiếng của Comment**: Backend trả về thời gian UTC (VD: `2026-03-20T02:00:00`) nhưng bị mất ký hiệu `Z` hoặc Frontend không parse đúng về múi giờ Việt Nam. Dẫn đến bình luận vừa đăng thì ghi là "7 giờ trước".
  - *Giải pháp*: Cập nhật `CommentSection.jsx`, nối thêm `'Z'` nếu thiếu trước khi `new Date()` hoặc dùng `date-fns`.

Dành riêng cho **Truyện Chữ (Novel)** (Ưu tiên cao nhất):
- [x] **Tùy chỉnh Cỡ chữ (Font Size) & Phông chữ (Font Family)**:
  - Cung cấp ít nhất 3 loại Phông Chữ (Serif, Sans-Serif, Monospace).
  - Tùy chỉnh tăng giảm Cỡ chữ từ `text-sm` đến `text-3xl`.
  - Tích hợp thêm Căn lề / Khoảng cách dòng.
- [x] **Chạm để ẩn/hiện Menu (Tap to Toggle UI)**:
  - Hiện tại ở truyện tranh có vùng `w-[40%]` ở giữa màn hình để chạm vào toggles toolbar. Trong phần *Truyện chữ* đang bị thiếu vùng chạm này. Cần bổ sung để ấn 1 chạm màn hình mở Menu.

Dành riêng cho **Truyện Tranh (Manga/Comic)** (Ưu tiên tiếp theo):
- [ ] **Trang Đôi (Double Page Spread)**: Chế độ hiển thị 2 trang cạnh nhau trên các màn hình lớn, kèm theo các tùy chọn giữ lề hoặc cắt lề hở.
- [ ] **Phóng to Ảnh (Double tap to Zoom)**: Cải tiến trải nghiệm mobile cho phép user tap 2 lần để phóng to khung tranh chứa chữ nhỏ.

---

## 5. Phân công đặc nhiệm (Agent Assignments)
- **🤖 MK01 (@Frontend Specialist)**: Đảm nhận trực tiếp luồng update UI này. Thao tác hoàn chỉnh tại `CommentSection.jsx` và `ChapterViewer.jsx`. Nắm chắc quy chuẩn thiết kế Component mở rộng và CSS của Tailwind.
- **🤖 MK05 (@Quality & Audit)**: Verify liên tục các thay đổi trên cả mobile viewport để đảm bảo không rớt UX sau khi gắn thêm logic.
