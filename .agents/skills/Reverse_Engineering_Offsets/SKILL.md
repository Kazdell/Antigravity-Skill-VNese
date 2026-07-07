---
name: Reverse Engineering & Memory Offsets Management
description: Static/dynamic analysis using Ghidra, x64dbg, and ReClass.NET, runtime External Pattern Scanning (AOB), and automated SDK dumping.
---

# REVERSE ENGINEERING & MEMORY OFFSETS MANAGEMENT

This skill documents static and dynamic analysis workflows using Free and Open-Source Software (FOSS) to inspect game binaries, reconstruct structural layouts, and automate offset scanning.

---

## 1. Selected FOSS Tools & Methodologies

1.  **Ghidra (Static Analysis & Decompilation):**
    *   Import executable files to analyze control flow. Search for unique string references to locate static base pointers.
2.  **x64dbg (Dynamic Debugging):**
    *   Attach to running processes and set **Hardware Breakpoints** on writes to trace what instruction updates specific player offsets (e.g. `mov [rcx + 0x48], eax` means the health offset is `0x48`).
3.  **ReClass.NET (Structure Modeling):**
    *   Map raw memory pointers dynamically into C++ class definitions. Pad unused fields (e.g. `char pad_0000[16]`) to align offsets accurately.

---

## 2. Runtime Pattern Scanning (AOB - Array of Bytes)

To prevent code breakdown after minor game updates, use signature scanning to locate functions dynamically at runtime:

### C++ External Pattern Scanner:
```cpp
#include <windows.h>
#include <vector>

bool CompareByteArray(const BYTE* pData, const BYTE* bMask, const char* szMask) {
    for (; *szMask; ++szMask, ++pData, ++bMask) {
        if (*szMask == 'x' && *pData != *bMask) return false;
    }
    return (*szMask) == NULL;
}

// Scans target process memory remotely using a local read buffer
uintptr_t FindPatternExternal(HANDLE hProcess, uintptr_t startAddress, DWORD size, const BYTE* signature, const char* mask) {
    std::vector<BYTE> buffer(size);
    SIZE_t bytesRead;

    if (!ReadProcessMemory(hProcess, (LPCVOID)startAddress, buffer.data(), size, &bytesRead)) {
        return 0;
    }

    DWORD maskLen = strlen(mask);
    for (DWORD i = 0; i < bytesRead - maskLen; i++) {
        if (CompareByteArray(&buffer[i], signature, mask)) {
            return startAddress + i;
        }
    }
    return 0;
}
```

---

## 3. Automated SDK Dumping (External Frameworks)

Instead of searching addresses manually, dumpers parse structures directly from memory:

### A. Unreal Engine SDK Dumpers (External UEDumper)
*   **Mechanism:** Locates global symbols `GUObjectArray` and `GNames` in the game's virtual memory space remotely via `ReadProcessMemory`. It walks the internal object lists and automatically outputs C++ header files (`.h`) containing class offsets and structs.
*   **Usage:** Run the external dumper executable, pass the game PID, retrieve the generated SDK folder, and include it directly in your project.

### B. Unity IL2CPP Dumper (Il2CppDumper)
*   **Mechanism:** Parses binary files `GameAssembly.dll` (or `libil2cpp.so`) and runtime metadata `global-metadata.dat` to reconstruct class names, fields, function definitions, and their respective offsets.
*   **Usage:** Generates C# class structures (`dump.cs`) and Python scripts for auto-naming symbols inside disassemblers (IDA Pro / Ghidra).
