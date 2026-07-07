---
description: Triển khai viết mã nguồn hook hàm, vẽ overlay ImGui và tiêm thư viện.
---

# /mod-create - Mod Development Mode (External Only)

$ARGUMENTS

---

## 🔴 CRITICAL RULES
1. **EXTERNAL ARCHITECTURES ONLY:** You are strictly forbidden from writing code that compiles to a DLL injector, performs manual memory mapping, or hooks internal functions directly inside the game process memory.
2. **Defensive Programming:** Always check returned bytes from `ReadProcessMemory` and handle invalid/null pointers gracefully to prevent external tool crashes.
3. **Save Progress:** Call Mem0 `add_fact` after successfully writing a compiler-ready source file.

---

## Mod Development Workflow

### 1. Environment Setup
- Configure compilation files for external targets (`CMakeLists.txt` linking GLFW, OpenGL3, and Dear ImGui).
- Ensure project compiles to an independent Executable (`.exe`), not a Dynamic Library (`.dll`).

### 2. Core Logic Implementation
- **Reading Module:** Implement safe external process handle creation (`OpenProcess` with restricted permissions) and memory reading (`ReadProcessMemory`).
- **Logic Module:** Run calculation threads to process entity lists, and convert 3D coordinates using World-to-Screen math.
- **Overlay Module:** Construct the transparent window, set window display affinity (`WDA_EXCLUDEFROMCAPTURE`) to bypass capture detection, and render ESP bounding boxes.

### 3. Compilation & Verification
- Compile project, clear build cache, and verify executable output.
