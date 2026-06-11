---
version: alpha
name: Antigravity
description: Visual Identity and Design System for Antigravity Framework
colors:
  primary: "#1A1C1E"       # Deep ink for text & titles
  secondary: "#6C7278"     # Slate for borders and metadata
  tertiary: "#B8422E"      # Boston Clay for highlight & call to actions
  neutral: "#F7F5F2"       # Warm limestone for background
  on-primary: "#FFFFFF"
  on-tertiary: "#FFFFFF"
typography:
  h1:
    fontFamily: Public Sans
    fontSize: 3rem
    fontWeight: 700
  h2:
    fontFamily: Public Sans
    fontSize: 2.25rem
    fontWeight: 600
  body-md:
    fontFamily: Public Sans
    fontSize: 1rem
    lineHeight: 1.5
  label-caps:
    fontFamily: Space Grotesk
    fontSize: 0.75rem
    fontWeight: 500
rounded:
  sm: 4px
  md: 8px
  lg: 16px
spacing:
  sm: 8px
  md: 16px
  lg: 32px
components:
  button-primary:
    backgroundColor: "{colors.tertiary}"
    textColor: "{colors.on-tertiary}"
    rounded: "{rounded.md}"
    padding: "12px 24px"
  card:
    backgroundColor: "{colors.neutral}"
    rounded: "{rounded.lg}"
    padding: "{spacing.lg}"
---

## Overview

Hệ thống thiết kế của Antigravity kết hợp giữa phong cách **Tối giản Kiến trúc (Architectural Minimalism)** và **Sự tinh tế Hiện đại (Modern Elegance)**. Giao diện được tối ưu hóa để mang lại cảm giác cao cấp, nhẹ nhàng với các tone màu tự nhiên và khoảng trắng (whitespace) rộng rãi.

## Colors

Bảng màu được bắt rễ từ các màu trung tính có độ tương phản cao và một màu nhấn duy nhất:
- **Primary (#1A1C1E):** Màu mực đậm dùng cho tiêu đề chính và nội dung quan trọng.
- **Secondary (#6C7278):** Màu đá phiến dùng cho viền, chú thích, metadata.
- **Tertiary (#B8422E):** Boston Clay - màu đất sét nhấn đỏ gạch, đóng vai trò điều hướng tương tác chính.
- **Neutral (#F7F5F2):** Màu đá vôi ấm làm nền tảng cho giao diện, dịu mắt hơn màu trắng tinh thông thường.

## Typography

Sử dụng hai họ phông chữ chính:
- **Public Sans:** Dùng cho toàn bộ hệ thống tiêu đề (H1, H2) và văn bản chính, mang lại cảm giác chuyên nghiệp, rõ ràng.
- **Space Grotesk:** Dùng cho các thẻ nhãn (labels), chỉ mục (indices) hoặc số liệu, mang hơi hướng công nghệ hiện đại.

## Layout

Hệ thống Spacing tuân thủ nghiêm ngặt bội số của 8px:
- Khoảng cách nhỏ (sm - 8px) dùng cho các phần tử kề nhau.
- Khoảng cách vừa (md - 16px) dùng làm khoảng đệm cho các component.
- Khoảng cách lớn (lg - 32px) dùng làm margin giữa các section lớn hoặc padding của Card lớn.

## Shapes

Các phần tử giao diện được bo góc mềm mại để tăng tính hiện đại và thân thiện:
- Bo góc nhỏ (sm - 4px) cho checkbox, thẻ tag nhỏ.
- Bo góc vừa (md - 8px) cho button, input fields.
- Bo góc lớn (lg - 16px) cho cards, modals, banners.

## Components

### Button Primary
Nút tương tác chính sử dụng màu nhấn đỏ đất sét `tertiary`, chữ màu trắng `on-tertiary` và bo góc vừa `rounded.md`.

### Card
Các khối nội dung sử dụng nền màu đá vôi ấm `neutral`, bo góc lớn `rounded.lg` và khoảng đệm bên trong lớn `spacing.lg`.
