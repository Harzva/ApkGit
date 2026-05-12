# Roadmap

## 0.1.x: Desktop MVP

- Fix build and CI reliability.
- Show repository metadata and APK release assets.
- Download APK files from upstream Release URLs.
- Calculate and display local SHA256 after download.
- Improve rate-limit and network error states.

## 0.2.x: Safer Downloads

- Display upstream license and source URL.
- Show release page links for every asset.
- Add signature fingerprint extraction where supported.
- Warn on unknown license, missing source, prerelease builds, or signature changes.
- Add download history and open-download-folder action.

## 0.3.x: Discovery

- Multi-repository list.
- Curated FLOSS repository catalog.
- Basic hot list based on stars, release recency, and APK availability.
- Cache API responses to reduce rate-limit pressure.

## Android Native MVP

Build a separate Kotlin + Jetpack Compose Android app for:

- Material 3 mobile UI,
- PackageManager metadata inspection,
- Android 8+ unknown-source permission guidance,
- FileProvider-based install intents,
- WorkManager update checks,
- Android-specific privacy and permission flows.
