import SwiftUI

enum GitMarketTab: Hashable {
    case home
    case discover
    case sources
    case security
    case settings
}

enum GitMarketLanguage: String, CaseIterable, Identifiable {
    case zh
    case en

    var id: String { rawValue }

    var label: String {
        switch self {
        case .zh: return "中文"
        case .en: return "English"
        }
    }

    func text(_ key: String) -> String {
        let zh: [String: String] = [
            "home": "首页",
            "discover": "发现",
            "sources": "来源",
            "security": "安全",
            "settings": "设置",
            "tagline": "人类与 Agent 的 Git 搜索市场",
            "heroTitle": "你的 Agent Git 智能看板",
            "heroBody": "聚合仓库、Release、Skill、MCP、安全信号和下载进度，把开源能力带回上游官方来源。",
            "verify": "待校验",
            "openConsole": "打开搜索台",
            "safety": "安全边界",
            "popularRepos": "热门仓库",
            "githubDesc": "主来源，读取仓库、Release、资产与下载统计。",
            "giteeDesc": "补充国内开源生态，适合中文项目与镜像发现。",
            "gitcodeDesc": "作为搜索入口兜底，后续可接入统一多源协议。",
            "language": "语言",
            "theme": "主题",
            "about": "关于",
            "mode": "模式",
            "upstream": "只链接上游",
            "upstreamBody": "GitMarket 不托管、不镜像、不重签第三方二进制文件。",
            "hash": "校验优先",
            "hashBody": "下载前展示 Release、许可证、SHA256、签名指纹和发布者信息。",
            "privacy": "隐私克制",
            "privacyBody": "Token 仅用于 API 额度，正式版本应接入系统凭据存储。"
        ]

        let en: [String: String] = [
            "home": "Home",
            "discover": "Discover",
            "sources": "Sources",
            "security": "Security",
            "settings": "Settings",
            "tagline": "Git search market for humans and agents",
            "heroTitle": "Your Agent Git intelligence board",
            "heroBody": "Collect repositories, Releases, Skills, MCPs, safety signals, and download progress while sending packages back to upstream sources.",
            "verify": "Verify",
            "openConsole": "Open console",
            "safety": "Safety",
            "popularRepos": "Popular repositories",
            "githubDesc": "Primary source for repositories, Releases, assets, and download stats.",
            "giteeDesc": "Adds Chinese open-source projects and mirror discovery.",
            "gitcodeDesc": "Fallback search source before a unified multi-source protocol.",
            "language": "Language",
            "theme": "Theme",
            "about": "About",
            "mode": "Mode",
            "upstream": "Upstream first",
            "upstreamBody": "GitMarket does not host, mirror, or re-sign third-party binaries.",
            "hash": "Verify first",
            "hashBody": "Show Release pages, licenses, SHA256, signatures, and publisher evidence before download.",
            "privacy": "Privacy aware",
            "privacyBody": "Tokens only raise API limits; production builds should use system credential storage."
        ]

        return (self == .zh ? zh : en)[key] ?? key
    }
}

enum GitMarketTheme: String, CaseIterable, Identifiable {
    case happyCat
    case meAgent
    case cleanBlue
    case warmLaunch
    case launch
    case aurora
    case sakura
    case graphite
    case ocean

    var id: String { rawValue }

    func label(language: GitMarketLanguage) -> String {
        switch (self, language) {
        case (.happyCat, .zh): return "GitMarket 橙绿"
        case (.meAgent, .zh): return "ME Agent"
        case (.cleanBlue, .zh): return "清爽蓝白"
        case (.warmLaunch, .zh): return "暖色卡片"
        case (.launch, .zh): return "橙色发布"
        case (.aurora, .zh): return "极光深色"
        case (.sakura, .zh): return "樱粉产品"
        case (.graphite, .zh): return "石墨专业"
        case (.ocean, .zh): return "海盐蓝"
        case (.happyCat, .en): return "GitMarket Orange"
        case (.meAgent, .en): return "ME Agent"
        case (.cleanBlue, .en): return "Clean Blue"
        case (.warmLaunch, .en): return "Warm Cards"
        case (.launch, .en): return "Launch Orange"
        case (.aurora, .en): return "Aurora"
        case (.sakura, .en): return "Sakura"
        case (.graphite, .en): return "Graphite"
        case (.ocean, .en): return "Ocean"
        }
    }

    var accent: Color {
        switch self {
        case .happyCat: return Color(red: 1.00, green: 0.54, blue: 0.08)
        case .meAgent: return Color(red: 0.20, green: 0.32, blue: 0.72)
        case .cleanBlue: return Color(red: 0.05, green: 0.48, blue: 0.82)
        case .warmLaunch: return Color(red: 0.94, green: 0.36, blue: 0.18)
        case .launch: return Color(red: 0.96, green: 0.47, blue: 0.20)
        case .aurora: return Color(red: 0.11, green: 0.82, blue: 0.63)
        case .sakura: return Color(red: 0.81, green: 0.29, blue: 0.46)
        case .graphite: return Color(red: 0.30, green: 0.60, blue: 1.00)
        case .ocean: return Color(red: 0.00, green: 0.48, blue: 0.63)
        }
    }

    var background: Color {
        switch self {
        case .happyCat: return Color(red: 1.00, green: 0.97, blue: 0.92)
        case .meAgent: return Color(red: 0.96, green: 0.97, blue: 0.99)
        case .cleanBlue: return Color(red: 0.93, green: 0.97, blue: 1.00)
        case .warmLaunch: return Color(red: 1.00, green: 0.96, blue: 0.89)
        case .launch: return Color(red: 0.04, green: 0.06, blue: 0.09)
        case .aurora: return Color(red: 0.02, green: 0.07, blue: 0.12)
        case .sakura: return Color(red: 1.00, green: 0.97, blue: 0.98)
        case .graphite: return Color(red: 0.05, green: 0.06, blue: 0.07)
        case .ocean: return Color(red: 0.94, green: 0.98, blue: 0.99)
        }
    }

    var card: Color {
        switch self {
        case .happyCat: return Color(red: 1.00, green: 0.99, blue: 0.97)
        case .launch: return Color(red: 0.10, green: 0.13, blue: 0.18)
        case .aurora: return Color(red: 0.05, green: 0.12, blue: 0.19)
        case .graphite: return Color(red: 0.09, green: 0.11, blue: 0.12)
        default: return .white
        }
    }

    var primaryText: Color {
        switch self {
        case .launch, .aurora, .graphite: return Color(red: 0.94, green: 0.98, blue: 0.96)
        case .happyCat: return Color(red: 0.12, green: 0.16, blue: 0.23)
        default: return Color(red: 0.08, green: 0.10, blue: 0.16)
        }
    }

    var secondaryText: Color {
        switch self {
        case .launch, .aurora, .graphite: return Color(red: 0.64, green: 0.72, blue: 0.72)
        case .happyCat: return Color(red: 0.42, green: 0.36, blue: 0.31)
        default: return Color(red: 0.42, green: 0.47, blue: 0.56)
        }
    }
}

struct SampleRepository: Identifiable {
    let id = UUID()
    let name: String
    let source: String
    let zhDescription: String
    let enDescription: String
    let version: String
    let asset: String

    func description(language: GitMarketLanguage) -> String {
        language == .zh ? zhDescription : enDescription
    }

    static let all: [SampleRepository] = [
        SampleRepository(
            name: "termux/termux-app",
            source: "GitHub",
            zhDescription: "Android 终端模拟器，Release 资产以 APK 为主。",
            enDescription: "Android terminal emulator with APK-focused Release assets.",
            version: "v0.119.0",
            asset: "APK"
        ),
        SampleRepository(
            name: "2dust/v2rayNG",
            source: "GitHub",
            zhDescription: "Android 代理客户端，适合展示签名和安装风险提示。",
            enDescription: "Android proxy client for signature and install-risk checks.",
            version: "v1.10.5",
            asset: "APK"
        ),
        SampleRepository(
            name: "obsproject/obs-studio",
            source: "GitHub",
            zhDescription: "跨平台桌面应用，覆盖 Windows、macOS 与 Linux 包。",
            enDescription: "Cross-platform desktop app with Windows, macOS, and Linux packages.",
            version: "31.0.2",
            asset: "EXE / DMG"
        )
    ]
}

struct SecurityNote: Identifiable {
    let id = UUID()
    let symbol: String
    let title: String
    let body: String

    static func notes(language: GitMarketLanguage) -> [SecurityNote] {
        [
            SecurityNote(symbol: "link", title: language.text("upstream"), body: language.text("upstreamBody")),
            SecurityNote(symbol: "number", title: language.text("hash"), body: language.text("hashBody")),
            SecurityNote(symbol: "lock", title: language.text("privacy"), body: language.text("privacyBody"))
        ]
    }
}
