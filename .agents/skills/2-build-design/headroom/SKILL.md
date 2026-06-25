---
name: headroom-token-saver
description: Nén log và output của các lệnh terminal thông qua Headroom wrapper (run.py) để tiết kiệm 40% - 90% tokens
---

# Kỹ năng Tiết kiệm Token qua Headroom Wrapper

Kỹ năng này hướng dẫn AI Agent chạy các lệnh terminal lớn (ví dụ: chạy tests, build logs, migration, directory audit) qua wrapper `run.py` để tự động lọc và nén output trước khi gửi về cho AI Client.

## 🚀 Cách sử dụng

Khi cần chạy bất kỳ lệnh terminal nào dự kiến có đầu ra lớn (như build, test, audit, git status, git log...):
Thay vì gọi trực tiếp lệnh, hãy bọc nó qua script `run.py` bằng Python:

```powershell
python .agents/scripts/run.py "<lệnh cần chạy>"
```

### Ví dụ:
* Chạy verify dự án:
  ```powershell
  python .agents/scripts/run.py "python .agents/scripts/verify_all.py ."
  ```
* Chạy test suite:
  ```powershell
  python .agents/scripts/run.py "dotnet test"
  ```
* Xem log git dài:
  ```powershell
  python .agents/scripts/run.py "git log -n 50"
  ```

## ⚙️ Cấu hình Ngưỡng giới hạn Token
Có thể điều chỉnh giới hạn độ dài ký tự mong muốn (mặc định là 3000 ký tự) bằng tham số `--token-limit`:
```powershell
python .agents/scripts/run.py "npm run build" --token-limit 2000
```
