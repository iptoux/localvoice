# Learning: Scrollbar im transparenten Pill-Fenster (Windows)

**Milestone:** MS-01
**Symptom:** Frameless/transparentes Tauri-Fenster zeigt trotzdem eine native Scrollbar am rechten Rand.

---

## Ursache

Der eingebettete Webview erbt das Standard-Browser-Verhalten: `html` und `body` haben keinen `overflow`-Wert gesetzt. Sobald Inhalt auch nur 1 px überläuft (z. B. durch Padding, Rounding oder Subpixel-Rendering), erscheint der native Scrollbar.

## Fix

```css
/* src/index.css */
html,
body {
  margin: 0;
  padding: 0;
  overflow: hidden;
  background: transparent;
}
```

Zusätzlich am Root-`<div>` in `index.html`:

```html
<div id="root" style="overflow:hidden;width:100%;height:100%"></div>
```

## Regel

> Bei jedem Tauri-Fenster ohne Dekoration oder mit transparentem Hintergrund immer `overflow: hidden` auf `html` **und** `body` setzen. Sonst zeigt Windows den nativen Scrollbar auch dann, wenn kein sichtbarer Überlauf vorhanden ist.
