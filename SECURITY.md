# Security Policy

## Supported Versions

`0.1.x` is an early MVP. Security fixes should target the latest `main` branch until stable releases begin.

## Reporting a Vulnerability

Please do not open a public issue for vulnerabilities. Use GitHub private vulnerability reporting if it is enabled, or contact the maintainer privately with:

- affected version or commit,
- reproduction steps,
- impact,
- suggested mitigation if known.

## APK Safety Scope

ReleaseMarket does not review, host, modify, or re-sign third-party APK files. It links to upstream Release assets and downloads them from their original URLs.

Before public release, the app should visibly expose:

- upstream repository URL,
- upstream release URL,
- project license,
- APK SHA256 digest when available,
- locally calculated SHA256 after download,
- Android signing certificate fingerprint when available,
- warning on signature change between versions.

## Token Handling

GitHub tokens are optional and should be treated as secrets. Do not log tokens, include them in screenshots, or store them in plain long-term storage unless the user explicitly opts in and the platform storage is appropriate.
