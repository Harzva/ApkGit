---
name: app-skills-suite
description: Reusable app product readiness patterns for version checks, online update manifests, one-click latest downloads, and safe overwrite-install behavior across Harzva apps.
---

# App Skills Suite

Use this skill when building or reviewing any Harzva app that needs a repeatable product shell: version visibility, update checks, latest downloads, install safety, and release readiness.

## Version And Update Contract

Every app home screen should expose:

- Current app version from the native bundle/package/build metadata.
- Online latest version from a public JSON manifest.
- A clear update state: checking, up to date, update available, or check failed.
- A visible "Download Latest" action that opens the best platform-specific asset.
- A short install note explaining whether overwrite install is supported on that platform.

## Manifest Rules

Publish a stable JSON file with the app site, for example:

```text
https://example.com/app-update.json
```

The manifest should follow `templates/app-update.json` and include:

- `latestVersion` and `latestTag`.
- `releaseUrl` for a human fallback.
- Platform-specific `downloadUrl` values.
- Platform-specific `installMode` values.
- Package or bundle identifiers where overwrite safety depends on identity.

## Overwrite Install Rules

Android:

- Overwrite install is allowed when package id and signing lineage match.
- Download buttons may open an APK, but installation must still go through the system installer.
- The manifest should state `same-package-same-signature-overwrite`.

Desktop:

- Prefer launcher-managed cache replacement or next-launch binary replacement.
- Do not overwrite a running executable in place.
- The manifest should state `launcher-cache-replace` or `next-launch-replace`.

iOS:

- Public apps should update through App Store or TestFlight.
- Enterprise/MDM apps may use managed distribution.
- Simulator previews may provide a zip, but production iOS apps must not claim silent self-update.

## App Home Checklist

- Version card is visible above secondary content.
- Current and online versions use the same semantic version format.
- The download action still works when the app is already up to date.
- Network failures degrade to a visible retry state.
- The app never hides platform install limitations.

## Release Checklist

- Bump app package version, native bundle version, and launcher package version together.
- Update `app-update.json` to point at the new tag and assets.
- Build and publish release assets before relying on the manifest in production.
- Keep old releases available so older apps can still update through the manifest.
