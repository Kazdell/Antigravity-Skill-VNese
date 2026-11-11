---
name: audit
description: >-
  Kiểm tra chất lượng toàn diện trước khi bàn giao: thẩm định tính xác thực của
  Hồ sơ Bằng chứng (Evidence Provenance), kiểm tra Clean Code / Zero Defensive
  Redundancy trên code tái tạo, xác minh biên dịch và đóng sạch các phiên REA.
---

# /audit — Evidence Provenance & Reconstruction Quality Gate

Quy trình thẩm định toàn diện kết quả dịch ngược và mã nguồn tái tạo trước khi bàn giao.

## Quy trình 4 Bước Thẩm định

### Bước 1: Kiểm tra Phạm vi Thay đổi & Trạng thái Phiên REA
- Kiểm tra danh sách file vừa thêm/sửa (`git status --short` / `git diff --name-only`) và đọc trực tiếp bằng `read_file`.
- Với dự án lớn (`> 5` files nhiều mảng), có thể phối hợp qua Native Swarm (`swarm`) với các `verifier` độc lập soi song song Security, Logic/Contract và Clean Code.
- Kiểm tra đảm bảo mọi phiên phân tích nhị phân (`close_binary`) hoặc phiên hook động (`close_frida_session`, GDB/Rizin) đã được đóng sạch sẽ sau khi trích xuất đủ bằng chứng.

### Bước 2: Rà soát Evidence Provenance & Clean Code Checklist

#### A. Kỷ luật Bằng chứng Dịch ngược (REA Evidence Provenance)
- [ ] **Phân tách rõ `Observed` vs `Inferred` vs `Unknown`**: Không khẳng định giả mã decompiler là mã nguồn gốc hoặc sự tồn tại tĩnh chứng minh hành vi lúc chạy.
- [ ] **Trích dẫn `evidence_id` đầy đủ**: Mọi kết luận về giao thức, địa chỉ hàm (VA/RVA/file offset), hoặc cấu trúc dữ liệu đều gắn với `evidence_id` thực tế.

#### B. Chất lượng Code Tái tạo (Zero Defensive Redundancy & Idiomatic Code)
- [ ] **Không còn rác Decompiler**: Không còn tên biến vô nghĩa (`v1`, `sub_...`, `_0x...`) hoặc control flow rối từ giả mã.
- [ ] **CẤM Inline FQCN & Check thừa**: Đã khai báo `import`/`use`/`using` ở đầu file; không check null/rỗng lặp lại sau Boundary Validation.
- [ ] **Security & Secret**: Không hardcode khóa bí mật/token thật của môi trường live vào repo; nạp qua cấu hình bảo mật.

### Bước 3: Build & Scoped Verification Gate
Tại thư mục gốc của phân hệ đích (nếu có viết/sửa code tái tạo):
- Chạy lệnh kiểm tra biên dịch nhanh 1 lần — phải đạt `0 errors`.
- Nếu người dùng có yêu cầu chạy test hoặc `verify_reconstruction`, đảm bảo toàn bộ test mục tiêu đạt `PASS`.

### Bước 4: Báo cáo Kết quả Thẩm định
- **Session & Build Status**: Trạng thái đóng phiên REA (`CLEAN`) & trạng thái biên dịch (`PASS`/`FAIL`).
- **Phát hiện Vi phạm** (nếu có): Phân loại theo `[Blocker]` ➔ `[Warning]` ➔ `[Suggestion]` kèm link file và số dòng.
