# 🚀 Hướng Dẫn Cài Đặt Antigravity Agent Kit

Bộ kit này được thiết kế để nhúng trực tiếp vào thư mục gốc dự án của bạn (dưới dạng thư mục ẩn `.agents`). Điều này giúp AI Editor của bạn (Cursor, Windsurf, v.v.) có thể ngay lập tức đọc được luật và kỹ năng liên quan đến dự án.

## 📥 Cách Cài Đặt (Chỉ 1 lệnh duy nhất)

Mở Terminal tại thư mục gốc của dự án bạn muốn cài đặt, copy và chạy dòng lệnh sau:

```bash
git clone git@github.com:Kazdell/Antigravity-Skill-VNese.git .agents && rm -rf .agents/.git
```

*(Lưu ý: Bạn cần cấu hình SSH key trên Github để sử dụng lệnh clone này).*

### Lệnh này làm gì?
1. Clone bộ kit từ Github về máy bạn và tự động đổi tên thư mục tải về thành `.agents`.
2. Xóa đi file theo dõi `.git` gốc của repo bộ kit, giúp thư mục `.agents` hoàn toàn thuộc về Git Repository dự án hiện tại của bạn.

---

## ⚙️ Các Bước Tiếp Theo (Bắt buộc)

### 1. Ẩn khỏi Git của dự án (Tùy chọn)
Nếu bạn **KHÔNG** muốn đẩy bộ kit Agent này lên Git chung của team, hãy mở file `.gitignore` ở thư mục gốc dự án của bạn và thêm dòng sau vào:
```text
# Ignore Antigravity Agent Kit
.agents/
```
Nếu bạn **MUỐN** team của mình cũng dùng chung luật Agent này, đừng thêm vào `.gitignore` và hãy commit thư mục `.agents` lên repo bình thường.

### 2. Cài Đặt MCP (Model Context Protocol)
Sức mạnh của Antigravity v5.0 nằm ở trí nhớ và khả năng tương tác. Hãy bật file config MCP của Editor bạn đang dùng (VD: Cursor) và thêm 3 server sau:

**1. Trí nhớ hệ thống (Mem0 Local - 100% Offline):**
Để đạt hiệu năng cao nhất, không phụ thuộc vào Python và chạy hoàn toàn cục bộ, bạn nên sử dụng phiên bản thực thi Rust đã được biên dịch:

```json
"mem0": {
  "command": ".agents\\scripts\\mem0-rust\\mem0_rust_server.exe",
  "args": ["mcp"]
}
```

> [!TIP]
> **Đóng gói Độc lập (Standalone) - Chạy Offline 100% không cần Internet & Cài đặt:**
> Bạn có thể đóng gói file `mem0_rust_server.exe` để chạy trên bất kỳ máy Windows nào khác mà không cần cài đặt Rust/Python. Cách làm:
> 1. Copy file `mem0_rust_server.exe` vào thư mục mong muốn.
> 2. Tạo một thư mục con tên là `model` ngay bên cạnh file `.exe`.
> 3. Tải/Copy 3 tệp tin của mô hình `all-MiniLM-L6-v2` (`model.safetensors`, `config.json`, `tokenizer.json`) đặt vào thư mục `model` này.
> 4. Khi khởi chạy, server sẽ tự động nhận diện và sử dụng mô hình trong thư mục cục bộ mà không cần kết nối mạng hay tải thêm bất cứ gì.

*Tùy chọn phụ (Python Fallback):* Nếu bạn chưa biên dịch phiên bản Rust, bạn vẫn có thể sử dụng phiên bản Python cũ thông qua Qdrant:
```json
"mem0": {
  "command": "python",
  "args": ["<YOUR_USER_HOME>\\.gemini\\antigravity\\mcp_servers\\mem0\\mem0_mcp_server.py"]
}
```
*(Lưu ý: Thay thế `<YOUR_USER_HOME>` bằng đường dẫn thư mục cá nhân của bạn, ví dụ: `C:\\Users\\Tên_Của_Bạn`).*

**2. Giao tiếp mã nguồn (Github):**
```json
"github": {
  "command": "npx",
  "args": ["-y", "@modelcontextprotocol/server-github"],
  "env": {
    "GITHUB_PERSONAL_ACCESS_TOKEN": "<Thay_Token_Github_Của_Bạn_Vào_Đây>"
  }
}
```

**3. Test & Đánh giá (Genkit):**
```json
"genkit-mcp-server": {
  "command": "npx",
  "args": ["-y", "@genkit-ai/mcp-server"]
}
```

### 3. Xem & Quản Lý Ký Ức (Web Dashboard & Interactive CLI)

Để mở giao diện trực quan Glassmorphism quản lý, tìm kiếm hoặc thêm/xóa ký ức offline, bạn có hai phương án tiện lợi:

**Cách 1: Khởi chạy Web Dashboard trực tiếp**
Chạy từ thư mục gốc của dự án:
```powershell
.\.agents\scripts\mem0-rust\mem0_rust_server.exe dashboard
```
Hệ thống sẽ tự động khởi chạy máy chủ cục bộ và tự động mở trình duyệt tại địa chỉ `http://127.0.0.1:8899`.

**Cách 2: Chạy trực tiếp file thực thi (Menu Tương Tác)**
Khi bạn nhấp đúp chuột vào file `mem0_rust_server.exe` trên Windows Explorer hoặc chạy trực tiếp không truyền đối số trên Terminal:
```powershell
.\.agents\scripts\mem0-rust\mem0_rust_server.exe
```
Chương trình sẽ tự động kích hoạt **Menu Tương Tác** (Interactive CLI). Bạn chỉ cần nhập số tương ứng từ `0` đến `8` rồi ấn `Enter` để thực hiện tác vụ (chạy Dashboard, xem danh sách ký ức, tìm kiếm ngữ nghĩa, thêm/xóa ký ức) cực kỳ trực quan và không lo bị đóng cửa sổ đột ngột.

### 4. Khởi Động Dự Án Mới
Sau khi cài đặt xong, chỉ cần mở Editor lên, mở khung Chat và ra lệnh:
> *"Thức dậy đi Jarvis"*

AI sẽ tự động dò tìm file `.agents/README.md` (Mỏ neo) và bắt đầu phục vụ bạn! Chúc bạn code vui vẻ!
