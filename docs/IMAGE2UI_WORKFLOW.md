# GitMarket Image-to-UI Workflow

This project uses the `image2UI` skill as a design implementation checklist when turning UI screenshots, product mockups, or visual references into GitMarket screens.

The goal is not to copy a screenshot as one static image. The goal is to decide which parts should remain real UI and which parts need bitmap assets, then verify the final screen in the running app.

## When To Use

Use this workflow when adding or refining:

- Android/Rust egui mobile screens.
- GitHub Pages product sections.
- Theme variants based on screenshots or visual references.
- Hero art, app preview imagery, icon sheets, and screenshot/GIF assets.

## First Pass

Before coding, create a short audit:

| Area | Implementation | Difficulty | Notes |
| --- | --- | --- | --- |
| Navigation, labels, buttons | Code UI | Easy | Must stay selectable, translatable, and interactive. |
| Repository cards and release rows | Code UI | Medium | Use layout constraints; do not bake text into images. |
| Hero visual, product mockup, texture | Image asset | Hard | Use generated or source bitmap assets when CSS would look weak. |
| Logo, trademark, exact product screenshot | Source asset | Confirm | Prefer user-provided or official assets; do not hallucinate exact marks. |

## Asset Manifest

For every visual asset, record the target before generating or integrating it.

| id | UI slot | Type | Slot size | Export size | Treatment | Target path |
| --- | --- | --- | --- | --- | --- | --- |
| hero-release-board | Home hero | hero-image | full width | 2880x1600 | crop, webp | `docs/assets/generated/hero-release-board.webp` |
| android-theme-preview | README screenshot | screenshot | 430x900 | 860x1800 | png | `docs/screenshots/android-theme-preview.png` |
| theme-card-sheet | Theme gallery | composite | 16:9 | 1920x1080 | webp | `docs/assets/generated/theme-card-sheet.webp` |

## Prompt Rules

Prompts for image assets should describe one asset, not the whole application.

Include:

- purpose and UI slot,
- subject and composition,
- style tokens,
- aspect ratio and export size,
- safe whitespace for UI text,
- negative constraints such as no readable text, no logos, no watermarks, and no system status bar.

Keep real UI text, buttons, search inputs, badges, and repository data in code.

## GitMarket Theme Mapping

| Theme | Source Direction | Code UI | Image Assets |
| --- | --- | --- | --- |
| ME Agent | clean white mobile workspace | cards, tabs, search, metrics | optional app preview mockups |
| Warm Cards | soft orange market cards | chips, cards, CTA | optional mascot or product hero |
| Clean Blue | utility/dashboard clarity | dense data UI | optional architecture/screenshot visuals |
| Launch Orange | release/launch energy | hero controls, badges | optional launch artwork |

## Verification

Before considering an image-to-UI pass done:

- Run the app or page and capture the rendered result.
- Check desktop and mobile widths.
- Confirm generated assets actually render in the UI.
- Confirm no key text is trapped inside generated images.
- Confirm buttons, cards, tabs, and links remain clickable.
- Check that images are not stretched, blurred, cropped badly, or blocking controls.
- Compare the rendered result against the reference image and list accepted differences.

## Delivery Notes

When reporting an image-to-UI change, include:

- generated or source asset paths,
- screens where they are used,
- which areas remain code-rendered UI,
- validation commands or screenshots,
- any copyright, logo, font, or source-asset assumptions.
