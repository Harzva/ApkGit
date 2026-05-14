# GitMarket iOS Preview

This is the first iOS product slice for GitMarket.

It is intentionally a SwiftUI preview app, not a signed App Store or TestFlight build yet. The app provides:

- native SwiftUI home, sources, security, and settings screens,
- Chinese / English language switching,
- four visual themes aligned with the Android/mobile preview direction,
- an embedded `WKWebView` search console for the live GitMarket Pages app,
- simulator build output through GitHub Actions.

## Build Locally On macOS

```bash
xcodebuild \
  -project ios/GitMarket/GitMarket.xcodeproj \
  -scheme GitMarket \
  -configuration Release \
  -sdk iphonesimulator \
  -destination 'generic/platform=iOS Simulator' \
  -derivedDataPath ios/build \
  CODE_SIGNING_ALLOWED=NO \
  build
```

The CI artifact is `gitmarket-ios-simulator.zip`. A real `.ipa` requires an Apple Developer team, signing certificate, provisioning profile, and distribution workflow.
