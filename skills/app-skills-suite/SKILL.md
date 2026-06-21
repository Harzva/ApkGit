---
name: app-skills-suite
description: Orchestrate Harzva app work across product UI, mobile and desktop implementation, local preview, emulator QA, release packaging, online update manifests, latest-download buttons, and safe overwrite-install behavior. Use when creating, redesigning, testing, releasing, or standardizing any app or app-like product.
---

# App Skills Suite

Use this as the first-stop suite for Harzva app work. It is an orchestrator, not a replacement for specialist skills: keep specialized skills independent, but route through this suite so every app gets the same product shell, QA expectations, and release/update contract.

For the full routing map, read `references/app-skill-map.md` when the task spans more than one app discipline or when deciding which specialist skill to use.

## Default App Lifecycle

Follow this order unless the user asks for a narrower slice:

1. Product shape: define the core screen, platform target, user promise, and update story.
2. UI design: create a polished app shell before implementing details.
3. Implementation: preserve the existing tech stack and platform conventions.
4. Preview: produce a local browser, simulator, or emulator preview when the platform supports it.
5. QA: run only the checks the user asked for, or the checks required by a release request.
6. Release: publish assets, update the version manifest, and verify remote availability.
7. Reuse: capture repeatable app patterns back into this suite.

## Always-On App Shell

Every Harzva app home screen should expose:

- Current app version from native bundle, package, or build metadata.
- Online latest version from a public JSON manifest.
- A visible state: checking, up to date, update available, or check failed.
- A "Download Latest" action that opens the best platform-specific asset.
- A short install note explaining overwrite behavior for the current platform.

## Online Update Manifest

Publish a stable JSON file with the app site, for example:

```text
https://example.com/app-update.json
```

Use `templates/app-update.json` as the base. Include:

- `latestVersion` and `latestTag`.
- `releaseUrl` as a human fallback.
- Platform-specific `downloadUrl` values.
- Platform-specific `installMode` values.
- Package or bundle identifiers where overwrite safety depends on identity.

## Safe Overwrite Rules

Android:

- Overwrite install is allowed only when package id and signing lineage match.
- Download buttons may open an APK, but installation must still go through the system installer.
- Use `same-package-same-signature-overwrite` in the manifest.

Desktop:

- Prefer launcher-managed cache replacement or next-launch binary replacement.
- Do not overwrite a running executable in place.
- Use `launcher-cache-replace` or `next-launch-replace` in the manifest.

iOS:

- Public apps update through App Store or TestFlight.
- Enterprise apps may use managed distribution.
- Simulator previews may provide a zip, but production iOS apps must not claim silent self-update.

## App Home Checklist

- Version card is visible above secondary content.
- Current and online versions use the same semantic version format.
- The download action works even when the app is already up to date.
- Network failures degrade to a visible retry state.
- Platform install limitations are honest and visible.

## Release Checklist

- Bump app package version, native bundle version, and launcher package version together.
- Update `app-update.json` to point at the new tag and assets.
- Build and publish release assets before relying on the manifest in production.
- Verify release assets and the online manifest after publishing.
- Keep old releases available so older apps can still update through the manifest.

## Suite Maintenance

- Add new app-related specialist skills to `references/app-skill-map.md`.
- Prefer routing to existing specialist skills over copying their full instructions here.
- Keep this `SKILL.md` lean; move detailed platform playbooks into references or templates.
- Never store credentials, token values, cookies, `.env` values, raw chat logs, or private machine paths in this suite.
