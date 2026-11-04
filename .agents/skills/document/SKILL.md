---
name: document
description: >-
  Viết và cập nhật tài liệu kỹ thuật cho phân hệ: Doc comments (Javadoc /
  Rustdoc / XML Docs / Docstrings) cho Service/Utility, OpenAPI/Swagger cho
  Controller/Handler, và tài liệu kiến trúc / nghiệp vụ Markdown.
---

# /document — Technical Documentation & API Specs

Đồng bộ tài liệu kỹ thuật và đặc tả API với mã nguồn thực tế của phân hệ.

## Quy tắc Thực hiện

1. **Doc Comments cho Service / Domain / Utility**:
   - Viết tài liệu hàm rõ ràng (`///` trong Rust/C#, `/** ... */` trong Java/TS, `"""..."""` trong Python) cho các hàm nghiệp vụ phức tạp và hàm helper dùng chung.
   - Giữ nguyên mọi comment/docstring hợp lệ đang có trong file.
2. **OpenAPI / Schema Annotations trên API Handler**:
   - Khai báo mô tả endpoint và mã phản hồi trực tiếp trên Controller/Handler theo đúng quy ước của dự án (`@Operation`, `#[utoipa::path]`, `[ProducesResponseType]`, FastAPI `summary`/`responses`).
3. **Tài liệu Kiến trúc & Nghiệp vụ**:
   - Dùng `read_file` đọc mã nguồn thực tế trước khi viết tài liệu vào thư mục tài liệu của dự án để đảm bảo tên bảng, tên trường DTO và mã trạng thái khớp 100% với code.
