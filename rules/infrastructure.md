---
trigger: glob
glob: "**/*.{yaml,yml,dockerfile,tf,sh,ps1,bat,ini,conf,toml}"
---

# INFRASTRUCTURE.MD - DevOps & System Guardrails

> **Mục tiêu**: Chuẩn hóa cấu hình, quản trị hạ tầng, deployment và xử lý terminal an toàn.

---

## ️ 1. INFRASTRUCTURE AS CODE (IaC)

1. **Immutable Infrastructure**: Mọi cấu hình hạ tầng phải được định nghĩa bằng code (Terraform, YAML).
2. **Secrets Management**: Tuyệt đối không hardcode DB password hay cloud credentials trong các file config. Truyền qua Environment Variables.
3. **Portability**: Các scripts (`sh`, `ps1`) phải chạy được trên nhiều môi trường mà không hardcode đường dẫn tuyệt đối.

---

##  2. DOCKER & K8S STANDARDS

1. **Least Privilege**: Chạy container dưới quyền `non-root` user.
2. **Image Optimization**: Sử dụng Alpine hoặc distroless base images để giảm kích thước.
3. **Multi-stage Builds**: Tách biệt môi trường build và môi trường runtime.

---


