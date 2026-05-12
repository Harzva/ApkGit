# Contributing

Thanks for helping make GitMarket safer and more useful.

## Development Checks

Run these before opening a pull request:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Pull Request Guidelines

- Keep changes focused.
- Add or update tests for parser, API, download, or safety behavior changes.
- Do not commit private tokens, downloaded APKs, or personal screenshots.
- For UI changes, include a screenshot or short GIF in `docs/screenshots/`.
- For APK-related behavior, describe the security impact in the PR.

## Product Route

This repository is desktop-first Rust/egui. Android-native MVP work should happen in a separate Kotlin + Jetpack Compose app or a clearly separated module.
