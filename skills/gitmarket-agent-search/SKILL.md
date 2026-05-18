---
name: gitmarket-agent-search
description: Discover trustworthy GitHub, Gitee, and GitCode repositories, Releases, installable assets, Codex skills, and MCP servers for AI-agent tasks. Use when an agent needs to find upstream software, compare release assets, inspect license/safety signals, locate skills/MCP integrations, or choose what to install or cite.
---

# GitMarket Agent Search

## Overview

Use GitMarket as an upstream-first search market for agents. Prefer official repository and Release data over mirrors, and return compact evidence that helps the caller decide whether a project, asset, skill, or MCP server is safe and useful.

## Search Workflow

1. Clarify the target: release asset, source repository, local skill, MCP server, mobile app, desktop app, library, or comparison set.
2. Search GitHub first, then add Gitee or GitCode when the user asks for Chinese ecosystem coverage or multiple sources.
3. Inspect the upstream repository before recommending an installable asset.
4. Prefer the latest stable Release unless the user asks for prerelease, nightly, or source builds.
5. Report enough evidence for an agent to act without reading the whole repository.

## Evidence Checklist

For each recommended result, include:

- Repository URL and source platform.
- Latest Release tag and release URL when available.
- Relevant assets, platform labels, sizes, and download URLs.
- License, source code URL, and whether the asset is official upstream.
- SHA256 or upstream checksum availability when known.
- Safety notes: prerelease, archived repository, unknown license, missing source, suspicious asset naming, or missing checksum.
- Agent fit: why this result helps the requested task.

## Skill And MCP Discovery

Treat skills and MCP servers as optional capability plugins, separate from normal Release results.

- Skill results should identify the skill name, repository, install path or setup command, trigger scope, and trust signals.
- MCP results should identify server name, transport/setup command, tools exposed, permissions needed, and whether credentials are required.
- Do not mix Skill/MCP results into normal software downloads unless the user asks for all capability types.

## Output Shape

Use a compact ranked list. For complex searches, use the schema in `references/result-schema.md`.

Avoid claiming that GitMarket hosts, mirrors, verifies, or endorses third-party binaries. It discovers upstream Releases and helps agents inspect them.
