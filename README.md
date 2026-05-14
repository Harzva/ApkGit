# GitMarket

<p align="center">
  <strong>一个面向 GitHub / Gitee / GitCode Release 的开源软件发现、校验与下载助手。</strong>
</p>

<p align="center">
  <a href="https://github.com/Harzva/GitReleaseMarket/actions/workflows/build.yml"><img alt="Build" src="https://github.com/Harzva/GitReleaseMarket/actions/workflows/build.yml/badge.svg" /></a>
  <a href="https://github.com/Harzva/GitReleaseMarket/releases"><img alt="Release" src="https://img.shields.io/github/v/release/Harzva/GitReleaseMarket?include_prereleases" /></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue" /></a>
  <img alt="Rust" src="https://img.shields.io/badge/Rust-egui-orange" />
  <img alt="Android" src="https://img.shields.io/badge/Android-preview-green" />
</p>

<p align="center">
  <a href="https://harzva.github.io/GitReleaseMarket/">产品首页</a>
  ·
  <a href="https://harzva.github.io/GitReleaseMarket/app.html">Web 预览</a>
  ·
  <a href="https://harzva.github.io/GitReleaseMarket/mobile-preview.html">移动端预览</a>
  ·
  <a href="https://github.com/Harzva/GitReleaseMarket/releases">下载 Release</a>
  ·
  <a href="SECURITY.md">安全策略</a>
  ·
  <a href="ROADMAP.md">路线图</a>
</p>

GitMarket 不是重新分发 APK 的应用商店，而是一个“Release 市场入口”：搜索开源项目，读取官方 Release，展示资产类型、下载量、上游地址、SHA256 与安全提示，然后把下载动作导回原始发布源。

<p align="center">
  <img src="docs/assets/architecture_diagram.png" alt="GitMarket architecture" width="860" />
</p>

## 最新发布

`v0.1.6` 在 `v0.1.5` 基础上补齐 iOS SwiftUI 预览工程，并把 Android WebView preview APK 入口切到更像移动 App 的 `mobile-preview.html`。GitHub Release 会继续产出 Android preview、Android experimental、Windows、Linux、macOS 与 iOS simulator 预览包。

| 产物 | 入口 |
| --- | --- |
| Android experimental APK | [gitmarket-android-experimental.apk](https://github.com/Harzva/GitReleaseMarket/releases/latest) |
| Android WebView preview APK | [gitmarket-android-preview](https://github.com/Harzva/GitReleaseMarket/releases/latest) |
| iOS simulator preview | [gitmarket-ios-simulator.zip](https://github.com/Harzva/GitReleaseMarket/releases/latest) |
| Windows 桌面包 | [gitmarket-windows-x86_64.zip](https://github.com/Harzva/GitReleaseMarket/releases/latest) |
| Linux 桌面包 | [gitmarket-linux-x86_64.tar.gz](https://github.com/Harzva/GitReleaseMarket/releases/latest) |
| macOS Apple Silicon 桌面包 | [gitmarket-macos-aarch64.tar.gz](https://github.com/Harzva/GitReleaseMarket/releases/latest) |
| 在线体验 | [GitHub Pages](https://harzva.github.io/GitReleaseMarket/) |

## 当前进度

| 模块 | 状态 | 说明 |
| --- | --- | --- |
| Web 首页 / Pages | 已上线 | `docs/index.html` 作为宣传页，`docs/app.html` 作为可交互预览，`docs/mobile-preview.html` 作为 APK / IPA 本地预览 |
| Rust / egui 客户端 | `0.1.6` | 已升级桌面工作台布局、移动端卡片流、四套主题、中英双语、多源发现页，并内置中文字体子集 |
| Android preview APK | `0.1.6` | WebView 壳，指向 GitHub Pages 移动端预览地址 |
| Android Rust experimental APK | 已发布 | 用于验证 egui 移动 UI，不作为最终 Android 原生路线 |
| 桌面包 | 已发布 | Windows / Linux / macOS release artifact |
| iOS | SwiftUI 预览版 | `ios/GitMarket` 已提供原生 SwiftUI 壳、搜索台 WebView、安全页、来源页、主题和中英双语；正式 IPA 需要 Apple 签名配置 |

## 新版 App 方向

这次移动端 UI 不再是单页输入框，而是完整的产品壳：

- **ME Agent 主题**：默认主题，白底、轻阴影、信息流卡片，参考移动端个人工作台风格。
- **暖色卡片主题**：柔和橙色软件市场视觉，适合轻量应用集合页。
- **清爽蓝白主题**：偏工具型、干净、高对比，适合桌面和浅色系统。
- **橙色发布主题**：偏品牌发布和活动宣传，适合展示新版本与推荐项目。
- **中文 / English**：设置页可切换语言，导航、搜索、下载、安全页均覆盖。

## APK / IPA 本地预览

`docs/mobile-preview.html` 可以直接用浏览器本地打开，用来减少反复安装测试包的次数：

- **APK WebView 模式**：`0.1.6` 起默认打开 `docs/mobile-preview.html`，更接近真实移动端 App 壳；搜索台仍可从预览页进入。
- **Android Native / iOS Native 模式**：用同一套示例 Release 数据预演移动端原生界面，支持四套主题、中文 / English、底部导航、仓库检查、下载状态和安全页。
- **验收方式**：日常先在 HTML 里检查布局、文案、主题和交互，最终发版前再用真实 Android / iOS 设备做关键路径验收。

## 核心能力

| 能力 | GitHub | Gitee | GitCode |
| --- | --- | --- | --- |
| 仓库 URL 解析 | 支持 | 支持 | 支持 |
| 仓库信息读取 | 支持 | 支持 | 源站跳转兜底 |
| Release 资产读取 | 支持 | 支持 | 源站跳转兜底 |
| 关键词搜索 | 支持 | 支持 | 搜索入口兜底 |
| 默认无关键词展示 | 10 条 | 10 条 | 入口卡片 |
| 搜索结果展示 | 20 条 | 20 条 | 入口卡片 |

支持识别的 Release 资产包括 `.apk`, `.exe`, `.msi`, `.dmg`, `.pkg`, `.AppImage`, `.deb`, `.rpm`, `.ipa`, `.zip`, `.tar.gz`, `.tgz`, `.7z`。

## 安全边界

GitMarket 的底线是透明，不伪装成审核过的商店：

- 不托管、不镜像、不重签第三方二进制。
- 下载链接指向上游官方 Release asset。
- 展示上游仓库、Release 页面、许可证、源代码地址和下载量。
- 下载后计算本地 SHA256；后续将对接上游校验文件和签名指纹。
- GitHub Token 只用于提高 API rate limit；正式移动端应接入系统级凭据存储。
- Android 权限用途必须透明，尤其是网络访问和安装包相关权限。

更多细节见 [SECURITY.md](SECURITY.md)、[PRIVACY.md](PRIVACY.md)、[DISCLAIMER.md](DISCLAIMER.md)。

## 本地开发

Windows 本地编译 Rust 桌面端需要安装 Visual Studio Build Tools，并勾选 **Desktop development with C++**，否则会缺少 `link.exe`。

```bash
cargo fmt
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo run --release
```

Android preview APK:

```powershell
cd android-preview
.\gradlew.bat :app:assembleRelease `
  -PreleaseStoreFile="D:\path\to\release.jks" `
  -PreleaseStorePassword="changeit" `
  -PreleaseKeyAlias="release" `
  -PreleaseKeyPassword="changeit"
```

产物位置：`android-preview/app/build/outputs/apk/release/app-release.apk`。

## CI 与发布

`.github/workflows/build.yml` 会在 push、PR 和 tag 上执行：

- `cargo fmt --check`
- `cargo test --locked`
- `cargo clippy --locked --all-targets --all-features -- -D warnings`
- Windows / Linux / macOS 桌面构建
- Android preview APK 构建
- Android Rust experimental APK 非阻塞构建
- iOS simulator preview 构建

发布新版本：

```bash
git tag v0.1.6
git push origin v0.1.6
```

Tag 构建通过后，GitHub Release 会自动上传桌面包、Android preview APK、实验 APK、iOS simulator 预览包和 SHA256 文件。

## 路线图

近期重点：

- 让 Rust/egui 客户端稳定通过 CI，产出可试用桌面包和实验 APK。
- 给移动端补截图、录屏 GIF、下载流和安全提示的真实演示。
- 增强 GitCode 接入方式，必要时通过后端代理统一多源搜索协议。
- 增加 Release 缓存、懒加载、Show more 展开和失败重试。
- 提取 Android 包签名指纹、权限说明和签名变更提醒。

长期路线：

- 桌面端继续由 Rust/egui 承担。
- Android 正式 MVP 建议使用 Kotlin + Jetpack Compose 重做，以便接入 PackageManager、FileProvider、WorkManager 和系统凭据存储。
- iOS 从 SwiftUI/WebKit preview 开始，签名和 TestFlight 自动化需要 Apple Developer 配置。

## 参与贡献

欢迎提交 Issue、功能建议和 UI 参考。安全问题请不要公开发 Issue，按 [SECURITY.md](SECURITY.md) 私下报告。

```bash
git clone https://github.com/Harzva/GitReleaseMarket.git
cd GitReleaseMarket
cargo fmt
cargo test --locked
```

## License

GitMarket is licensed under GPL-3.0-or-later. See [LICENSE](LICENSE).
