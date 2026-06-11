# GUIDE — Cài đặt & Sử dụng CodeGraph + Semble

> **Dành cho**: Developers và AI Agent muốn hiểu sâu hơn về 2 công cụ này.

---

## 1. Tổng quan

| Công cụ | Mục đích | Công nghệ lõi | Token Saving |
|:---|:---|:---|:---|
| **Semble** | Tìm code theo từ khóa/ngữ nghĩa | Model2Vec + BM25 | ~98% |
| **CodeGraph** | Trace dependency & call graph | tree-sitter + SQLite | ~70-80% |

---

## 2. Cài đặt

### Cách 1: Script tự động (Khuyến nghị)

```powershell
# Mở PowerShell với quyền Administrator và chạy:
.\scripts\install-codesearch-tools.ps1
```

Script sẽ tự động:
- Kiểm tra Node.js và Python đã cài chưa
- Cài `@colbymchenry/codegraph` qua npm
- Cài `semble` qua pip
- Verify kết quả

### Cách 2: Cài thủ công

```powershell
# Yêu cầu: Node.js >= 18, Python >= 3.10

# 1. Cài CodeGraph
npm install -g @colbymchenry/codegraph

# 2. Cài Semble
pip install semble

# 3. Verify
codegraph --version
semble --version
```

---

## 3. Thiết lập CodeGraph cho Project mới

Mỗi project cần được index riêng một lần:

```powershell
# Chạy trong thư mục gốc của project
codegraph init -i

# Kiểm tra trạng thái
codegraph status

# Hoặc dùng script có sẵn:
.\scripts\init-codegraph.ps1 -ProjectPath "C:\path\to\project"
```

File index được lưu vào `.codegraph/` trong project (nên add vào `.gitignore`).

---

## 4. Semble — Hướng dẫn chi tiết

### Tìm kiếm đơn giản

```bash
# Tìm theo mô tả ngữ nghĩa
semble search "handle user login" ./src

# Tìm theo tên hàm/class
semble search "AuthService" .

# Giới hạn kết quả trả về
semble search "database connection pool" . --top-k 3
```

### Index trước để tìm nhanh hơn

```bash
# Tạo index (chỉ làm 1 lần hoặc khi code thay đổi nhiều)
semble index -o .semble-cache .

# Tìm kiếm dùng index đã tạo (nhanh hơn 10x)
semble search "payment webhook" --index .semble-cache
```

### Tìm trong GitHub repo (không cần clone)

```bash
semble search "transformer architecture" https://github.com/huggingface/transformers
```

---

## 5. CodeGraph — Hướng dẫn chi tiết

### Các lệnh chính

```bash
# Xem tất cả callers của một hàm
codegraph callers --symbol validateToken

# Xem tất cả callees (hàm này gọi những gì)
codegraph callees --symbol processPayment

# Dependency tree của một file
codegraph deps --file src/services/user.service.ts

# Impact analysis (BLAST RADIUS)
codegraph impact --symbol UserRepository

# Query tổng hợp về một symbol
codegraph query --symbol AuthMiddleware
```

### Tích hợp với Workflow "BLAST RADIUS"

Theo **Chỉ thị số 3** trong `rules/GEMINI.md` (BLAST RADIUS), trước khi sửa bất kỳ hàm/class nào:

```bash
# B1: Tìm chính xác file chứa code cần sửa
semble search "tên tính năng" .

# B2: Chạy BLAST RADIUS analysis
codegraph impact --symbol <TênHàmCầnSửa>

# B3: Báo cáo cho User danh sách file bị ảnh hưởng
# B4: Mới tiến hành sửa code
```

---

## 6. Cấu hình MCP Server (Nâng cao)

Để tích hợp Semble như một MCP Server cho các AI Agent khác:

```json
{
  "mcpServers": {
    "semble": {
      "command": "uvx",
      "args": ["--from", "semble[mcp]", "semble"]
    }
  }
}
```

Lưu vào:
- Claude Code: `~/.claude/mcp.json`
- Cursor: `.cursor/mcp.json`
- Gemini CLI/Antigravity: Đăng ký qua `agent_manager.py`

---

## 7. Troubleshooting

| Lỗi | Nguyên nhân | Giải pháp |
|:---|:---|:---|
| `codegraph: command not found` | npm global path chưa trong PATH | Chạy `npm config get prefix` và thêm vào PATH |
| `semble: command not found` | pip Scripts chưa trong PATH | Thêm `%APPDATA%\Python\Scripts` vào PATH |
| `codegraph status: no index found` | Chưa chạy `codegraph init` | Chạy `codegraph init -i` trong thư mục project |
| Semble trả về ít kết quả | Query quá cụ thể | Dùng từ khóa tổng quát hơn, hoặc tăng `--top-k` |
