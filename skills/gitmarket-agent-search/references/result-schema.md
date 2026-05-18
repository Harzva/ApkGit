# GitMarket Agent Result Schema

Use this shape when returning structured results to another agent.

```json
{
  "query": "android package manager open-source",
  "source_policy": "upstream-only",
  "results": [
    {
      "rank": 1,
      "type": "release | repository | skill | mcp",
      "name": "owner/repo",
      "platform": "GitHub | Gitee | GitCode",
      "repo_url": "https://github.com/owner/repo",
      "latest_release": {
        "tag": "v1.2.3",
        "url": "https://github.com/owner/repo/releases/tag/v1.2.3",
        "published_at": "2026-05-19"
      },
      "assets": [
        {
          "name": "app-arm64-v8a.apk",
          "kind": "APK | EXE | IPA | DMG | ZIP | source",
          "url": "https://github.com/owner/repo/releases/download/v1.2.3/app.apk",
          "size": 12345678,
          "sha256": null
        }
      ],
      "license": "MIT",
      "trust": {
        "upstream_official": true,
        "archived": false,
        "prerelease": false,
        "checksum_available": false,
        "credentials_required": false
      },
      "agent_fit": "Why this result helps the current task.",
      "notes": ["Missing upstream checksum file."]
    }
  ]
}
```

Keep missing fields as `null` rather than inventing values.
