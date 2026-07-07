---
description: Phân tích lỗi crash hoặc sửa lỗi lệch offsets do game cập nhật (External Only).
---

# /mod-debug - Mod Debugging & Offset Maintenance Mode (External)

$ARGUMENTS

---

## 🔴 CRITICAL RULES
1. **Access Violation Prevention (0xC0000005):** Always validate pointer addresses before calling `ReadProcessMemory`/`WriteProcessMemory` dynamically. Reading address `0x0` or uncommitted memory pages causes instant external program crash.
2. **Defensive Handle Management:** Verify process handle validity (`hProcess != NULL` and `hProcess != INVALID_HANDLE_VALUE`) before executing memory operations. Handle invalidation occurs when the game restarts.
3. **Data Alignment Verification:** Ensure structures in C++ use proper packaging (e.g. `#pragma pack(push, 1)`) to avoid compiler padding offsets that mismatch actual RAM bytes.

---

## Mod Debugging Workflow

### 1. Crash Diagnostic & Exception Parsing
When the external tool or the game client crashes:
- Identify if the external tool threw `0xC0000005` (Access Violation). This indicates a bad offset calculation (reading wild pointers).
- Check if `ReadProcessMemory` returns `FALSE`. Call `GetLastError()` to verify the error code (e.g. `ERROR_PARTIAL_COPY` `299` indicates reading uncommitted memory pages).

### 2. Structure Misalignment Resolution
- Compare struct size in C++ compiler output with ReClass.NET class sizes.
- Verify byte alignment: Ensure padding arrays (e.g., `char pad_0018[8]`) match the exact distance between two known variables.

### 3. Automatic Offset Restoration
If the offsets break due to a game update:
- Run the **External SDK dumper** to generate new headers.
- If pattern matching fails, load the updated binary in **Ghidra**, locate the updated static base pointers, and generate a new Array of Bytes (AOB) signature.
- Update `offsets.hpp` or JSON configs.
