---
name: Java Swing & Javassist Modding
description: Hướng dẫn kỹ thuật dịch ngược, đóng gói, vá class Java Swing an toàn và thiết kế overlay menu.
---

# KỸ THUẬT MODDING GAME JAVA SWING & JAVASSIST

## 1. Tránh vá trực tiếp Class UI / Class khổng lồ
*   **Không vá trực tiếp:** Cấm vá trực tiếp các class JFrame/UI chính (như `Project.class`) hoặc các class quá nặng chứa nhiều class ẩn phức tạp.
*   **Giải pháp:** Viết một lớp Launcher trung gian (ví dụ: `ModLauncher.java`) để chạy mã mod (như bật menu cheat) trước, sau đó dùng phản xạ hoặc gọi trực tiếp `GameMain.main(args)`.
*   **Chạy gián tiếp:** Sửa đường dẫn thực thi trong file `.bat` hoặc `.sh` trỏ vào Launcher đó để giữ file class gốc nguyên bản 100%.

## 2. Quản lý mã nguồn Decompile
*   **Tránh lỗi biên dịch:** Các file mã nguồn dịch ngược (từ CFR, Fernflower...) thường dùng định dạng mã hóa khác hoặc chứa ký tự đặc biệt (`\u0000`). Nếu để trong thư mục build, `javac` sẽ tự động quét và báo lỗi biên dịch.
*   **Giải pháp:** Luôn lưu file dịch ngược dưới đuôi `.txt` thay vì `.java`, hoặc cất chúng ở một thư mục độc lập ngoài tầm quét của trình biên dịch `javac`.

## 3. Kỹ thuật thiết kế Menu Overlay ẩn trong Swing
*   **Phím tắt toàn cục:** Sử dụng `KeyEventDispatcher` đăng ký với `KeyboardFocusManager` để lắng nghe phím nóng (ví dụ phím nóng `) trên toàn cục mà không phụ thuộc vào trạng thái Focus của các ô nhập liệu trong game.
*   **Modeless Dialog:** Sử dụng `JDialog` với thuộc tính `ModalityType.MODELESS` để làm menu overlay. Tránh dùng `JPopupMenu` cho các menu phức tạp hoặc `Modal Dialog` vì sẽ chặn tương tác chuột với màn hình game chính.
