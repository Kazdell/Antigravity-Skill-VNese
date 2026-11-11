# Quy tắc Điều phối Đa Tác Tử (Unified 3-Tier & Native Swarm Rules)

> Tài liệu chuẩn hóa cơ chế phối hợp nhóm giữa Jarvis (Coordinator), Subagent Scouts và hệ sinh thái **Built-in Native Swarm** (`swarm`, `swarm_monitor`, `swarm_narrator`).

## 1. Phễu Điều Phối 3 Tầng (3-Tier Dispatch)
1. **Tầng 0 — Direct Scalpel (`<= 5` files, 1 phân hệ)**: Jarvis xử lý trực tiếp bằng các lệnh đọc/sửa song song. Không khởi tạo agent phụ để tối ưu tốc độ và tiết kiệm Quota.
2. **Tầng 1 — Parallel Scouts (`invoke_subagent`)**: Dùng cho 1–3 nhánh tra cứu/khảo sát Read-Only độc lập một chiều, thu tóm tắt gọn về Jarvis.
3. **Tầng 2 — Native Swarm (`swarm` Built-in / `/orchestrate`)**: Dùng cho tác vụ `> 5` files, đa phân hệ (BE + FE + DB), dịch ngược đa tầng, hoặc cần kiểm chéo độc lập (`Author <-> @verifier`).

## 2. Kỷ Luật Vận Hành Native Swarm
- **Contract-First Brief**: Bài giao việc đầu tiên trên bảng tin phải khóa rõ phạm vi file sở hữu, vùng cấm chạm, hợp đồng giao tiếp (`Boundary Contract` / `evidence_id`), và tiêu chí hoàn thành.
- **Cô lập Workspace**: Dùng `workspace: "share"` cho các Author sửa code/build đồng thời; dùng `workspace: "inherit"` cho các worker chỉ đọc và `verifier`.
- **Giao tiếp qua `#seq` & `#channels`**: Chỉ tham chiếu chéo bằng mã bài `#seq` trên bảng tin (cấm copy-paste dài dòng); chia kênh riêng khi nhóm `> 6` workers; chỉ `@mention` đúng người cần hành động tiếp theo vì mỗi `@mention` sẽ đánh thức 1 lượt chạy.
- **Giám sát Tự động**: Thêm worker `monitor` (`swarm_monitor`) canh kẹt tự động mỗi 3 phút và `narrator` (`swarm_narrator`) duy trì Overview khi khả dụng.
