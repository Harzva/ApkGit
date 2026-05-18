# GitMarket Agent Market

GitMarket is evolving from a Release discovery app into a shared market for humans and AI agents.

## Positioning

GitMarket should serve two audiences:

- Human mobile and desktop users who want to discover, inspect, and download upstream open-source releases.
- AI agents that need a compact, trustworthy way to find repositories, release assets, skills, MCP servers, licenses, hashes, and setup paths.

## Surfaces

| Surface | Audience | Status | Goal |
| --- | --- | --- | --- |
| GitMarket App | Humans | Experimental | Search Releases, inspect safety signals, download upstream assets |
| GitMarket Skill | Agents | Initial repo skill | Return compact upstream evidence for Release / Skill / MCP discovery |
| GitMarket MCP | Agents and tools | Planned | Expose structured tools such as `search_releases`, `search_skills`, `search_mcp_servers`, and `inspect_repo` |

## Product Rules

- Release discovery stays the default product path.
- Skill and MCP discovery are opt-in plugins.
- The app must not mix installable software, local skills, and connector servers without clear labels.
- Results must prefer official upstream sources and show trust evidence before download or installation.
- Agent-facing output should be compact enough for tool use, with links for deeper inspection.

## Minimum Agent Schema

See `../skills/gitmarket-agent-search/references/result-schema.md`.
