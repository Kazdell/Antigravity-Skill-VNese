---
name: mem0
description: Long-term memory system using local Mem0 and Qdrant in-process database to store and retrieve user facts.
allowed-tools: add_fact, search_facts, get_all_facts, delete_fact, delete_all_facts
---

# Mem0 Long-Term Memory

> Hệ thống quản lý bộ nhớ dài hạn cục bộ (Offline) tích hợp trực tiếp qua MCP Server.

---

## 1. Tổng quan (Overview)

Hệ thống này giúp Agent ghi nhớ các sự kiện, sở thích, thông tin quan trọng hoặc cấu hình của người dùng qua các phiên làm việc khác nhau mà không cần gửi dữ liệu lên đám mây hay tốn token context window.

### Các công cụ hỗ trợ (MCP Tools):

| Tên công cụ | Tham số chính | Chức năng |
| :--- | :--- | :--- |
| `add_fact` | `fact: str`, `user_id: str` | Lưu một thông tin (sự thật) mới vào bộ nhớ. |
| `search_facts` | `query: str`, `user_id: str`, `limit: int` | Tìm kiếm ngữ nghĩa (semantic search) các thông tin liên quan. |
| `get_all_facts` | `user_id: str` | Liệt kê toàn bộ các sự thật đã được ghi nhớ của user. |
| `delete_fact` | `fact_id: str` | Xóa một sự thật cụ thể bằng ID. |
| `delete_all_facts` | `user_id: str` | Xóa sạch toàn bộ bộ nhớ của user. |

---

## 2. Quy trình trích xuất và lưu trữ Fact (Workflow)

Agent cần chủ động trích xuất các thông tin quan trọng từ cuộc hội thoại để lưu trữ.

### Nguyên tắc trích xuất Fact:
1. **Thông tin cô đọng**: Mỗi Fact chỉ nên chứa một thông tin duy nhất, rõ ràng (Ví dụ: *"Người dùng thích sử dụng Rust"* thay vì *"Người dùng thích sử dụng Rust vì nó nhanh và an toàn nhưng thỉnh thoảng cũng dùng Python"*).
2. **Loại bỏ thông tin thừa**: Không lưu trữ các câu chào hỏi, thông tin tạm thời hoặc cảm xúc nhất thời.
3. **Chuyển đổi ngôi thứ**: Luôn chuyển đổi câu nói của người dùng thành Fact ở ngôi thứ ba (Ví dụ: Người dùng nói *"Tôi đang dùng VS Code"* -> Fact: *"Người dùng đang sử dụng VS Code"*).

---

## 3. Cách sử dụng (Usage Examples)

### Thêm Fact mới:
Khi người dùng chia sẻ sở thích hoặc cấu hình hệ thống:
- Gọi `add_fact(fact="Người dùng sử dụng hệ điều hành Windows.", user_id="acer")`.

### Truy vấn thông tin trước khi thực hiện tác vụ:
Trước khi đưa ra các quyết định thiết kế hoặc lựa chọn công nghệ, hãy chủ động tìm kiếm:
- Gọi `search_facts(query="công nghệ ưa thích", user_id="acer")`.

### Xóa ký ức bị mâu thuẫn:
Khi người dùng cập nhật thông tin mới mâu thuẫn với thông tin cũ:
1. Tìm kiếm và lấy danh sách Fact liên quan.
2. Gọi `delete_fact(fact_id="...")` để xóa Fact cũ bị lỗi thời.
3. Gọi `add_fact(...)` để thêm Fact mới chính xác.
