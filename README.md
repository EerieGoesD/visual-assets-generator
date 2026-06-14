# Visual Assets Generator

Turn a single source image into every app icon and store asset you need - Microsoft
Store, Windows `.ico`, macOS, Android, iOS, Chrome Extension, favicon, social/OG images
and Flatpak - exported as a ZIP. All image processing runs locally on your device.

This is a [Tauri 2](https://tauri.app) desktop app. The UI is a static, dependency-free
frontend (`src/`); icon generation runs entirely in the webview via the Canvas API. The
only native code is a small `write_file` command that writes the generated ZIP to the
location chosen in the native save dialog.

## Project layout

```
src/                 Static frontend (index.html, jszip.min.js, fonts/)
src-tauri/           Rust/Tauri shell (config, icons, write_file command)
Package.appxmanifest MSIX manifest (created by `winapp init`)
Assets/              MSIX tile/store icons (created by `winapp init`)
```

## Develop

```powershell
cargo tauri dev
```

(There is no JS build step or dev server - the frontend is static and served directly.)

## Build a release binary

```powershell
cargo tauri build --no-bundle
```

Output: `src-tauri/target/release/visual-assets-generator.exe` (a self-contained exe;
Tauri statically links the WebView2 loader). Drop `--no-bundle` to also produce the
NSIS installer (Windows) or `.app` + `.dmg` (macOS).

## Package as MSIX for the Microsoft Store (Windows)

Uses Microsoft's official [`winapp` CLI](https://learn.microsoft.com/windows/apps/dev-tools/winapp-cli/guides/tauri)
(`winget install microsoft.winappcli`).

```powershell
# 1. Release build
cargo tauri build --no-bundle

# 2. One-time: create manifest + Assets, and a dev signing cert
winapp init
winapp cert generate --if-exists skip

# 3. Stage the exe and pack
if (-not (Test-Path dist)) { New-Item -ItemType Directory dist | Out-Null }
Copy-Item src-tauri\target\release\visual-assets-generator.exe dist\ -Force
winapp pack .\dist --cert .\devcert.pfx
```

This produces `Visual Assets Generator_1.0.0.0_x64.msix` in the repo root.

To install/test the self-signed package locally:

```powershell
winapp cert install .\devcert.pfx   # run once, as Administrator
Add-AppxPackage .\*.msix
```

### Before submitting to the Microsoft Store

The self-signed identity is for local testing only. Reserve the app name in
[Partner Center](https://partner.microsoft.com), then edit `Package.appxmanifest` so
these match **exactly** what Partner Center shows under *Product management > Product identity*:

- `<Identity Name="..." Publisher="CN=..." Version="1.0.0.0" />`
- `<PublisherDisplayName>...</PublisherDisplayName>`

The Store re-signs the MSIX during submission, so no CA certificate is required. WebView2
(Evergreen) ships with Windows 10/11.

## macOS

`cargo tauri build` on a Mac produces the `.app` and `.dmg`. For the **Mac App Store** you
additionally need an Apple signing identity, app sandbox entitlements, and a provisioning
profile - configure these when building on macOS.

---

Made by [EERIE](https://eeriegoesd.com).
