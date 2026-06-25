---
trigger: always_on
---
# MANDATORY TESTING RULE (ENFORCED)

> **Mục tiêu**: Đảm bảo chất lượng hệ thống (Quality Assurance) mức cao nhất bằng Test-Driven Development và Regression Avoidance.

1. **Test-Driven / Coverage Mandatory**: Kể từ nay, MỖI KHI hệ thống thêm một tính năng mới (Feature), API mới hoặc một file logic mới, BẮT BUỘC toàn bộ dòng code đó phải đi kèm hoặc được cập nhật các Test Case tương ứng (Playwright E2E cho Frontend UI hoặc xUnit Integration/Unit Test cho Backend C#).
2. **Tuyệt đối không được Deploy/Hoàn Tất**: Không một module nào được phép báo cáo "Xong" (Done) nếu như kịch bản giả lập chưa chạy hoặc test bị FAIL.
3. **Tuân thủ Decision Tables**: Mọi Test Case sinh ra cần phải cover đủ nhánh đúng (Happy Path) và cụm nhánh sai (Edge Cases/Alternative Flows).
4. **Testcontainers Standard**: Backend bắt buộc sử dụng cơ chế Testcontainers tạo DB Ảo dựa trên Image của SQL Edge/Postgres để tách biệt môi trường Prod/Dev, nghiêm cấm trỏ Test vào DB thật của ứng dụng.
5. **Mandatory AI Code Review**: Trước khi kết thúc Task hoặc merge Pull Request, BẮT BUỘC phải thực hiện quy trình AI Code Review nội bộ (bằng cách chạy lệnh `/audit` hoặc tự rà soát theo kỹ năng `open-code-review`). Mọi lỗi nghiêm trọng về bảo mật, NPE hoặc rò rỉ tài nguyên được phát hiện bởi AI phải được khắc phục và xác nhận hoàn tất trước khi tiến hành chạy Unit Test/E2E cuối cùng.

---


