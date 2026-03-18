# MS-20 — Accessibility, Multi-language & Final Polish

**Goal:** Add keyboard navigation and screen reader support, expand language support beyond DE/EN, and apply final polish for a v1.0 public release.
**Depends on:** MS-19
**Status:** `todo`

---

## Engineering Tasks

### Accessibility

- [ ] TASK-269: Accessibility audit — ensure all interactive elements have `aria-label`, `role`, and `tabIndex`; all pages are navigable via Tab/Shift+Tab; focus indicators are visible in both themes
- [ ] TASK-270: Screen reader support — add `aria-live` regions for dynamic content (pill state changes, toast notifications, transcription results); test with NVDA on Windows, VoiceOver on macOS
- [ ] TASK-271: High contrast mode — detect `prefers-contrast: more` media query; apply high-contrast color tokens; ensure minimum 4.5:1 contrast ratio for all text
- [ ] TASK-272: Keyboard shortcuts for main window — `Ctrl+1` through `Ctrl+5` for sidebar navigation (Dashboard, History, Dictionary, Models, Settings); `Ctrl+R` to start/stop recording; `Escape` to close dialogs; document in Settings page

### Multi-language

- [ ] TASK-273: Multi-language model support — update `models/registry.rs` to include French (FR), Spanish (ES), Italian (IT), Portuguese (PT), and "auto-detect" entries; map each to the appropriate whisper model flags
- [ ] TASK-274: Update `transcription/language.rs` — add language codes for FR, ES, IT, PT; add "auto" option that passes `--language auto` to whisper-cli for language auto-detection
- [ ] TASK-275: React: Language selector update — expand language dropdown in Settings and ExpandedPill to include all supported languages plus "Auto-detect" option
- [ ] TASK-276: DB migration 7 — update default language settings to accept expanded language codes; add `transcription.auto_detect_language` setting (default: false)

### Final Polish

- [ ] TASK-277: React: Onboarding flow v2 — improved with: language selection step, model recommendation based on chosen language, microphone test step with live audio level preview, shortcut customization step
- [ ] TASK-278: React: Empty states for all list views — Dashboard (no sessions yet), History (no results for filter), Dictionary (no entries yet), Models (none installed), Logs (no entries); each with illustration and actionable CTA
- [ ] TASK-279: Final UX polish pass — consistent spacing, alignment, and typography across all pages; ensure all loading states use `Spinner.tsx`; ensure all error states are consistent; review all toast/notification messages
- [ ] TASK-280: Write `docs/user/` documentation — getting-started.md, recording.md, history.md, dictionary.md, models.md, settings.md with screenshots
- [ ] TASK-281: Write `docs/dev/` documentation — architecture.md, database-schema.md, tauri-commands.md, transcription-pipeline.md, contributing.md
- [ ] TASK-282: Update `docs/changelog.md` with entries for v0.2, v0.3, v0.4, v1.0
- [ ] TASK-283: Update `README.md` — final v1.0 content: feature list, screenshots, installation instructions per platform, build from source instructions, contributing guide link
- [ ] TASK-284: Execute comprehensive smoke test checklist — full end-to-end testing on Windows, macOS, and Linux; document results; fix P0/P1 issues
- [ ] TASK-285: Tag and release v1.0 — create GitHub release with changelog, signed installers for all platforms, and release notes

## QA / Acceptance

- [ ] TASK-285a: Verify all pages are fully keyboard-navigable without mouse
- [ ] TASK-285b: Verify NVDA reads pill state changes and transcription results correctly
- [ ] TASK-285c: Verify French and Spanish transcription produce reasonable results with multilingual models
- [ ] TASK-285d: Verify auto-detect language correctly identifies DE, EN, FR in test clips
- [ ] TASK-285e: Verify onboarding v2 guides new users from install to first dictation seamlessly

---

## Acceptance Criteria

- All interactive elements are keyboard-navigable and screen-reader-accessible
- At least 6 languages are supported (DE, EN, FR, ES, IT, PT) plus auto-detect
- Complete user and developer documentation exists in `docs/`
- README reflects v1.0 state with accurate feature list and install instructions
- Smoke test passes end-to-end on all supported platforms
- v1.0 release is tagged and published with signed installers

---

## Technical Notes

- Accessibility testing uses NVDA (Windows) and VoiceOver (macOS) — both free
- High contrast mode uses the `prefers-contrast` CSS media query, supported in all modern browsers/webviews
- Auto-detect language relies on whisper.cpp's built-in `--language auto` flag; only works with multilingual models
- Onboarding v2 replaces the simple modal from MS-10 with a multi-step wizard
- Documentation should include screenshots — use Tauri's screenshot capability or manual captures
- v1.0 tag triggers the release pipeline (MS-17 TASK-243) which builds and publishes all platform artifacts
