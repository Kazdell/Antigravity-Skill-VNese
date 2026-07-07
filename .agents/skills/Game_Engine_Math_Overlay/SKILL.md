---
name: Game Engine Architecture & Overlay Mathematics
description: Core architecture concepts of Unity/Unreal Engine, View Matrix math, World-to-Screen (W2S) algorithms, and GLFW transparent click-through overlay.
---

# GAME ENGINE ARCHITECTURE & OVERLAY MATHEMATICS (EXTERNAL)

This skill covers the spatial transformations, native game loops, and independent overlay rendering techniques used to display visual information (such as ESP bounding boxes) on the game screen from out-of-process.

---

## 1. Game Loop & Lifecycle Execution

Understanding engine lifecycles is critical for knowing when variables (like player coordinates) update in RAM.

### Unity (C# / MonoBehaviour) Lifecycle:
```
Awake() ──► Start() ──► FixedUpdate() (Physics) ──► Update() (Logic) ──► LateUpdate() (Camera)
```
- **LateUpdate Crossover:** Perfect for camera matrix extraction, as camera position updates here after player movement.

### Unreal Engine (C++ / AActor) Lifecycle:
```
PostActorCreated() ──► BeginPlay() ──► Tick(float DeltaSeconds) ──► EndPlay()
```

### Entity Component System (ECS) vs Object-Oriented (OOP):
- **OOP (Traditional):** Data is contiguous per object (e.g. `PlayerObject -> [Health, Position, Team]`).
- **ECS (Modern):** Components are isolated in large arrays by type (e.g. `PositionArray` stores positions for all Entities in contiguous memory).

---

## 2. World-to-Screen (W2S) Transformation

To draw a 2D bounding box around an entity in a 3D environment, the entity's 3D coordinates must be multiplied by the camera's **View Matrix** read dynamically from the game's memory.

### W2S Algorithm (C++):
```cpp
struct Vector3 { float x, y, z; };
struct Vector2 { float x, y; };
struct Matrix4x4 { float m[4][4]; };

bool WorldToScreen(Vector3 pos, Vector2& screen, Matrix4x4 matrix, int screenWidth, int screenHeight) {
    float clipX = pos.x * matrix.m[0][0] + pos.y * matrix.m[0][1] + pos.z * matrix.m[0][2] + matrix.m[0][3];
    float clipY = pos.x * matrix.m[1][0] + pos.y * matrix.m[1][1] + pos.z * matrix.m[1][2] + matrix.m[1][3];
    float clipZ = pos.x * matrix.m[2][0] + pos.y * matrix.m[2][1] + pos.z * matrix.m[2][2] + matrix.m[2][3];
    float clipW = pos.x * matrix.m[3][0] + pos.y * matrix.m[3][1] + pos.z * matrix.m[3][2] + matrix.m[3][3];

    // Clipping check: if object is behind camera, discard
    if (clipW < 0.1f) return false;

    // Perspective division
    Vector3 ndc = { clipX / clipW, clipY / clipW, clipZ / clipW };

    // Convert NDC to screen pixel coordinates
    screen.x = (screenWidth / 2.0f) + (ndc.x * screenWidth / 2.0f);
    screen.y = (screenHeight / 2.0f) - (ndc.y * screenHeight / 2.0f);
    return true;
}
```

---

## 3. External Overlay Window using GLFW & OpenGL3 (C++)

To render without injecting drawing hooks, establish a transparent borderless click-through window.

### Setup Transparent Window:
```cpp
#include <GLFW/glfw3.h>
#define GLFW_EXPOSE_NATIVE_WIN32
#include <GLFW/glfw3native.h>

void SetupTransparentWindow(GLFWwindow* window) {
    HWND hwnd = glfwGetWin32Window(window);
    // Transparent, layered, topmost, click-through styles
    SetWindowLongPtr(hwnd, GWL_EXSTYLE, WS_EX_TOPMOST | WS_EX_TRANSPARENT | WS_EX_LAYERED);
    SetLayeredWindowAttributes(hwnd, RGB(0, 0, 0), 0, LWA_COLORKEY);
    // Hide window from screenshots / game capture tools
    SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE);
}
```

### Rendering Loop:
```cpp
void RenderLoop(GLFWwindow* window) {
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        glClearColor(0.0f, 0.0f, 0.0f, 0.0f); // Clear to black (makes transparent)
        glClear(GL_COLOR_BUFFER_BIT);

        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui::NewFrame();

        // Draw external ESP overlay
        ImDrawList* drawList = ImGui::GetBackgroundDrawList();
        drawList->AddRect(ImVec2(100, 100), ImVec2(200, 200), IM_COL32(0, 255, 0, 255), 0.0f, 0, 2.0f);

        ImGui::Render();
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        glfwSwapBuffers(window);
    }
}
```
