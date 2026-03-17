# Learning: data-tauri-drag-region funktioniert nicht auf Windows

**Milestone:** MS-01
**Symptom:** Pill-Fenster lässt sich auf Windows nicht verschieben, obwohl `data-tauri-drag-region` gesetzt ist.

---

## Ursachen

### 1. Fehlende Capability `core:window:allow-start-dragging`

`data-tauri-drag-region` ruft intern `plugin:window|start_dragging` via `invoke()` auf.
In Tauri v2 gilt das Capability-/ACL-System: jede invoke-Aktion muss explizit freigegeben sein.
`core:default` enthält diese Permission **nicht**.

**Fix:**

```json
// src-tauri/capabilities/default.json
"permissions": [
  "core:default",
  "core:window:allow-start-dragging",
  ...
]
```

### 2. Kind-Elemente schlucken das mousedown-Event (Windows-spezifisch)

Tauri's injiziertes Drag-Skript hört auf `mousedown` am `document` und prüft,
ob das Event-Target selbst oder ein Vorfahre `data-tauri-drag-region` trägt.
Auf Windows kann das Bubbling in bestimmten Konstellationen (transparentes Fenster,
Child-Events) unterbrochen werden, bevor Tauri's Listener greift.

**Fix:** `data-tauri-drag-region` auch auf alle direkten Kind-Elemente im Drag-Bereich setzen:

```tsx
<div data-tauri-drag-region>
  <div data-tauri-drag-region /* Icon */ />
  <span data-tauri-drag-region>Label</span>
</div>
```

---

## Regel

> Bei jedem frameless/transparenten Fenster mit Drag-Region:
> 1. `core:window:allow-start-dragging` in die Capability aufnehmen.
> 2. `data-tauri-drag-region` auf **alle** sichtbaren Kind-Elemente im Drag-Bereich setzen,
>    nicht nur auf den äußeren Container.
