---
name: Stealth Memory Access & Anti-Cheat Bypass
description: Out-of-process memory manipulation (RPM/WPM), Direct Syscalls, Kernel Drivers (Ring 0), DMA hardware access, and stealth evasion of Anti-Cheat detection vectors.
---

# STEALTH MEMORY ACCESS & ANTI-CHEAT BYPASS

This skill provides advanced directives for reading and writing virtual memory in out-of-process execution, utilizing Direct Syscalls, Kernel Copy drivers, DMA hardware, and evading Anti-Cheat detection.

---

## 1. Process Handles & Memory Protections

Windows requires process handles to read/write memory. Requesting excessive access rights (like `PROCESS_ALL_ACCESS`) immediately triggers Anti-Cheat detection.

### Restricted Handle Acquisition (C++):
```cpp
// Request minimum rights needed for read/write
HANDLE hProcess = OpenProcess(PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION, FALSE, processId);
```

### Safe Memory Writing:
Verify page status via `VirtualQueryEx` to prevent calling uncommitted/protected regions, which throws `0xC0000005` (Access Violation) and crashes your tool.

---

## 2. Direct Syscalls (Bypassing User-Land API Hooks)

Anti-cheats hook standard APIs (e.g. `ReadProcessMemory` -> `NtReadVirtualMemory` in `ntdll.dll`). Calling the system call number (SSN) directly from custom assembly bypasses user-mode hooks.

### Dynamic SSN Resolution Concept:
1. Load `ntdll.dll` file statically from disk (`C:\Windows\System32\ntdll.dll`).
2. Parse its Export Address Table to locate functions (`NtReadVirtualMemory`).
3. Extract the System Service Number (SSN) dynamically to support Windows Updates.
4. Call the Syscall inside custom `.asm` stubs.

---

## 3. Kernel Mode & Hardware Evasion

### A. Kernel Drivers (Ring 0)
- Bypass Ring 3 handle checks by calling `MmCopyVirtualMemory` inside a driver.
- **BYOVD (Bring Your Own Vulnerable Driver):** Load vulnerable signed drivers (e.g., `gdrv.sys`, `mhyprot2.sys`) to read/write physical memory without registering a custom, unsigned driver.

### B. Direct Memory Access (DMA)
- Use hardware PCIe devices (screamer cards) to read/write physical memory from a secondary PC. This does not run any code on the host CPU and bypasses operating system memory mapping completely.

---

## 4. Anti-Cheat Detection Vectors & Evasion

### A. Handle Audits
*   *Detection:* Anti-Cheats scan the system handle table for processes referencing the game.
*   *Stealth:* Use **Handle Hijacking** (duplicating an existing handle from a trusted process like `lsass.exe` or Discord) or execute code fileless.

### B. Overlay Window Audits
*   *Detection:* Anti-Cheats scan for transparent, topmost window styles. Some take screenshots of the desktop.
*   *Stealth:* Call `SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE)` to make the window invisible to screenshot captures.

### C. Simulated Inputs
*   *Detection:* Anti-Cheats install low-level mouse hooks (`WH_MOUSE_LL`) and inspect the `LLMHF_INJECTED` flag.
*   *Stealth:* Inject raw inputs using custom kernel drivers (like Interception framework) and smooth aim movements using spline curves.
