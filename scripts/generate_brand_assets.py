from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw


ROOT = Path(__file__).resolve().parents[1]


def draw_logo(size: int) -> Image.Image:
    scale = 4
    canvas = 512 * scale
    img = Image.new("RGBA", (canvas, canvas), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)

    def v(n: float) -> int:
        return round(n * scale)

    bg = "#FFF7EA"
    border = "#F3C58C"
    orange = "#FF8A14"
    ink = "#1E293B"
    green = "#2FD18A"
    amber = "#FFB84D"

    d.rounded_rectangle([v(34), v(34), v(478), v(478)], radius=v(118), fill=bg)
    d.rounded_rectangle(
        [v(34), v(34), v(478), v(478)],
        radius=v(118),
        outline=border,
        width=v(12),
    )

    d.ellipse([v(120), v(118), v(376), v(374)], outline=orange, width=v(44))
    d.line([v(338), v(337), v(430), v(429)], fill=orange, width=v(48))

    d.line([v(168), v(231), v(225), v(231)], fill=ink, width=v(20))
    d.line([v(225), v(231), v(296), v(174)], fill=ink, width=v(20))
    d.line([v(225), v(231), v(303), v(302)], fill=ink, width=v(20))
    for x, y, color in [
        (168, 231, ink),
        (225, 231, green),
        (296, 174, ink),
        (303, 302, ink),
    ]:
        d.ellipse([v(x - 25), v(y - 25), v(x + 25), v(y + 25)], fill=color)

    for x, y, color in [
        (330, 91, green),
        (382, 91, amber),
        (330, 143, amber),
        (382, 143, ink),
    ]:
        d.rounded_rectangle([v(x), v(y), v(x + 38), v(y + 38)], radius=v(12), fill=color)

    d.arc([v(150), v(287), v(370), v(442)], start=25, end=160, fill=ink, width=v(20))
    return img.resize((size, size), Image.Resampling.LANCZOS)


def save_png(path: Path, size: int) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    draw_logo(size).save(path)


def main() -> None:
    brand_sizes = [16, 32, 48, 72, 96, 144, 192, 256, 512, 1024]
    for size in brand_sizes:
        save_png(ROOT / "assets" / "brand" / f"gitmarket-logo-{size}.png", size)
    save_png(ROOT / "assets" / "brand" / "gitmarket-logo.png", 512)
    draw_logo(256).save(
        ROOT / "assets" / "brand" / "gitmarket-logo.ico",
        sizes=[(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)],
    )

    for size in [192, 512, 1024]:
        save_png(ROOT / "docs" / "assets" / f"gitmarket-logo-{size}.png", size)

    android_sizes = {
        "mipmap-mdpi": 48,
        "mipmap-hdpi": 72,
        "mipmap-xhdpi": 96,
        "mipmap-xxhdpi": 144,
        "mipmap-xxxhdpi": 192,
    }
    for folder, size in android_sizes.items():
        for name in ["ic_launcher.png", "ic_launcher_round.png"]:
            save_png(ROOT / "android-res" / folder / name, size)

    save_png(
        ROOT
        / "ios"
        / "GitMarket"
        / "GitMarket"
        / "Resources"
        / "Assets.xcassets"
        / "GitMarketLogo.imageset"
        / "gitmarket-logo-512.png",
        512,
    )
    save_png(
        ROOT
        / "ios"
        / "GitMarket"
        / "GitMarket"
        / "Resources"
        / "Assets.xcassets"
        / "AppIcon.appiconset"
        / "gitmarket-logo-1024.png",
        1024,
    )


if __name__ == "__main__":
    main()
