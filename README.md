# 🖼️ Visual Assets Generator

Generate every platform's app icons and store assets from a single source image - Microsoft Store, Windows `.ico`, macOS, Android, iOS, Chrome Extension, Favicon, Social/OG and Flatpak.

A desktop app for **Windows** and **macOS**. Everything runs locally on your device - your image is never uploaded.

---

## What it does

Drop in one icon image. Pick your target platforms. Save a ZIP with every asset pre-sized, named, and organized into folders, ready to drop straight into your project.

| Platform | Assets included |
|---|---|
| 🪟 Microsoft Store | 42 assets - badge logos, tiles, splash screen, store listing screenshots, and a multi-resolution Windows `.ico` |
| 🤖 Android (Play Store) | 8 assets - mipmap densities + feature graphic, with optional adaptive-icon safe-padding files |
| 🍎 iOS (App Store) | 16 assets - iPhone & iPad icon sizes |
| 🍏 macOS | 7 assets - app icon sizes 16 to 1024px |
| 🌐 Chrome Extension | 7 assets - extension icons + Web Store promo tiles |
| 🌐 Favicon (browser tab) | 6 assets - standard favicon sizes + Apple touch icon |
| 🔗 Social / OG image | 5 assets - Open Graph, Twitter Card, LinkedIn, Facebook cover, YouTube thumbnail |
| 📦 Flatpak (Flathub) | 4 assets - app icons at 64, 128, 256, 512px |

The ZIP contains folders like:

```
visual-assets.zip
├── Assets/                       ← Microsoft Store visual assets
│   ├── BadgeLogo.png
│   ├── Square150x150Logo.png
│   ├── Square44x44Logo.png
│   ├── Wide310x150Logo.png
│   ├── SplashScreen.png
│   └── ...
├── Screenshots/                  ← Microsoft Store listing images
│   ├── Poster 9x16.png
│   ├── Box 1x1.png
│   └── ...
├── Windows/
│   └── app_icon.ico              ← multi-resolution desktop icon (16-256px)
├── macOS/
│   ├── icon-16.png
│   ├── icon-512.png
│   ├── icon-1024.png
│   └── ...
├── Android/
│   ├── app-icon-512.png
│   ├── mipmap-hdpi/ic_launcher.png
│   ├── mipmap-anydpi-v26/ic_launcher.xml        (with safe-padding option)
│   ├── mipmap-hdpi/ic_launcher_foreground.png   (with safe-padding option)
│   ├── drawable/ic_launcher_background.xml      (with safe-padding option)
│   └── ...
├── iOS/
│   ├── AppStore-1024.png
│   ├── iPhone-180.png
│   └── ...
├── Chrome/
│   ├── icon-128.png
│   ├── small-promo-tile-440x280.png
│   └── ...
├── Favicon/
│   ├── favicon-16x16.png
│   ├── favicon-32x32.png
│   └── apple-touch-icon.png
├── Social/
│   ├── og-image-1200x630.png
│   ├── twitter-card-1200x628.png
│   └── ...
└── Flatpak/
    ├── icon-64.png
    ├── icon-256.png
    └── icon-512.png
```

## Options

- **Letterbox (contain)** or **Crop (cover)** - choose how your image fits each target size. For square icons both look the same.
- **Flutter naming convention** - exports iOS and macOS icons with the filenames Flutter expects (e.g. `Icon-App-60x60@2x.png`), so you can drop them straight into a Flutter project.
- **Add padding to Android app icons** - see below.

## Microsoft Store - badge logo note

Badge logos must pass [WACK](https://learn.microsoft.com/en-us/windows/uwp/debug-test-perf/windows-app-certification-kit) validation:

- All non-transparent pixels must be **pure white**
- Background must be **transparent**

The app removes the background automatically using dominant-color detection, Otsu thresholding, and foreground cropping, then forces the remaining pixels to white - producing compliant badge logos with no manual editing.

## Android adaptive icon padding

Enable **Add padding to Android app icons** when exporting Android assets if your launcher icon gets cropped on Pixel or other Android launchers. The ZIP then includes adaptive-icon foreground/background resources with the logo centered at a safe size on a white background.

---

The original web version's source is archived under [`/website`](website). Building the app from source is documented in [BUILDING.md](BUILDING.md).

Made by [EERIE](https://eeriegoesd.com).
