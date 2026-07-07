---
description: Kiểm thử cách ly an toàn trên tài khoản phụ để phát hiện Anti-Cheat ban.
---

# /mod-test - Safe Isolation Testing Mode

$ARGUMENTS

---

## 🔴 CRITICAL RULES
1. **NO MAIN ACCOUNTS:** Testing must exclusively occur on burner/secondary accounts.
2. **VM Sandbox Isolation:** Run dynamic testing in isolated Virtual Machines or offline/test servers.
3. **Verification Checklist:** Follow the 3-Phase Isolation checklist to detect anti-cheat trigger vectors.

---

## Mod Testing Workflow

### 1. Pre-test Verification (OpSec Setup)
- Ensure game is connected via test account.
- Check firewall and network routes (if testing on private server).
- Verify VM isolation settings to avoid HWID mapping to host PC.

### 2. Execution of 3-Phase Isolation Testing
Run the test binary through three distinct execution configurations to isolate which module triggers detection:

```
[ Phase 1: Silent Read ]
     ├── Description: Read RAM dynamically. NO memory writes. NO overlay drawing.
     └── Check: Does account get flagged/banned?

[ Phase 2: Read + Draw Overlay ]
     ├── Description: Read RAM + render D3D transparent ImGui overlay. NO memory writes.
     └── Check: Does overlay rendering trigger window or graphics hooks detection?

[ Phase 3: Full Features (Read + Write + Draw) ]
     ├── Description: Activate memory patches, cheats, and writes.
     └── Check: Does code integrity check trigger ban?
```

### 3. Log Analysis
- Monitor process logs, event viewer, or local anti-cheat log files.
- Document exactly which phase triggered the flag in the testing summary.
