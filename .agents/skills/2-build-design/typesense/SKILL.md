---
name: typesense-search
description: Tra cứu tài liệu, quy chuẩn rules, workflows và skills của hệ thống qua Typesense Search Engine
---

# Kỹ năng Tra cứu tài liệu qua Typesense Search Engine

Kỹ năng này hướng dẫn AI Agent tra cứu nhanh chóng rules, skills, workflows và shared DNA trong thư mục `.agents/` mà không cần đọc toàn bộ thư mục hoặc chạy `grep_search` thô sơ.

## 🚀 Cách sử dụng

Khi cần tìm kiếm bất kỳ thông tin nào liên quan đến quy chuẩn code, cấu trúc DB, thiết kế UI, API standards, hay cách debug:
Chạy lệnh Python sau từ thư mục gốc của dự án:

```powershell
python .agents/scripts/typesense_indexer.py --search "<từ khóa tìm kiếm>"
```

### Ví dụ:
* Tra cứu về cách thiết kế API/cổng thanh toán:
  ```powershell
  python .agents/scripts/typesense_indexer.py --search "payment backend standard"
  ```
* Tra cứu về quy chuẩn font/màu sắc (UI/UX):
  ```powershell
  python .agents/scripts/typesense_indexer.py --search "Sapa theme colors"
  ```

## 🛠️ Đánh chỉ mục lại (Re-index)
Khi có sự thay đổi lớn hoặc thêm mới tệp trong `.agents/`, chạy lệnh sau để cập nhật dữ liệu lên Typesense:
```powershell
python .agents/scripts/typesense_indexer.py --index
```
