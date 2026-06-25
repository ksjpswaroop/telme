# Telme — build & release guide

## Logos & assets

All logos live under `src-tauri/icons/`:
- `icon.png` (1024×1024 master) — used by `cargo tauri icon` to regenerate every OS size
- `icon.icns` (macOS bundle icon)
- `icon.ico` (Windows bundle icon)
- All required Tauri sizes: 32, 64, 128, 128@2x, plus Tauri/Linux/Windows store variants
- `logo-1024.png` — marketing-friendly logo (same artwork, master source)

## Brand

- **Mark**: rounded navy square with a white magnifying glass, AI sparkle tucked into the lower-right of the lens
- **Color**: deep navy gradient `#0F172A → #1E293B` (slate-900 → slate-800)
- **Accent**: warm yellow sparkle (`#FCD34D`) + soft white core
- **Background**: subtle vertical gradient + radial top highlight
- **System font**: SF Pro / Helvetica Neue

## Building

### One-shot release build (macOS)

```bash
cd ~/projects/telme

# Build release binary + .app bundle + .dmg
pnpm tauri build

# Output:
#   src-tauri/target/release/bundle/macos/Telme.app     (7.2 MB)
#   src-tauri/target/release/bundle/dmg/Telme_0.1.0_aarch64.dmg  (3.9 MB)
```

The default `pnpm tauri build` runs Tauri's `bundle_dmg.sh` which produces a
**plain** DMG (no background image, default Finder layout).

### Building the **beautiful** custom DMG

The beautiful DMG uses a hand-designed background, positioned icons, and an
Applications symlink. To rebuild it after the release build:

```bash
DMG_SCRIPT=src-tauri/target/release/bundle/dmg/bundle_dmg.sh
APP_BUNDLE=src-tauri/target/release/bundle/macos/Telme.app
BACKGROUND=src-tauri/dmg-background.png
ICON_ICNS=src-tauri/icons/icon.icns
OUTPUT=src-tauri/target/release/bundle/dmg/Telme_0.1.0_aarch64.dmg

# Remove any leftover DMG / volume state first
rm -f "$OUTPUT"
hdiutil detach /tmp/telme_dmg_mount 2>/dev/null

bash "$DMG_SCRIPT" \
  --volname "Telme" \
  --volicon "$ICON_ICNS" \
  --background "$BACKGROUND" \
  --window-pos 200 120 \
  --window-size 900 540 \
  --icon-size 130 \
  --text-size 14 \
  --icon "Telme.app" 560 200 \
  --app-drop-link 780 200 \
  "$OUTPUT" \
  "$APP_BUNDLE"
```

This produces the 3.9 MB DMG that, when double-clicked, shows:
- A 900×540 Finder window with the `dmg-background.png` backdrop
- Telme.app icon positioned at (560, 200)
- Applications folder symlink positioned at (780, 200)
- "Telme" label under the app icon, "Applications" under the folder
- Tagline at the bottom of the window

## Regenerating icons after a logo change

```bash
# Drop a new master PNG into src-tauri/icons/icon.png (1024x1024)
cargo tauri icon src-tauri/icons/icon.png
```

This regenerates every OS-required size (32, 64, 128, 128@2x, icns, ico, plus
all Windows/Linux/Android variants).

## Verifying the DMG

```bash
# Checksum
hdiutil verify src-tauri/target/release/bundle/dmg/Telme_0.1.0_aarch64.dmg
# -> checksum of "...dmg" is VALID

# Mount + list
hdiutil attach -nobrowse -readonly -mountpoint /tmp/mnt \
  src-tauri/target/release/bundle/dmg/Telme_0.1.0_aarch64.dmg
ls /tmp/mnt
# -> .DS_Store  .VolumeIcon.icns  .background/  Telme.app/
hdiutil detach /tmp/mnt
```

## Code signing (TODO, requires Apple Developer ID)

The `.app` and `.dmg` are currently **unsigned**, so first-launch will be
gated by Gatekeeper on a non-dev machine. To ship to others:

1. Acquire a **Developer ID Application** certificate from Apple.
2. Add to `tauri.conf.json`:
   ```json
   "bundle": {
     "macOS": {
       "signingIdentity": "Developer ID Application: Your Name (TEAMID)",
       "providerShortName": "TEAMID",
       "entitlements": null
     }
   }
   ```
3. For DMG signing/notarization, pass `--codesign` and `--notarize` flags to
   `bundle_dmg.sh` (it already supports these).
4. Run `pnpm tauri build` — Tauri will sign + notarize automatically.

## Verified macOS artifacts (2026-06-25)

```
DMG:  src-tauri/target/release/bundle/dmg/Telme_0.1.0_aarch64.dmg  (3.92 MB)
      Format: UDIF read-only compressed (zlib)
      Partition: Apple_HFS
      Checksum: CRC32 $A26FDAB6 — VALID
      Compressed ratio: 0.457

App:  src-tauri/target/release/bundle/macos/Telme.app              (7.21 MB)
      Binary: 7,325,792 bytes (release, LTO, opt-level="s")
      Info.plist: CFBundleIdentifier=com.telme.desktop, version=0.1.0
      Bundle launch: PID 46832 — runs cleanly
```
