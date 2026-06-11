# 🚀 Antigravity Agent Framework (v5.1) - README & MỎ NEO

> **Mục tiêu:** Xây dựng một môi trường làm việc thông minh, tự động hóa và điều phối các AI Agent thông qua **Tri-Part Linkage**, **Mem0 Memory**, và **Progressive Token Protocol (Giảm 97% Token tiêu thụ)**.

---

## ⚓ 1. MỎ NEO KÝ ỨC (MEM0 ANCHOR) - BẮT BUỘC ĐỌC

Nếu bạn là AI Agent vừa được khởi động vào một phiên làm việc mới, **BẮT BUỘC NGAY LẬP TỨC** thực hiện quy trình sau trước khi nói chuyện với User:

1. **Khôi phục ngữ cảnh (Progressive Disclosure):** Gọi công cụ `search_facts` hoặc `get_all_facts` của Mem0. Lưu ý: Không dump toàn bộ data, đọc có chọn lọc qua tìm kiếm ngữ nghĩa.
2. **Cấm suy đoán:** Tuyệt đối **KHÔNG** tự tiện đoán kiến trúc hệ thống hay tự quyết định tiến trình. Mọi thay đổi phải đi kèm **Graph-Based Impact Analysis** (Xây dựng đồ thị ảnh hưởng).
3. **Tri-Part Linkage:** Nạp các tiêu chuẩn chung bằng cách tham chiếu `.agents/.shared/SHARED_INDEX.md` and `.agents/skills/SKILL_INDEX.md` trước khi xử lý yêu cầu.
4. **Quy tắc Phản hồi (Greeting Protocol):** Mọi tin nhắn phản hồi của bạn bắt buộc phải bắt đầu bằng khối Intent, và ngay sau đó (dòng đầu tiên của nội dung phản hồi chính) bắt buộc phải chào bằng đúng 3 từ: **"Thưa chủ nhân,"**. Nếu thiếu, bạn đã vi phạm hiến pháp GEMINI.md.

---

## 📖 2. Giới thiệu (Introduction)

Antigravity v5.0 là một bộ khung (Framework) kết hợp **Prompt Engineering**, **Workflow Automation**, và **Knowledge Management**. Nó giúp biến AI Client/Editor của bạn (như Cursor, Claude Desktop, Windsurf, v.v.) thành một đội ngũ gồm 20+ chuyên gia phát triển phần mềm, hoạt động độc lập và phối hợp nhịp nhàng.

Thay vì dựa vào trí nhớ ngắn hạn của một cửa sổ chat, Antigravity sử dụng **Mem0 Local Memory** để nhớ ngữ cảnh dự án dài hạn và **Model Context Protocol (MCP)** để tương tác trực tiếp với các hệ thống bên ngoài.

---

## ⚙️ 3. Cấu trúc Hệ thống (Architecture Anatomy)

Toàn bộ "vũ khí" của hệ thống được đóng gói gọn gàng trong thư mục trung tâm `.agents/`:

| Thư mục / File | Phân loại | Mô tả chức năng cốt lõi |
| --- | --- | --- |
| `README.md` | **Mỏ Neo (Anchor)** | File hướng dẫn sử dụng và là điểm neo kích hoạt trí nhớ Mem0 cho AI. |
| `.agents/rules/` | **Hiến pháp** | Chứa `GEMINI.md` (Luật tối cao), `security.md`, v.v. Bất di bất dịch. Đảm bảo Agent không vượt rào. |
| `.agents/.shared/` | **DNA Dự án** | Các tiêu chuẩn chung về thiết kế, API, DB. Tra cứu mục lục tại `SHARED_INDEX.md`. |
| `.agents/skills/` | **Kho Kỹ năng** | 500+ kỹ năng cụ thể cho từng tác vụ (Bash, React, Docker...). Tra cứu tại `SKILL_INDEX.md`. |
| `.agents/agents/` | **Đội ngũ (MK)** | Các file định dạng 20+ Đặc nhiệm (VD: `mk01` Frontend, `mk02` Backend, `mk04` Security). |
| `.agents/workflows/` | **Chiến dịch** | Kịch bản tự động kích hoạt bằng Slash Command (`/plan`, `/create`, `/orchestrate`...). |

---

## 🛠️ 4. Hướng dẫn Cài đặt MCP (Installation Guide)

Sức mạnh của hệ thống phụ thuộc vào việc kết nối với môi trường thông qua MCP. Bạn **bắt buộc** phải cấu hình các MCP Server sau vào AI Client của mình (Ví dụ: Thêm vào file `claude_desktop_config.json`, cấu hình MCP của Cursor, hoặc tệp cấu hình tùy chỉnh).

### 🧠 A. Mem0 Local MCP (Trí nhớ hệ thống - 100% Offline)
Mem0 MCP đóng vai trò là não bộ dài hạn cục bộ (Knowledge & Memory Store) chạy in-process siêu nhẹ mà không cần Docker. Hệ thống lưu trữ lịch sử, cấu trúc thư mục, và bài học kinh nghiệm tại đây. Để đạt hiệu năng cao nhất và tối giản tài nguyên, ta sử dụng phiên bản thực thi Rust:

- **Cấu hình (Cline / Roo Code settings):**
  ```json
  "mem0": {
    "command": ".agents\\scripts\\mem0-rust\\mem0_rust_server.exe",
    "args": ["mcp"]
  }
  ```

> [!TIP]
> **Đóng gói Standalone & Tự động chạy Offline:**
> Bạn có thể sao chép tệp `mem0_rust_server.exe` sang máy khác mà không cần cài Rust/Python. Để chạy hoàn toàn offline không cần tải mô hình từ internet, chỉ cần tạo một thư mục con tên là `model` ngay bên cạnh file `.exe` này và copy 3 file mô hình `all-MiniLM-L6-v2` (`model.safetensors`, `config.json`, `tokenizer.json`) vào đó.

- **Tùy chọn phụ (Python Fallback):**
  ```json
  "mem0": {
    "command": "python",
    "args": ["<YOUR_USER_HOME>\\.gemini\\antigravity\\mcp_servers\\mem0\\mem0_mcp_server.py"]
  }
  ```
  *(Lưu ý: Thay thế `<YOUR_USER_HOME>` bằng đường dẫn thư mục cá nhân của bạn, ví dụ: `C:\\Users\\Tên_Của_Bạn`).*

- **Vai trò:** AI sẽ tự động gọi các lệnh như `search_facts` / `get_all_facts` để đọc bối cảnh khi bắt đầu, và `add_fact` để lưu lại bài học kiến trúc khi chốt task.

### 📊 B. Web Dashboard & Interactive CLI (Quản lý Ký Ức Trực Quan)
Để xem, quản lý, tìm kiếm, hoặc thêm/xóa ký ức offline, bạn có hai phương thức cực kỳ tiện lợi tích hợp sẵn trong file thực thi:

1. **Khởi chạy Web Dashboard:**
   Chạy lệnh sau từ thư mục gốc của dự án:
   ```powershell
   .\.agents\scripts\mem0-rust\mem0_rust_server.exe dashboard
   ```
   Hệ thống sẽ tự động khởi chạy máy chủ cục bộ và tự động mở trình duyệt tại địa chỉ `http://127.0.0.1:8899`.

2. **Chế độ Menu Tương Tác (Interactive Console Mode):**
   Khi bạn nhấp đúp trực tiếp vào file thực thi `mem0_rust_server.exe` trên Windows Explorer hoặc chạy trực tiếp không truyền tham số trên Terminal:
   ```powershell
   .\.agents\scripts\mem0-rust\mem0_rust_server.exe
   ```
   Chương trình sẽ tự động kích hoạt **Menu Tương Tác** trực quan. Bạn chỉ cần nhập số từ `0` đến `8` rồi ấn `Enter` để thực hiện nhanh các tác vụ (như khởi chạy Dashboard, xem danh sách ký ức, tìm kiếm vector, thêm/xóa fact) mà không cần phải nhớ các cú pháp dòng lệnh CLI phức tạp. Điều này giúp ngăn hiện tượng terminal mở lên rồi tự biến mất khi click đúp chuột.

### 🐙 B. GitHub MCP (Quản lý Source Code & Review)
Giúp AI có thể giao tiếp trực tiếp với kho lưu trữ (Repository) của bạn trên GitHub.

- **Cài đặt & Cấu hình:**
  ```json
  "github": {
    "command": "npx",
    "args": ["-y", "@modelcontextprotocol/server-github"],
    "env": {
      "GITHUB_PERSONAL_ACCESS_TOKEN": "<YOUR_TOKEN_HERE>"
    }
  }
  ```
- **Vai trò:** Cho phép AI tự động đọc/ghi Pull Request, thêm Comment, xem lịch sử Issue và rà soát các thay đổi code của team.

### 🤖 C. Genkit MCP (Tích hợp AI Testing & Flow)
Genkit cung cấp công cụ mạnh mẽ để phát triển, tracing và testing AI ứng dụng.

- **Cài đặt & Cấu hình:**
  ```json
  "genkit-mcp-server": {
    "command": "npx",
    "args": ["-y", "@genkit-ai/mcp-server"]
  }
  ```
- **Vai trò:** Cung cấp tài liệu và công cụ để AI có thể tự test AI Flows, Tracing hoặc Evaluate các module liên quan đến LLM trong dự án. Giúp AI hiểu và tương tác với Genkit Framework.

### 🧠 D. Token Savior & Code Review Graph (Advanced Context)
Nâng cấp khả năng điều hướng cấu trúc (Structural Navigation) và giảm tiêu thụ Token lên đến 97%.

- **Cài đặt môi trường:**
  ```bash
  pip install token-savior-recall code-review-graph
  ```
- **Cấu hình MCP:**
  ```json
  "token-savior-recall": {
    "command": "token-savior",
    "env": {
      "WORKSPACE_ROOTS": "<đường dẫn thư mục project>",
      "TOKEN_SAVIOR_CLIENT": "claude-code"
    }
  },
  "code-review-graph": {
    "command": "code-review-graph",
    "args": ["install"]
  }
  ```
- **Vai trò:** Ép AI điều hướng bằng Progressive Disclosure (khai mở 3 lớp), nhớ ngữ cảnh vĩnh viễn với Bayesian Decay và thiết lập biểu đồ ảnh hưởng.

---

## 🚀 5. Hướng dẫn Sử dụng (Usage Guide)

Cách tốt nhất để làm việc với Antigravity là ra lệnh thông qua **Slash Commands** (Quy trình Workflow). Hãy theo dõi 4 bước tiêu chuẩn sau:

### Bước 1: Khởi động & Lên Kế hoạch (Planning Mode)
- **Thời điểm:** Khi bắt đầu một ngày mới, một Task phức tạp hoặc một chức năng mới hoàn toàn.
- **Cách dùng:**
  ```text
  /plan Hãy thiết kế một tính năng quản lý giỏ hàng cho ứng dụng e-commerce
  ```
- **Kỳ vọng:** AI (Jarvis) sẽ đọc `README.md` này để nạp mỏ neo, truy vấn bộ nhớ Mem0, và sử dụng `mk12-system-architect` để sinh ra file kế hoạch chi tiết `docs/PLAN-*.md` thay vì vội vàng viết code.

### Bước 2: Điều phối Thực thi (Orchestration & Execution)
- **Thời điểm:** Sau khi kế hoạch (`/plan`) đã được bạn phê duyệt, hoặc khi bạn giao một task lớn cần nhiều Agent phối hợp.
- **Cách dùng:**
  ```text
  /orchestrate Thực thi kế hoạch trong file PLAN-ecommerce-cart.md
  ```
  *(Hoặc dùng `/create <tên tính năng>` nếu tính năng đơn giản)*
- **Kỳ vọng:** Jarvis sẽ tự động bóc tách task, gọi **ít nhất 3 MK Specialists** (ví dụ MK01, MK02, MK04), nạp đúng Kỹ năng từ `SKILL_INDEX.md` và mã gen từ `SHARED_INDEX.md` để triển khai.

### Bước 3: Đảm bảo Chất lượng & Kiểm thử (QA & Audit)
- **Thời điểm:** Khi code đã được viết xong nhưng cần kiểm tra lỗ hổng, lint, type-check, hoặc viết Test.
- **Cách dùng:**
  ```text
  /audit (Để kiểm tra bảo mật, convention và SEO)
  /test (Để viết Unit/E2E test theo chuẩn TDD)
  /debug (Nếu trong quá trình test hoặc chạy thử phát hiện ra lỗi)
  ```
- **Kỳ vọng:** Các chuyên gia MK04 (Security) hoặc MK05 (QA/Test) sẽ được triệu hồi để rà soát mã nguồn khắt khe trước khi cho phép Merge/Deploy.

### Bước 4: Lưu trữ Ký Ức & Đóng gói (Completion)
- **Thời điểm:** Bạn đã nghiệm thu code và chức năng chạy tốt, thông báo với AI là "Đã xong" (Done).
- **Cách dùng:**
  ```text
  Xong rồi. Hãy đóng gói task này và lưu bài học.
  ```
- **Kỳ vọng:** AI **BẮT BUỘC** dừng việc sinh code, thu thập các bài học/quyết định kiến trúc mới, gọi công cụ `add_fact` của Mem0 để nén tri thức và ghi chú vĩnh viễn. Điều này giúp AI "thông minh" hơn trong lần mở project sau.

### Bước 5: Khôi phục Hệ thống (Emergency Reset)
- **Thời điểm:** Bất cứ khi nào bạn thấy AI bắt đầu nói lảm nhảm, code sai chuẩn, hoặc bỏ qua việc check Blast Radius, báo cáo sai lệch.
- **Cách dùng:**
  ```text
  Thức dậy đi Jarvis
  ```
  *(Hoặc: Wake up Jarvis)*
- **Kỳ vọng:** Đây là **Lệnh Sinh Tử (Emergency Wake-up Protocol)**. Nó ép AI lập tức dừng mọi luồng suy nghĩ ảo giác, đọc lại từ đầu `README.md`, `GEMINI.md`, `SHARED_INDEX.md` và `SKILL_INDEX.md` để reset toàn bộ hệ thống chuẩn mực về trạng thái tỉnh táo nhất. Cực kỳ hiệu quả để "chỉnh đốn" lại Agent sau một đoạn chat dài.

---

## 🆘 6. Xử lý sự cố (Troubleshooting)

### Vấn đề 1: AI bị "Mất trí nhớ" (Amnesia) hoặc Code lung tung
Sau những cuộc trò chuyện quá dài hoặc task quá phức tạp, AI có thể quên mất quy tắc và bắt đầu đưa ra code rác hoặc không tuân thủ Tri-Part Linkage. 
**👉 Cách khắc phục:** Đừng đôi co với AI, hãy dùng ngay Lệnh Sinh Tử sau:
> **"Thức dậy đi Jarvis"**

*(Lệnh này ép AI lập tức bỏ qua mọi giả định, đọc lại toàn bộ `.agents/README.md`, `GEMINI.md`, và thiết lập lại chuẩn mực ban đầu mà không làm thêm bất kỳ điều gì khác).*

### Vấn đề 2: AI gọi Tool MCP thất bại
**👉 Cách khắc phục:**
1. Đảm bảo Node.js đã được cài đặt đúng cách và có sẵn trên biến môi trường `PATH`.
2. Kiểm tra log của AI Editor (VD: Output tab trong Cursor) để xem tiến trình MCP Server có bị crash không.
3. Nếu output của terminal quá dài làm AI bị quá tải context, hãy nhắc AI dùng công cụ proxy tiết kiệm token (nếu có cấu hình, ví dụ `rtk` - Rust Token Killer).
4. Nếu MCP của GitHub lỗi `Bad Credentials`, hãy check lại biến môi trường `GITHUB_PERSONAL_ACCESS_TOKEN`.

---
*Powered by Antigravity v5.0 - Xây dựng để không bao giờ lặp lại sai lầm.*
