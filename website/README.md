# 🖼️ Visual Assets Generator

Generate all platform visual assets from a single source image — Windows Store, Android, iOS, Chrome Extension & Favicon.

**[Here](https://eeriegoesd.com/visual-assets-generator-website/)**

---

## What it does

Upload one icon image. Select your target platforms. Download a ZIP with every asset pre-sized, named, and organized into folders. Ready to drop into your project.

| Platform | Assets included |
|---|---|
| 🪟 Windows (Microsoft Store) | 43 assets — badge logos, tiles, splash screen, store listing screenshots |
| 🤖 Android (Play Store) | 7 assets — mipmap densities + feature graphic, with optional adaptive icon safe-padding files |
| 🍎 iOS (App Store) | 16 assets — iPhone & iPad icon sizes |
| 🌐 Chrome Extension | 7 assets — extension icons + Web Store promo tiles |
| 🌐 Favicon (Browser Tab) | 6 assets — standard favicon sizes + Apple touch icon |
| 🔗 Social / OG Image | 5 assets — Open Graph, Twitter Card, LinkedIn, Facebook cover, YouTube thumbnail |
| 📦 Flatpak (Flathub) | 4 assets — app icons at 64, 128, 256, 512px |

The ZIP will contain folders like:

```
visual-assets.zip
├── Assets/                        ← Windows Store visual assets
│   ├── BadgeLogo.png
│   ├── Square150x150Logo.png
│   ├── Wide310x150Logo.png
│   └── ...
├── Screenshots/                   ← Windows Store listing images
│   ├── Poster 9x16.png
│   ├── Box 1x1.png
│   └── ...
├── Android/
│   ├── app-icon-512.png
│   ├── drawable/ic_launcher_background.xml
│   ├── mipmap-anydpi-v26/ic_launcher.xml
│   ├── mipmap-anydpi-v26/ic_launcher_round.xml
│   ├── mipmap-hdpi/ic_launcher.png
│   ├── mipmap-hdpi/ic_launcher_foreground.png
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
│   ├── favicon-64x64.png
│   └── apple-touch-icon.png
├── Social/
│   ├── og-image-1200x630.png
│   ├── twitter-card-1200x628.png
│   ├── linkedin-share-1200x627.png
│   ├── facebook-cover-820x312.png
│   └── youtube-thumbnail-1280x720.png
└── Flatpak/
    ├── icon-64.png
    ├── icon-128.png
    ├── icon-256.png
    └── icon-512.png
```
## Windows Store — Badge Logo note

Badge logos must pass [WACK](https://learn.microsoft.com/en-us/windows/uwp/debug-test-perf/windows-app-certification-kit) validation:
- All non-transparent pixels must be **pure white**
- Background must be **transparent**

This tool ports the background-removal algorithm from the companion Windows app, using dominant color detection, Otsu thresholding, and foreground cropping to produce compliant badge logos automatically.

## Android adaptive icon padding

Enable **Add padding to Android app icons** when exporting Android assets if the launcher icon gets cropped on Pixel or other Android launchers. The ZIP will include adaptive icon foreground/background resources with the logo centered at a safe size and a white background.
