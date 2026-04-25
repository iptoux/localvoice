# Tauri AppImage bundling requires a real square PNG icon

**Date:** 2026-04-25
**Area:** Tauri / Linux / CI
**Milestone:** Release pipeline

## What Happened

The Linux release build failed during AppImage bundling with:

`couldn't find a square icon to use as AppImage icon`

## Root Cause

Some configured PNG icons had square-looking filenames but non-square image dimensions. For example, `128x128.png` was 128x118 and `32x32.png` was 64x59. The AppImage bundler needs an actual square PNG icon and can fail even when the file name suggests the icon is square.

## Fix / Solution

Added a real 256x256 PNG icon to `src-tauri/icons/256x256.png` and changed `tauri.conf.json` so the Tauri bundle icon list starts with square PNG assets and no longer includes the non-square PNG files.

## Learning / Rule of Thumb

For Tauri Linux AppImage builds, verify actual PNG dimensions, not just filenames. Keep at least one real square PNG in the bundle icon list, preferably a standard size like 256x256.

## References

- `src-tauri/tauri.conf.json`
- `src-tauri/icons/256x256.png`
