---
trigger: glob
glob: "**/*.{js,jsx,ts,tsx,css,scss,html,vue,svelte,dart,swift,kt,xml}"
---

# FRONTEND.MD - Client-Side Mastery

> **Mục tiêu**: Quản lý thống nhất Giao diện Web & Mobile. Một nguồn chân lý cho trải nghiệm người dùng.

---

##  1. PREMIUM UX/UI

1. **Design Spec Anchoring (DESIGN.md)**:
   - AI **BẮT BUỘC** phải đọc và phân tích file `DESIGN.md` tại thư mục gốc của dự án trước khi viết hoặc chỉnh sửa bất kỳ mã giao diện nào (HTML/CSS/JS/Tailwind/React/Vue...).
   - CẤM TUYỆT ĐỐI việc tự ý sử dụng mã màu hex, phông chữ hoặc kích thước bo góc nằm ngoài danh sách Design Tokens đã được quy định trong đặc tả.
2. **Aesthetics**: Tuân thủ hệ màu phối hợp (như limestone, slate, Boston clay) và typography chuẩn (Public Sans, Space Grotesk) được quy định trong spec. Whitespace là sang trọng.
3. **Spacing & Shapes**: Hệ thống spacing tuân thủ bội số của 8px (`spacing.sm`, `spacing.md`, `spacing.lg`). Độ bo góc sử dụng đúng tỷ lệ (`rounded.sm`, `rounded.md`, `rounded.lg`).
4. **Feedback**: Mọi tương tác (Click, Tap) đều phải có phản hồi thị giác ngay lập tức.

---

##  2. MOBILE & RESPONSIVE

1. **Touch Targets**: Button tối thiểu 44x44px (Chuẩn ngón tay cái).
2. **Safe Areas**: Tôn trọng tai thỏ (Notch) và Home Indicator trên iOS/Android.
3. **Mobile-First**: Code CSS cho mobile trước, override cho PC sau.

---

##  3. PERFORMANCE DOMAIN

1. **Core Web Vitals**: LCP < 2.5s, CLS < 0.1, FID < 100ms.
2. **Optimistic UI**: Cập nhật giao diện TRƯỚC khi API trả về (Zalo/Facebook style).
3. **Asset Optimization**: Ảnh WebP, Video lazy-load.

---

## ️ 4. STATE & COMPONENT

1. **Atomic Design**: Component nhỏ, tái sử dụng cao (`<Button />`, `<INPUT />`).
2. **State**: Server State (TanStack Query) !== Client State (Zustand/Context). Tách biệt rõ ràng.

---


