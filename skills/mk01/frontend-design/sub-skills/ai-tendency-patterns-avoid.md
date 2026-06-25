# ❌ AI Tendency Patterns & AI Slop (AVOID!)

Bộ chỉ dẫn phòng ngừa các thói quen thiết kế rập khuôn, lười biếng và thiếu thẩm mỹ mà các mô hình AI (như GPT, Gemini, Claude) thường tự động áp dụng. Agent bắt buộc phải kiểm tra và tự tránh các patterns này ngay khi viết mã nguồn giao diện.

---

## 🎨 1. AI Color & Visual Slop (Lỗi màu sắc & Hiệu ứng)

### [AI-01] Dải màu Gradient Tím-Hồng Mặc Định (Purple/Pink Gradient Slop)
- **Anti-pattern:** Sử dụng dải màu gradient từ tím sang hồng/xanh dương (`from-purple-500 to-pink-500`, v.v.) cho các nút chính, tiêu đề Hero section, hoặc viền trang trí. Đây là thiết kế phản xạ của AI làm cho mọi website trông giống nhau.
- **Quy tắc (Purple Ban):** Cấm tự ý dùng màu tím/hồng trừ khi file [DESIGN.md](file:///c:/Users/ACER/Downloads/Antigravity-Skill-VNese/DESIGN.md) của dự án ghi rõ đây là màu thương hiệu chính thức (Primary brand color). Hãy sử dụng các màu trung tính được pha nhẹ (tinted neutrals) hoặc màu thương hiệu thực tế của người dùng.

### [AI-02] Bóng sáng Neon trên Nền tối (Dark Glow)
- **Anti-pattern:** Thêm các shadow mờ có màu sắc sặc sỡ (như xanh ngọc, tím, hồng) đè lên các box trên nền tối (`dark-glow`) để tạo cảm giác "cyberpunk". Điều này làm loãng độ tương phản và gây nhiễu thị giác nghiêm trọng.
- **Quy tắc:** Trên giao diện nền tối, shadow phải có màu đen hoặc xám siêu tối với độ mờ nhẹ (blur thấp, opacity dưới 15%), tuyệt đối không dùng glow có màu sắc rực rỡ trừ khi đó là trạng thái active/hover có chủ đích rõ rệt.

### [AI-03] Màu Nền Kem mặc định (Cream Palette)
- **Anti-pattern:** Sử dụng màu kem/off-white ấm nhạt ($R \ge G \ge B$ và màu rất sáng) làm màu nền mặc định cho mọi trang sáng để tạo cảm giác "sang trọng".
- **Quy tắc:** Hãy thiết lập hệ thống màu nền linh hoạt dựa trên bối cảnh sản phẩm (ví dụ: dùng màu xám trung tính lạnh hoặc trắng tinh khiết có sắc độ tương phản cao cho Product/SaaS để giảm mệt mỏi mắt).

### [AI-04] Dải sọc trang trí lặp lại (Repeating Stripes)
- **Anti-pattern:** Dùng `repeating-linear-gradient` để tạo các dải sọc chéo trang trí làm hình nền hoặc dải phân cách.
- **Quy tắc:** Tránh các chi tiết trang trí thừa thãi không mang lại giá trị chức năng. Sử dụng khoảng trắng và đường phân cách mỏng màu xám nhạt (`#E2E8F0` / `border-slate-200`) để chia bố cục.

---

## 📐 2. AI Layout & Spacing Slop (Lỗi bố cục & Khoảng cách)

### [AI-05] Thẻ lồng Thẻ quá mức (Nested Cards Slop)
- **Anti-pattern:** AI có thói quen đóng khung mọi nhóm thông tin bằng thẻ Card, dẫn đến tình trạng Card lồng trong Card (`nested-cards`) 3-4 cấp liên tiếp.
- **Quy tắc:** Hạn chế tối đa việc lồng ghép card. Hãy chia nhóm bằng khoảng trắng (whitespaces), đường kẻ phân cách mỏng (`divider`), hoặc thay đổi màu nền nhẹ (`bg-slate-50`) để phân tách khu vực thông tin một cách thông thoáng.

### [AI-06] Layout Bento rập khuôn (Bento Grid Overuse)
- **Anti-pattern:** Tự động chia trang Landing Page hoặc một form điền đơn giản thành hệ lưới Bento (Bento grid) với các ô kích thước lộn xộn.
- **Quy tắc:** Grid Bento chỉ phù hợp để làm Dashboard hiển thị số liệu hoặc phần giới thiệu tính năng có độ dài khác nhau. Tránh lạm dụng cho các trang tin tức, bài viết, hoặc luồng đăng ký/đăng nhập tuyến tính.

### [AI-07] Khoảng cách chật chội (Cramped Padding)
- **Anti-pattern:** Thiết kế các card/container chứa nhiều chữ nhưng khoảng đệm (`padding`) hai bên và trên dưới quá nhỏ (dưới 12px), làm chữ đè sát mép viền.
- **Quy tắc:** Padding tối thiểu của một card phải là **24px** (`p-6` trong Tailwind) đối với desktop và **16px** (`p-4`) đối với mobile để tạo không gian thở cho giao diện.

### [AI-08] Khoảng cách đơn điệu (Monotonous Spacing)
- **Anti-pattern:** Dùng chung một chỉ số spacing (ví dụ: mọi khoảng cách đều là `gap-4` hoặc `margin-bottom: 16px`) cho tất cả các cấp độ phần tử từ khoảng cách giữa các chữ, các đoạn, đến các section lớn.
- **Quy tắc:** Phải tuân thủ nguyên lý Gestalt về khoảng cách: Khoảng cách giữa các khối lớn (section) phải lớn hơn khoảng cách giữa các phần tử nhỏ bên trong khối (ví dụ: section gap = 48px, component gap = 24px, element gap = 12px).

---

## 🛠️ 3. AI Component & Interaction Slop (Lỗi chi tiết & Tương tác)

### [AI-09] Viền siêu mỏng kèm Bóng đổ siêu rộng (Thin Border + Wide Shadow)
- **Anti-pattern:** Vẽ các khối card có border mỏng 1px đi kèm shadow nhòe cực lớn (`box-shadow` có blur $\ge 16\text{px}$ và độ lan rộng cao). Đây là phong cách thiết kế Vercel clone được AI copy rập khuôn.
- **Quy tắc:** Nếu dùng viền mỏng 1px, shadow phải cực kỳ tinh tế (blur dưới 8px và opacity màu bóng dưới 5%), hoặc bỏ hẳn border khi dùng shadow để tránh trùng lặp viền.

### [AI-10] Tự động phóng to hình ảnh khi Hover (Image Hover Transform)
- **Anti-pattern:** Tự động thêm hiệu ứng phóng to hoặc xoay nghiêng hình ảnh khi hover chuột (`hover:scale-105`, `hover:rotate-2`) cho mọi thẻ `<img>` trên trang.
- **Quy tắc:** Chỉ thêm hiệu ứng hover cho các hình ảnh là nút bấm hành động hoặc link dẫn sang trang khác. Tránh các chuyển động thừa thãi trên hình ảnh minh họa tĩnh.

### [AI-11] Chuyển động giật cục (Bounce Easing Slop)
- **Anti-pattern:** Dùng các hiệu ứng chuyển động có easing dạng bounce/elastic giật cục (`transition-timing-function: cubic-bezier(...)` kiểu bounce lố) cho các dropdown menu hoặc modal pop-up.
- **Quy tắc:** Sử dụng easing mượt mà, chuẩn mực (như `ease-in-out` hoặc `cubic-bezier(0.4, 0, 0.2, 1)`) với thời gian chuyển động (duration) lý tưởng từ 150ms đến 300ms.

---

## ✍️ 4. AI Copy & Copywriting Slop (Lỗi từ ngữ tiếp thị sáo rỗng)

### [AI-12] Cụm từ "X Theater" hoặc thuật ngữ sáo rỗng (Theater Slop)
- **Anti-pattern:** Sử dụng các từ tiếp thị rập khuôn của AI như "Delightful experience", "Seamless integration", "Unlock the power", "Seamlessly manage", hoặc đặt tên theo kiểu `"X theater"` (ví dụ: "Collaboration theater").
- **Quy tắc:** Viết nội dung giao diện (copywriting) trực diện, ngắn gọn, dùng động từ mạnh và tập trung vào lợi ích cụ thể của người dùng (ví dụ: Thay vì *"Unlock seamless collaboration"*, hãy viết *"Mời thành viên và cùng chỉnh sửa mã nguồn theo thời gian thực"*).