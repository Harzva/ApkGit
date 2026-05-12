# Privacy

ApkGit does not require an account and does not run its own backend service.

## Data Sent to Third Parties

When you search or inspect a repository, ApkGit sends requests to GitHub or Gitee APIs. Those services may receive:

- your IP address,
- requested repository owner/name,
- request timestamps,
- optional GitHub token if you configure one.

When you download an APK, the browser/download request is made to the upstream asset URL, usually GitHub/Gitee infrastructure or their file storage provider.

## Local Data

The desktop MVP may store downloaded APK files in a local `ApkGit_Downloads` directory. GitHub tokens should not be logged or committed. Future versions should use platform credential storage for persistent secrets.

## No Telemetry

The current app does not include analytics, crash reporting, ads, or tracking SDKs.
