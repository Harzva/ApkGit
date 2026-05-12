# GitMarket

GitMarket is a desktop-first open-source tool for discovering software packages published in GitHub and Gitee Releases. It helps users inspect repositories, list release assets for Android, Windows, macOS, Linux, and future iOS packages, then download the original files from the upstream release page.

> Current product route: this Rust/egui codebase is positioned as the **desktop utility**. The Android-native MVP described in the planning documents should be built separately with Kotlin + Jetpack Compose when mobile installation, permissions, PackageManager integration, and Material 3 UX become the main goal.

## Status

This repository is an early `0.1.2` MVP. It is intended for testing the core chain:

1. Parse a GitHub/Gitee repository URL.
2. Fetch repository metadata and releases.
3. Detect installable release assets.
4. Download selected upstream files.
5. Show release metadata, file size, download count, and upstream links.

The project does **not** host binaries, re-sign packages, mirror files, or audit third-party code. Every download link points to the original upstream Release asset.

## Features

| Feature | Status |
| --- | --- |
| GitHub repository parsing | Implemented |
| Gitee repository parsing | Implemented |
| Release asset listing | Implemented for APK in desktop MVP; Web preview covers more extensions |
| Upstream download | Implemented |
| GitHub token support | Implemented |
| SHA256 helper | Implemented, integration pending |
| Desktop UI | Implemented MVP |
| Android native installer flow | Planned for Kotlin/Compose app |
| Multi-repository management | Planned |
| Update detection | Planned |
| Security scoring and hot lists | Planned |

## Safety Model

GitMarket is a discovery and download helper, not an app store with review guarantees.

- Release assets are downloaded from upstream GitHub/Gitee Release URLs.
- Users should verify the upstream project, release notes, license, signatures, and hashes before installing.
- Future versions should display SHA256, signing certificate fingerprints, package metadata, source license, and historical signature changes before install actions.
- A GitHub token is optional and is only used to raise API rate limits.

See [SECURITY.md](SECURITY.md), [PRIVACY.md](PRIVACY.md), and [DISCLAIMER.md](DISCLAIMER.md) before distributing public builds.

## Local Development

Install Rust, then run:

```bash
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run --release
```

### Android preview APK

The Android preview app is a small WebView shell for the GitHub Pages Web app. It is not the planned Kotlin + Jetpack Compose MVP yet, but it gives the repository a reproducible Android package while the native mobile product is designed.

Requirements:

- JDK 17
- Android SDK command-line tools
- Android SDK Platform 35 and Build-Tools 35.0.0

On Windows, set `ANDROID_HOME` and `ANDROID_SDK_ROOT` to your Android SDK path, then run:

```powershell
cd android-preview
.\gradlew.bat :app:assembleRelease `
  -PreleaseStoreFile="D:\path\to\release.jks" `
  -PreleaseStorePassword="changeit" `
  -PreleaseKeyAlias="release" `
  -PreleaseKeyPassword="changeit"
```

The signed APK is written to `android-preview/app/build/outputs/apk/release/app-release.apk`.

## Release Builds

The GitHub Actions workflow builds desktop artifacts for Linux, macOS, and Windows on every push and pull request. It also builds the Android preview APK through `android-preview/gradlew`.

Current CI artifacts:

- `gitmarket-windows-x86_64.zip`, containing `gitmarket.exe`
- `gitmarket-linux-x86_64.tar.gz`
- `gitmarket-macos-aarch64.tar.gz`
- `gitmarket-android-preview-v0.1.2.apk`

Android cargo-apk output is kept as an experimental, non-blocking job only; the product route for a polished Android MVP is Kotlin + Jetpack Compose. iOS packaging is not enabled yet because there is no native iOS project or Apple Developer signing setup in this repository. The correct next step is a SwiftUI/WebKit preview target first, then signed IPA/TestFlight automation after certificates are available.

Tag a release with `v*`, for example:

```bash
git tag v0.1.2
git push origin v0.1.2
```

The workflow will publish GitHub Release artifacts when CI passes.

## Repository Structure

```text
src/
  api.rs        GitHub/Gitee API client
  app.rs        egui desktop UI
  data.rs       repository, release, and asset models
  download.rs   download and verification helpers
  main.rs       desktop and experimental Android entry points
```

## Screenshots

The GitHub Pages site lives in `docs/`. `docs/index.html` is the product landing page, and `docs/app.html` is the Web preview for browsing upstream Release assets. Screenshots and short demo GIFs should be placed in `docs/screenshots/` before the first public release. Use real Release data and avoid showing private GitHub tokens.

## License

GitMarket is licensed under GPL-3.0-or-later. See [LICENSE](LICENSE).
