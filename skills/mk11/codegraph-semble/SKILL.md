---
name: codegraph-semble
description: >
  Hybrid Code Search Protocol. Dùng Semble để tìm kiếm code theo từ khóa/ngữ nghĩa,
  và CodeGraph để trace dependency/call-graph. Thay thế grep_search và view_file mù quáng.
  Kích hoạt khi: tìm kiếm code, trace logic, phân tích impact của thay đổi.
triggers:
  - "tìm code"
  - "code nằm ở đâu"
  - "hàm này được gọi ở đâu"
  - "ai dùng function này"
  - "impact analysis"
  - "dependency"
  - "call graph"
  - "trace logic"
---

# MK11 SKILL — Hybrid Code Search (Semble + CodeGraph)

> **Mục tiêu**: Thay thế hoàn toàn `grep_search` + `view_file` thô bằng combo Semble (định vị) → CodeGraph (phân tích quan hệ) → `view_file` (đọc chi tiết có chủ đích). Tiết kiệm ~98% token trên codebase lớn.

---

## ⚠️ LUẬT BẮT BUỘC (MANDATORY RULES)

| ❌ CẤM LÀM | ✅ PHẢI LÀM |
|:---|:---|
| `grep_search` trên toàn bộ project | `semble search` để định vị trước |
| `view_file` cả file 500+ dòng | Dùng Semble lấy chunk → `view_file` đúng range |
| Quét nhiều folder để tìm logic | `codegraph query` để lấy call graph ngay |
| Đọc file import theo dây chuyền | `codegraph deps` để lấy dependency tree |

---

## 🧭 BẢNG QUYẾT ĐỊNH — Dùng Tool Nào?

```
Câu hỏi của Jarvis/User
        │
        ▼
┌─────────────────────────────────┐
│  Cần tìm CODE theo từ khóa      │──→ 🔵 SEMBLE
│  hoặc ngữ nghĩa?                │    semble search "<query>" <path>
└─────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────┐
│  Cần biết MỐI QUAN HỆ giữa      │──→ 🟠 CODEGRAPH
│  các hàm/class/module?          │    codegraph query --symbol <name>
└─────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────┐
│  Cần đọc CHI TIẾT đoạn code     │──→ 🟢 VIEW_FILE
│  đã biết chính xác vị trí?      │    view_file (StartLine, EndLine)
└─────────────────────────────────┘
```

---

## 🔵 SEMBLE — Tìm kiếm Code theo Ngữ nghĩa

**Dùng khi**: Tìm code theo từ khóa, tên tính năng, hoặc mô tả chức năng.

### Lệnh CLI

```bash
# Tìm kiếm cơ bản trong project
semble search "<query>" <project_path>

# Tìm trong project hiện tại
semble search "authentication flow" .

# Tìm và giới hạn số kết quả
semble search "payment processing" . --top-k 5

# Index trước để tìm nhiều lần (nhanh hơn)
semble index -o .semble-cache .
semble search "error handling" --index .semble-cache
```

### Output Semble trả về

```
File: src/auth/jwt.service.ts  (Lines 45-67)
Score: 0.94
Content:
  export async function verifyToken(token: string) {
    ...
  }
```

### Các use case điển hình

- `semble search "xử lý lỗi database" .` → Tìm error handling cho DB
- `semble search "stripe webhook" .` → Tìm code thanh toán
- `semble search "user permission check" .` → Tìm authorization logic
- `semble search "cache invalidation" .` → Tìm cache logic

---

## 🟠 CODEGRAPH — Trace Dependency & Call Graph

**Dùng khi**: Cần biết ai gọi hàm này, hàm này phụ thuộc vào gì, impact khi sửa.

### Thiết lập (chạy 1 lần/project)

```bash
# Khởi tạo index cho project
codegraph init -i

# Kiểm tra trạng thái index
codegraph status
```

### Lệnh truy vấn

```bash
# Xem call graph của một symbol (hàm/class)
codegraph query --symbol <symbol_name>

# Xem ai gọi hàm này (callers)
codegraph callers --symbol <function_name>

# Xem hàm này gọi những gì (callees)
codegraph callees --symbol <function_name>

# Phân tích dependency của một file
codegraph deps --file <file_path>

# Impact analysis: nếu sửa symbol này, ai bị ảnh hưởng?
codegraph impact --symbol <symbol_name>
```

### Các use case điển hình

- `codegraph callers --symbol validateUser` → Ai đang dùng `validateUser()`?
- `codegraph impact --symbol DatabaseConnection` → Sửa class này ảnh hưởng gì?
- `codegraph deps --file src/services/auth.ts` → File này import gì?

---

## 🔀 HYBRID WORKFLOW — Quy trình 3 bước chuẩn

```
BƯỚC 1: ĐỊNH VỊ (Semble)
━━━━━━━━━━━━━━━━━━━━━━━━
$ semble search "tên tính năng cần tìm" .
→ Nhận về: danh sách file + line range + code snippet

BƯỚC 2: PHÂN TÍCH QUAN HỆ (CodeGraph)  
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
$ codegraph callers --symbol <tên_hàm_tìm_được>
→ Nhận về: danh sách caller, dependencies

BƯỚC 3: ĐỌC CHI TIẾT (view_file có mục tiêu)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
view_file(path, StartLine=45, EndLine=67)
→ Chỉ đọc đúng đoạn cần thiết
```

### Ví dụ thực tế: "Tôi muốn sửa logic authentication"

```bash
# Bước 1: Tìm code auth
$ semble search "authentication login flow" .
→ Kết quả: src/auth/login.service.ts (Lines 23-89)

# Bước 2: Xem impact nếu sửa
$ codegraph impact --symbol LoginService
→ Kết quả: 3 controllers, 1 middleware bị ảnh hưởng

# Bước 3: Đọc chính xác
view_file("src/auth/login.service.ts", StartLine=23, EndLine=89)
view_file("src/api/auth.controller.ts", StartLine=10, EndLine=45)
```

**Kết quả**: Chỉ đọc ~110 dòng thực sự cần thiết thay vì scan toàn bộ project.

---

## 🟢 UNDERSTAND-ANYTHING — Trực quan hóa & Onboarding (Bổ trợ)

**Dùng khi**: Cần tạo bản đồ tri thức trực quan cho người dùng xem dưới dạng đồ thị (Visual Dashboard) hoặc khi bắt đầu một dự án mới hoàn toàn cần onboarding/guided tour để nắm cấu trúc tổng quan.

### BẢNG QUY ĐỊNH ƯU TIÊN (LẤY AGENT LÀM GỐC)
- **Ưu tiên 1 (Mặc định cho Agent):** Dùng `semble` và `codegraph` để phân tích logic nội bộ nhanh chóng qua văn bản dạng text nhằm tiết kiệm tối đa Token.
- **Ưu tiên 2 (Khi có yêu cầu hiển thị/onboard):** Dùng `understand-anything` để trực quan hóa kiến trúc hệ thống cho người dùng xem.

### Các lệnh sử dụng
```bash
# Quét dự án và phân tích cấu trúc đa tác nhân (chạy ở thư mục gốc)
understand

# Khởi chạy giao diện Web Dashboard tương tác trên trình duyệt cho User xem
understand-dashboard
```

---

## 📦 Cài đặt

```bash
# Chạy script cài đặt tổng hợp
.\scripts\install-codesearch-tools.ps1

# Hoặc cài thủ công
npm install -g @colbymchenry/codegraph
pip install semble
# Cài đặt CLI Understand-Anything
curl -fsSL https://understand-anything.com/install.sh | sh
```

## 🔗 References

- CodeGraph GitHub: https://github.com/colbymchenry/codegraph
- Semble GitHub: https://github.com/MinishLab/semble
- Understand-Anything GitHub: https://github.com/Lum1104/Understand-Anything
- Hướng dẫn chi tiết: `skills/mk11/codegraph-semble/GUIDE.md`
