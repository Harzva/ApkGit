# App Skill Map

Use this map to route app work through `app-skills-suite` while preserving specialist skills as focused modules.

## Routing Rule

Start with `app-skills-suite` for any app task, then load only the specialist skill needed for the current slice. If a named skill is unavailable in the current session, apply the same category guidance as a fallback and note the missing skill briefly.

## Product And UI Design

- `appui-design-skill`: Mobile app UI concepts, native-feeling screens, and visual direction.
- `frontend-design`: Distinctive web or hybrid app frontends with stronger visual taste.
- `taste-design`: Semantic design systems, design taste, and reusable visual language.
- `deposterize-product-ui`: Remove generic AI-looking dashboard or product UI patterns.
- `product-expression-guard`: Final UI taste gate before publishing or showing users.
- `web-accessibility`: WCAG-oriented accessibility review for web and hybrid surfaces.
- `ui-animation`: Purposeful motion, page-load animation, and interaction polish.
- `open-design-skill`: Open Design and Stitch-style visual workflows when applicable.

## UI Implementation

- `figma-implement-design`: Translate Figma designs into implementation.
- `image-to-ui-skill`: Convert screenshots or design images into frontend code.
- `react:components`: Convert Stitch designs into React components.
- `shadcn-ui`: Use or adapt shadcn/ui components inside an existing React stack.
- `tailwind-design-system`: Build or maintain Tailwind design tokens and systems.
- `10-factor-html`: Audit and improve single-page HTML prototypes.
- `app-preview-lab`: Create local browser-openable previews for app screens.

## Mobile And Native QA

- `android-release-emulator-qa-skill`: Validate Android APKs in an emulator, including install/update behavior.
- `ios-simulator-video-qa`: Validate iOS simulator builds and capture simulator evidence.
- `mobilecode-mac-local-qa`: Build and QA mobile code locally on macOS.

## Release, CI, And Distribution

- `github`: Use `gh` for release, asset, workflow, issue, and PR operations.
- `gh-actions-release-builder`: Design or repair GitHub Actions release workflows.
- `gh-run-view-monitor`: Watch GitHub Actions runs and inspect failures.
- `github-management-suite`: Cross-platform GitHub repository operations.
- `ci-ops-suite`: Diagnose and harden CI pipelines.
- `readme-design`: Make README release and install instructions clear.
- `readme-showcase-screenshot`: Produce README-ready app screenshots when useful.

## App Update Pattern

Apply this suite directly when a user asks for:

- Current version and online latest version in the app home UI.
- Public `app-update.json` manifests.
- One-click latest-download buttons.
- Android overwrite-install behavior.
- Desktop self-update or launcher replacement semantics.
- Release assets that line up with manifest download URLs.

## Standard App Workflows

New app:

1. Use `appui-design-skill` or `frontend-design` for the first UI direction.
2. Implement the smallest working shell.
3. Add the version card and update manifest contract from `app-skills-suite`.
4. Add preview or simulator QA based on platform.
5. Add release assets and verify the manifest after publish.

Existing app redesign:

1. Preserve the current stack and product constraints.
2. Use UI design skills to create a more comfortable home screen.
3. Add or repair the app update contract.
4. Preview locally before release if the user asks for validation.

Release/update task:

1. Bump all relevant version files together.
2. Update `app-update.json`.
3. Build release assets.
4. Publish tag and Release.
5. Confirm Release assets match manifest asset names.
6. Confirm the online manifest reports the new version.
