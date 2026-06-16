# Publishing Store Asset Studio to the Mac App Store

All of this runs on your **Mac** (Tauri macOS builds need macOS + Xcode). Bundle id:
`com.eeriegoesd.storeassetstudio`. The repo already includes `src-tauri/Entitlements.plist`
and the `bundle.macOS.entitlements` reference in `tauri.conf.json`.

## 0. One-time machine setup
```bash
xcode-select --install                      # Xcode command line tools (or install full Xcode)
# Rust + Tauri CLI (if not already): https://tauri.app
rustup target add aarch64-apple-darwin x86_64-apple-darwin   # universal build
```

## 1. Apple Developer / App Store Connect (web)
1. Enroll in the **Apple Developer Program** ($99/yr) if you have not.
2. In **App Store Connect > Apps > +**, create a new app:
   - Platform: macOS
   - Name: **Store Asset Studio** (must be unique on the Mac App Store - if taken, pick another)
   - Bundle ID: register/select `com.eeriegoesd.storeassetstudio`
   - SKU: any internal string (e.g. `storeassetstudio`)
3. Note your **Team ID** (App Store Connect > Membership).
4. (Optional but recommended) Enroll in the **Small Business Program** to drop commission from 30% to 15%.

## 2. Certificates + provisioning (Xcode or the Developer portal)
Easiest via Xcode: open Xcode > Settings > Accounts > add your Apple ID > Manage Certificates,
or use the Developer portal:
1. Create an **Apple Distribution** certificate (signs the `.app`).
2. Create a **Mac Installer Distribution** certificate (signs the `.pkg`).
3. Create a **Mac App Store** provisioning profile for `com.eeriegoesd.storeassetstudio` and download it (e.g. `StoreAssetStudio_AppStore.provisionprofile`).

## 3. Fill in the placeholders in this repo
1. In `src-tauri/Entitlements.plist`, replace **TEAMID** (both places) with your Team ID.
2. In `src-tauri/tauri.conf.json`, add the downloaded profile under `bundle.macOS`:
   ```json
   "macOS": {
     "minimumSystemVersion": "10.15",
     "entitlements": "Entitlements.plist",
     "files": { "embedded.provisionprofile": "/absolute/path/to/StoreAssetStudio_AppStore.provisionprofile" }
   }
   ```

## 4. Build a signed universal .app
```bash
# Sign the .app with your Apple Distribution cert (use the exact name from `security find-identity -v -p codesigning`)
export APPLE_SIGNING_IDENTITY="Apple Distribution: YOUR NAME (TEAMID)"
cargo tauri build --bundles app --target universal-apple-darwin
```
Output: `src-tauri/target/universal-apple-darwin/release/bundle/macos/Store Asset Studio.app`

## 5. Package into a signed .pkg
```bash
cd src-tauri/target/universal-apple-darwin/release/bundle/macos
xcrun productbuild --sign "3rd Party Mac Developer Installer: YOUR NAME (TEAMID)" \
  --component "Store Asset Studio.app" /Applications \
  "Store Asset Studio.pkg"
```
(The installer identity is your **Mac Installer Distribution** cert. Run
`security find-identity -v` to see its exact name.)

## 6. Upload to App Store Connect
1. Create an **App Store Connect API key** (Users and Access > Integrations > App Store Connect API). Download `AuthKey_<KEYID>.p8` into `~/.appstoreconnect/private_keys/`.
2. Upload:
   ```bash
   xcrun altool --upload-app --type macos \
     --file "Store Asset Studio.pkg" \
     --apiKey <KEYID> --apiIssuer <ISSUER_ID>
   ```
   (You can also drag the `.pkg` into the **Transporter** app instead.)

## 7. Finish the listing and submit
In App Store Connect: add the description, keywords, screenshots (the Microsoft Store copy
mostly carries over), set the price ($4.99), pick the **Developer Tools** category, attach the
build that just finished processing, answer the privacy questions (no data collected), and
**Submit for Review**.

## Notes
- **Verify saving works under the sandbox** before submitting: open the built `.app`, generate
  assets, and confirm both ZIP save and folder save write correctly. The
  `files.user-selected.read-write` entitlement covers paths the user picks in the save/open
  panel. If folder save misbehaves under sandbox, fall back to a direct notarized `.dmg`.
- Bump `version` in `tauri.conf.json` for each new submission (the Store rejects duplicate versions).
