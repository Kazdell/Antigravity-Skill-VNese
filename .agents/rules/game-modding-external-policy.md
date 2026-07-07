---
trigger: model_decision
description: "When the user asks to reverse engineer, analyze anti-cheat, search memory offsets, or develop external game modding tools."
---

# GAME MODDING EXTERNAL POLICY (100% OUT-OF-PROCESS ENFORCEMENT)

- **Rule 1: Zero Internal Footprint**
  - Never generate, plan, or suggest DLL injectors, manual mapping code, virtual method table (VMT) hooks, or inline bytecode memory patches inside the target game process virtual address space.
- **Rule 2: Out-of-Process Memory Manipulation**
  - All memory reading/writing must be executed externally using Win32 API functions (`ReadProcessMemory` / `WriteProcessMemory` with page verification via `VirtualQueryEx`), kernel-mode memory copy drivers, or DMA hardware-level access.
- **Rule 3: Overlay Isolation**
  - Graphical HUDs or ESP menus must be rendered inside a completely independent transparent click-through window (e.g., GLFW/OpenGL3 overlay), with display affinity configured to bypass capture (`SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE)`).
