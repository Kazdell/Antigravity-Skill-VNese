---
description: Phân tích cấu trúc game, bóc tách metadata và lập kế hoạch tìm offsets.
---

# /mod-plan - Game Modding Planning Mode

$ARGUMENTS

---

## 🔴 CRITICAL RULES
1. **NO CODE WRITING** - This workflow is for static/dynamic analysis and plan formulation only.
2. **EXTERNAL PARADIGM ONLY:** Do not plan any internal hooks, DLL injections, or memory patches. The plan must only support reading/writing memory externally (via RPM/WPM) and drawing overlays in a separate transparent process.
3. **Tri-Part Linkage:** Prior to formulating the plan, you must load the specific modding skills (`Memory_Modding_Fundamentals`, `Game_Mathematics_W2S`, `System_Authority_Bypass`, `Memory_Offsets_Management`, `Game_Reverse_Engineering_FOSS`, `Anti_Cheat_Detection_Vectors`).
4. **OpSec First:** Never target live production databases or primary user accounts. Always plan for isolated sandbox testing.

---

## Modding Planning Workflow

### 1. Engine Detection & Static Decompilation
- Identify target game engine (Unity Mono/IL2CPP, Unreal Engine 4/5, Source 2).
- Import target executable or main assembly to **Ghidra** for static decompilation and class mapping.
- Locate static offsets or entry points for player pointers and camera view matrices.

### 2. Offsets & AOB Signature Hunting Strategy
- Define pattern scanning signatures (AOB) to bypass ASLR without direct hook operations.
- Map the classes using **ReClass.NET** dynamically to verify byte padding (alignment).

### 3. Anti-Cheat & Detection Vector Assessment
- Reference `Anti_Cheat_Detection_Vectors` to evaluate handle protection risks.
- Decide stealth mitigation strategies (e.g. screenshot exclusion `WDA_EXCLUDEFROMCAPTURE`, target process hijacking, or kernel driver deployment).

### 4. Output Plan Generation
Create a modding plan under `docs/PLAN-{game-name}.md` outlining:
- Targeted memory offsets.
- Method of access (External RPM/WPM).
- Overlay drawing method (External transparent borderless window).
