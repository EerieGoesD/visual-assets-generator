# Publish Store Asset Studio to the Mac App Store

Do these in order, top to bottom, on your Mac. Bundle id is `com.eeriegoesd.storeassetstudio`.
Anything in ALL CAPS like `TEAMID` is a value you paste in from your own account.

---

## 1. Enroll in the Apple Developer Program
Go to https://developer.apple.com/programs/enroll and enroll ($99/yr). Skip if already enrolled.

## 2. Get your Team ID
Go to https://developer.apple.com/account → scroll to **Membership details** → copy **Team ID**
(10 characters, e.g. `AB12CD34EF`). You will need it in steps 3, 8, 11, 12.

## 3. Register the App ID
1. https://developer.apple.com/account/resources/identifiers/list
2. Click the blue **+** → select **App IDs** → **Continue** → **App** → **Continue**.
3. Description: `Store Asset Studio`. Bundle ID: choose **Explicit**, type `com.eeriegoesd.storeassetstudio`.
4. Leave capabilities as default → **Continue** → **Register**.

## 4. Create the App Store Connect listing
1. https://appstoreconnect.apple.com/apps → blue **+** → **New App**.
2. Platforms: tick **macOS**. Name: `Store Asset Studio`. Primary Language: English (U.S.).
3. Bundle ID: pick `com.eeriegoesd.storeassetstudio`. SKU: `storeassetstudio`. Full access → **Create**.

## 5. Make a certificate request (CSR) on your Mac
1. Open **Keychain Access** (Cmd+Space, type "Keychain Access").
2. Menu bar → **Keychain Access** → **Certificate Assistant** → **Request a Certificate From a Certificate Authority...**
3. Email: your Apple ID email. Common Name: your name. Select **Saved to disk** → **Continue** → save `CertificateSigningRequest.certSigningRequest` to your Desktop.

## 6. Create the two distribution certificates
At https://developer.apple.com/account/resources/certificates/list, do this **twice**:
1. Click **+**. Choose **Apple Distribution** → **Continue** → upload the CSR from step 5 → **Continue** → **Download**. Double-click the downloaded `.cer` to add it to Keychain.
2. Click **+** again. Choose **Mac Installer Distribution** → **Continue** → upload the **same** CSR → **Continue** → **Download** → double-click to add to Keychain.

## 7. Create the provisioning profile
1. https://developer.apple.com/account/resources/profiles/list → **+**.
2. Under **Distribution**, pick **Mac App Store Connect** (a.k.a. "Mac App Store") → **Continue**.
3. App ID: `com.eeriegoesd.storeassetstudio` → **Continue**.
4. Certificate: pick your **Apple Distribution** cert → **Continue**.
5. Name: `StoreAssetStudio MAS` → **Generate** → **Download**. Move the `.provisionprofile` to your Desktop.

## 8. Fill in the two placeholders in the repo
Run (replace `AB12CD34EF` with your real Team ID from step 2):
```bash
cd ~/visual-assets-generator
sed -i '' 's/TEAMID/AB12CD34EF/g' src-tauri/Entitlements.plist
```
Then open `src-tauri/tauri.conf.json` and change the `macOS` block to add your profile path:
```json
    "macOS": {
      "minimumSystemVersion": "10.15",
      "entitlements": "Entitlements.plist",
      "files": { "embedded.provisionprofile": "/Users/eeriegoesd/Desktop/StoreAssetStudio MAS.provisionprofile" }
    }
```

## 9. Find your exact certificate names
```bash
security find-identity -v
```
Copy the two full names, e.g. `Apple Distribution: Your Name (AB12CD34EF)` and
`3rd Party Mac Developer Installer: Your Name (AB12CD34EF)`. Use them in steps 10 and 11.

## 10. Build the signed universal app
```bash
cd ~/visual-assets-generator
export APPLE_SIGNING_IDENTITY="Apple Distribution: Your Name (AB12CD34EF)"
cargo tauri build --bundles app --target universal-apple-darwin
```
Output: `src-tauri/target/universal-apple-darwin/release/bundle/macos/Store Asset Studio.app`

## 11. Package into a signed .pkg
```bash
cd src-tauri/target/universal-apple-darwin/release/bundle/macos
xcrun productbuild --sign "3rd Party Mac Developer Installer: Your Name (AB12CD34EF)" \
  --component "Store Asset Studio.app" /Applications \
  "Store Asset Studio.pkg"
```

## 12. Create an App Store Connect API key (for upload)
1. https://appstoreconnect.apple.com/access/integrations/api → **+** → name `upload`, access **App Manager** → **Generate**.
2. Download `AuthKey_XXXXXXXXXX.p8`. Note the **Key ID** (`XXXXXXXXXX`) and the **Issuer ID** (shown at the top of that page).
3. Move the key:
```bash
mkdir -p ~/.appstoreconnect/private_keys
mv ~/Downloads/AuthKey_*.p8 ~/.appstoreconnect/private_keys/
```

## 13. Upload the build
```bash
xcrun altool --upload-app --type macos \
  --file "Store Asset Studio.pkg" \
  --apiKey XXXXXXXXXX --apiIssuer YOUR-ISSUER-ID
```
Wait for "No errors uploading". The build shows up in App Store Connect under your app within a few minutes.

## 14. Fill the listing and submit
In App Store Connect → your app:
- Pricing: $4.99. Category: **Developer Tools**. Privacy: **No data collected**.
- Add the description, keywords, and at least one screenshot (your Microsoft Store copy carries over).
- Select the processed build → **Add for Review** → **Submit**.

---

## Before submitting: sandbox check
The Mac App Store build runs sandboxed. Open the built `.app` (step 10 output), generate assets,
and confirm **both** ZIP save and folder save write correctly. The
`com.apple.security.files.user-selected.read-write` entitlement (already in `Entitlements.plist`)
covers files/folders the user picks in the dialog. If folder save fails under the sandbox, tell
me and I will switch you to a direct notarized `.dmg` instead.

## For every future update
Bump `version` in `tauri.conf.json` (the Store rejects a duplicate version), then redo steps 10, 11, 13, 14.
