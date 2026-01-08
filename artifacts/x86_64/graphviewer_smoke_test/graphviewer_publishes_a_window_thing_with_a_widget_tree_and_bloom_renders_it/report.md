# Scenario: graphviewer publishes a Window Thing with a widget tree and Bloom renders it

**Architecture**: `x86_64`  
**Feature**: `GraphViewer smoke test`

[Back to Index](../../../../index.md)

## Steps Summary

| Index | Step | Status | Artifacts |
| :---: | --- | :---: | --- |
| 0 | an app "graphviewer" is started as a user task | ✅ |  |
| 1 | "graphviewer" creates a Window Thing sized 520x340 titled "GraphViewer" | ✅ |  |
| 2 | "graphviewer" creates a Layout Thing of kind "Column" with padding 16 and gap 12 | ✅ |  |
| 3 | "graphviewer" creates a Label Thing with text "Welcome. Press the button..." | ✅ |  |
| 4 | "graphviewer" creates a Button Thing with text "Show current graph" | ✅ |  |
| 5 | "graphviewer" links the Layout as the Window content root | ✅ |  |
| 6 | "graphviewer" links the widgets as children of the Layout | ✅ |  |
| 7 | "graphviewer" commits a new Frame Thing for the Window | ✅ |  |
| 8 | Bloom must observe the Window Thing and its widget tree | ✅ |  |
| 9 | Bloom must render the Window rectangle with rounded corners | ✅ |  |
| 10 | Bloom must render the Label and Button positioned by the Layout | ✅ |  |

## Execution Details

### 1. an app "graphviewer" is started as a user task ✅

---

### 2. "graphviewer" creates a Window Thing sized 520x340 titled "GraphViewer" ✅

---

### 3. "graphviewer" creates a Layout Thing of kind "Column" with padding 16 and gap 12 ✅

---

### 4. "graphviewer" creates a Label Thing with text "Welcome. Press the button..." ✅

---

### 5. "graphviewer" creates a Button Thing with text "Show current graph" ✅

---

### 6. "graphviewer" links the Layout as the Window content root ✅

---

### 7. "graphviewer" links the widgets as children of the Layout ✅

---

### 8. "graphviewer" commits a new Frame Thing for the Window ✅

---

### 9. Bloom must observe the Window Thing and its widget tree ✅

---

### 10. Bloom must render the Window rectangle with rounded corners ✅

---

### 11. Bloom must render the Label and Button positioned by the Layout ✅

---

