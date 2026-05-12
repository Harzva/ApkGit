# ApkGit

ApkGit is a desktop-first open-source tool for discovering APK assets published in GitHub and Gitee Releases. It helps technical Android users inspect a repository, list release APKs, and download the original files from the upstream release page.

> Current product route: this Rust/egui codebase is positioned as the **desktop utility**. The Android-native MVP described in the planning documents should be built separately with Kotlin + Jetpack Compose when mobile installation, permissions, PackageManager integration, and Material 3 UX become the main goal.

## Status

This repository is an early `0.1.0` MVP. It is intended for testing the core chain:

1. Parse a GitHub/Gitee repository URL.
2. Fetch repository metadata and releases.
3. Detect APK assets in releases.
4. Download selected APK files.
5. Show release metadata, file size, download count, and upstream links.

The project does **not** host APK files, re-sign packages, mirror binaries, or audit third-party code. Every APK link points to the original upstream Release asset.

## Features

| Feature | Status |
| --- | --- |
| GitHub repository parsing | Implemented |
| Gitee repository parsing | Implemented |
| Release APK listing | Implemented |
| APK download | Implemented |
| GitHub token support | Implemented |
| SHA256 helper | Implemented, integration pending |
| Desktop UI | Implemented MVP |
| Android native installer flow | Planned for Kotlin/Compose app |
| Multi-repository management | Planned |
| Update detection | Planned |
| Security scoring and hot lists | Planned |

## Safety Model

ApkGit is a discovery and download helper, not an app store with review guarantees.

- APK files are downloaded from upstream GitHub/Gitee Release URLs.
- Users should verify the upstream project, release notes, license, signatures, and hashes before installing.
- Future versions should display APK SHA256, signing certificate fingerprints, package metadata, source license, and historical signature changes before install actions.
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

## Release Builds

The GitHub Actions workflow builds desktop artifacts for Linux, macOS, and Windows on every push and pull request. Android cargo-apk output is kept as an experimental artifact only; the product route for a polished Android MVP is Kotlin + Jetpack Compose.

Tag a release with `v*`, for example:

```bash
git tag v0.1.0
git push origin v0.1.0
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

Screenshots and short demo GIFs should be placed in `docs/screenshots/` before the first public release. Use real Release data and avoid showing private GitHub tokens.

## License

ApkGit is licensed under GPL-3.0-or-later. See [LICENSE](LICENSE).
