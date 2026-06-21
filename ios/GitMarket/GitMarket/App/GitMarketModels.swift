import Foundation
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
            "overview": "概览",
            "updatedTwoMinutes": "更新于 2 分钟前",
            "safeRunning": "安全运行中",
            "repos": "仓库",
            "availableUpdates": "可用更新",
            "watchedRepos": "关注仓库",
            "quickActions": "快捷操作",
            "search": "搜索",
            "pendingVerify": "待校验",
            "subscribe": "订阅",
            "all": "全部",
            "hotRepositories": "热门仓库",
            "searchPlaceholder": "搜索仓库、发布版本或关键字",
            "filter": "筛选",
            "apps": "应用",
            "tools": "工具",
            "games": "游戏",
            "libraries": "库",
            "connected": "已连接",
            "securityVerify": "安全验证",
            "uploadedTwoHours": "上传于 2 小时前",
            "unverifiedAsset": "未验证的资产",
            "unverifiedBody": "以下资产尚未通过完整验证，下载前请谨慎确认。",
            "securityFootnote": "安全验证基于社区贡献的数据，结果仅供参考。",
            "about": "关于",
            "language": "语言",
            "theme": "主题",
            "mode": "模式",
            "shaCheck": "SHA256 校验",
            "shaDetail": "3fbc9e0b7d2c4e3f...a9d1b7e5f0c8d2e1",
            "signatureCheck": "签名验证",
            "signatureDetail": "开发者签名有效 (v0.119.0)",
            "licenseCheck": "许可证",
            "licenseDetail": "OSI 认可的开源许可证",
            "upstreamCheck": "上游来源",
            "upstreamDetail": "https://github.com/termux/termux-app",
            "publisherCheck": "发布者信任",
            "publisherDetail": "该发布者历史记录良好",
            "safetyCheck": "安全状态",
            "safetyDetail": "未检测到已知风险",
            "matched": "匹配",
            "valid": "有效",
            "clean": "干净",
            "highTrust": "高可信",
            "mitLicense": "MIT License",
            "verified": "已验证",
            "release": "Release",
            "versionCenter": "版本中心",
            "localVersion": "当前版本",
            "onlineVersion": "在线版本",
            "checkingVersion": "检查中",
            "checkFailed": "检查失败",
            "downloadLatest": "一键下载最新版",
            "releaseUpToDate": "已是最新",
            "newVersionAvailable": "发现新版",
            "installNoteFresh": "在线版本与当前版本一致。下载按钮会打开最新 Release，方便重新获取安装包。",
            "installNoteUpdate": "检测到线上有新版本。Android 同包名同签名可覆盖安装；桌面端可由 launcher 替换；iOS 请使用 TestFlight、App Store 或 simulator 包。",
            "installNoteChecking": "正在读取线上版本 JSON，用于判断是否需要更新。",
            "githubDesc": "主来源，读取仓库、Release、资产与下载统计。",
            "giteeDesc": "补充国内开源生态，适合中文项目与镜像发现。",
            "gitcodeDesc": "作为搜索入口兜底，后续可接入统一多源协议。"
        ]

        let en: [String: String] = [
            "home": "Home",
            "discover": "Discover",
            "sources": "Sources",
            "security": "Security",
            "settings": "Settings",
            "overview": "Overview",
            "updatedTwoMinutes": "Updated 2 min ago",
            "safeRunning": "Safe",
            "repos": "Repos",
            "availableUpdates": "Updates",
            "watchedRepos": "Watching",
            "quickActions": "Quick Actions",
            "search": "Search",
            "pendingVerify": "Pending",
            "subscribe": "Subscribe",
            "all": "All",
            "hotRepositories": "Hot Repositories",
            "searchPlaceholder": "Search repositories, releases, or keywords",
            "filter": "Filter",
            "apps": "Apps",
            "tools": "Tools",
            "games": "Games",
            "libraries": "Libs",
            "connected": "Connected",
            "securityVerify": "Security Verify",
            "uploadedTwoHours": "uploaded 2 hours ago",
            "unverifiedAsset": "Unverified asset",
            "unverifiedBody": "The asset below has not completed verification. Review it carefully before downloading.",
            "securityFootnote": "Verification uses community contributed data. Results are references only.",
            "about": "About",
            "language": "Language",
            "theme": "Theme",
            "mode": "Mode",
            "shaCheck": "SHA256 Check",
            "shaDetail": "3fbc9e0b7d2c4e3f...a9d1b7e5f0c8d2e1",
            "signatureCheck": "Signature",
            "signatureDetail": "Developer signature is valid (v0.119.0)",
            "licenseCheck": "License",
            "licenseDetail": "OSI approved open-source license",
            "upstreamCheck": "Upstream",
            "upstreamDetail": "https://github.com/termux/termux-app",
            "publisherCheck": "Publisher Trust",
            "publisherDetail": "Publisher has a healthy release history",
            "safetyCheck": "Safety Status",
            "safetyDetail": "No known risk detected",
            "matched": "Matched",
            "valid": "Valid",
            "clean": "Clean",
            "highTrust": "Trusted",
            "mitLicense": "MIT License",
            "verified": "Verified",
            "release": "Release",
            "versionCenter": "Version Center",
            "localVersion": "Current",
            "onlineVersion": "Online",
            "checkingVersion": "Checking",
            "checkFailed": "Check failed",
            "downloadLatest": "Download Latest",
            "releaseUpToDate": "Up to date",
            "newVersionAvailable": "Update available",
            "installNoteFresh": "The online version matches this build. The download button opens the latest Release for a fresh installer.",
            "installNoteUpdate": "A newer online version is available. Android can overwrite with the same package and signature; desktop launchers can replace binaries; iOS should use TestFlight, App Store, MDM, or simulator packages.",
            "installNoteChecking": "Reading the online version JSON to decide whether this app needs an update.",
            "githubDesc": "Primary source for repositories, releases, assets, and download stats.",
            "giteeDesc": "Adds Chinese open-source projects and mirror discovery.",
            "gitcodeDesc": "Fallback search source before a unified multi-source protocol."
        ]

        return (self == .zh ? zh : en)[key] ?? key
    }
}

@MainActor
final class AppVersionStore: ObservableObject {
    @Published private(set) var manifest: AppUpdateManifest?
    @Published private(set) var isLoading = false
    @Published private(set) var errorMessage: String?

    private static let manifestURL = URL(string: "https://harzva.github.io/GitReleaseMarket/app-update.json")!

    var currentVersion: String {
        Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "0.3.7"
    }

    var currentBuild: String {
        Bundle.main.infoDictionary?["CFBundleVersion"] as? String ?? "22"
    }

    var latestVersion: String? {
        manifest?.latestVersion
    }

    var isUpdateAvailable: Bool {
        guard let latestVersion else { return false }
        return compareVersions(latestVersion, currentVersion) == .orderedDescending
    }

    var preferredDownloadURL: URL? {
        let preferred = manifest?.platforms?["iosSimulator"]?.downloadUrl ?? manifest?.releaseUrl
        return preferred.flatMap(URL.init(string:)) ?? URL(string: "https://github.com/Harzva/GitReleaseMarket/releases/latest")
    }

    func refresh(force: Bool = false) async {
        if isLoading || (manifest != nil && !force) {
            return
        }

        isLoading = true
        errorMessage = nil
        defer { isLoading = false }

        do {
            let (data, _) = try await URLSession.shared.data(from: Self.manifestURL)
            manifest = try JSONDecoder().decode(AppUpdateManifest.self, from: data)
        } catch {
            errorMessage = error.localizedDescription
        }
    }

    func onlineVersionText(language: GitMarketLanguage) -> String {
        if let latestVersion {
            return latestVersion
        }
        return isLoading ? language.text("checkingVersion") : language.text("checkFailed")
    }

    func statusText(language: GitMarketLanguage) -> String {
        if isLoading {
            return language.text("checkingVersion")
        }
        if errorMessage != nil && manifest == nil {
            return language.text("checkFailed")
        }
        return isUpdateAvailable ? language.text("newVersionAvailable") : language.text("releaseUpToDate")
    }

    func installNote(language: GitMarketLanguage) -> String {
        if isLoading || manifest == nil {
            return language.text("installNoteChecking")
        }
        return isUpdateAvailable ? language.text("installNoteUpdate") : language.text("installNoteFresh")
    }

    private func compareVersions(_ lhs: String, _ rhs: String) -> ComparisonResult {
        let leftParts = lhs.split(separator: ".").map { Int($0) ?? 0 }
        let rightParts = rhs.split(separator: ".").map { Int($0) ?? 0 }
        let count = max(leftParts.count, rightParts.count)

        for index in 0..<count {
            let left = index < leftParts.count ? leftParts[index] : 0
            let right = index < rightParts.count ? rightParts[index] : 0
            if left > right { return .orderedDescending }
            if left < right { return .orderedAscending }
        }

        return .orderedSame
    }
}

struct AppUpdateManifest: Decodable {
    let schemaVersion: Int
    let appId: String
    let name: String
    let latestVersion: String
    let latestTag: String
    let releaseUrl: String
    let publishedAt: String
    let platforms: [String: AppPlatformUpdate]?
}

struct AppPlatformUpdate: Decodable {
    let version: String
    let downloadUrl: String
    let installMode: String
    let packageId: String?
    let bundleId: String?
}

enum GitMarketTheme: String, CaseIterable, Identifiable {
    case meAgent
    case cleanBlue
    case warmLaunch
    case darkLab

    var id: String { rawValue }

    func label(language: GitMarketLanguage) -> String {
        switch (self, language) {
        case (.meAgent, .zh): return "舒适蓝白"
        case (.cleanBlue, .zh): return "清爽蓝白"
        case (.warmLaunch, .zh): return "暖色发布"
        case (.darkLab, .zh): return "深色实验室"
        case (.meAgent, .en): return "Comfort Blue"
        case (.cleanBlue, .en): return "Clean Blue"
        case (.warmLaunch, .en): return "Warm Launch"
        case (.darkLab, .en): return "Dark Lab"
        }
    }

    var background: Color {
        switch self {
        case .meAgent: return Color(red: 0.975, green: 0.980, blue: 0.992)
        case .cleanBlue: return Color(red: 0.955, green: 0.980, blue: 1.000)
        case .warmLaunch: return Color(red: 1.000, green: 0.975, blue: 0.940)
        case .darkLab: return Color(red: 0.055, green: 0.066, blue: 0.080)
        }
    }

    var surface: Color {
        switch self {
        case .darkLab: return Color(red: 0.100, green: 0.120, blue: 0.145)
        default: return .white
        }
    }

    var subtleSurface: Color {
        switch self {
        case .darkLab: return Color(red: 0.130, green: 0.155, blue: 0.180)
        default: return Color(red: 0.945, green: 0.960, blue: 0.980)
        }
    }

    var accent: Color {
        switch self {
        case .meAgent: return Color(red: 0.040, green: 0.365, blue: 0.850)
        case .cleanBlue: return Color(red: 0.020, green: 0.460, blue: 0.820)
        case .warmLaunch: return Color(red: 0.900, green: 0.355, blue: 0.130)
        case .darkLab: return Color(red: 0.300, green: 0.780, blue: 0.560)
        }
    }

    var success: Color {
        Color(red: 0.110, green: 0.680, blue: 0.380)
    }

    var warning: Color {
        Color(red: 0.960, green: 0.540, blue: 0.100)
    }

    var primaryText: Color {
        switch self {
        case .darkLab: return Color(red: 0.935, green: 0.955, blue: 0.975)
        default: return Color(red: 0.070, green: 0.090, blue: 0.130)
        }
    }

    var secondaryText: Color {
        switch self {
        case .darkLab: return Color(red: 0.690, green: 0.735, blue: 0.780)
        default: return Color(red: 0.365, green: 0.420, blue: 0.500)
        }
    }

    var mutedText: Color {
        switch self {
        case .darkLab: return Color(red: 0.500, green: 0.560, blue: 0.620)
        default: return Color(red: 0.565, green: 0.615, blue: 0.685)
        }
    }

    var hairline: Color {
        switch self {
        case .darkLab: return Color.white.opacity(0.08)
        default: return Color(red: 0.870, green: 0.900, blue: 0.940)
        }
    }

    var shadow: Color {
        switch self {
        case .darkLab: return Color.black.opacity(0.30)
        default: return Color.black.opacity(0.045)
        }
    }
}

enum ReleaseFilter: String, CaseIterable, Identifiable {
    case all
    case apps
    case tools
    case games
    case libraries

    var id: String { rawValue }

    func title(language: GitMarketLanguage) -> String {
        language.text(rawValue)
    }
}

enum VerificationState {
    case verified
    case pending

    func title(language: GitMarketLanguage) -> String {
        switch self {
        case .verified: return language.text("verified")
        case .pending: return language.text("pendingVerify")
        }
    }

    func color(theme: GitMarketTheme) -> Color {
        switch self {
        case .verified: return theme.success
        case .pending: return theme.warning
        }
    }

    func fill(theme: GitMarketTheme) -> Color {
        color(theme: theme).opacity(0.12)
    }
}

struct ReleaseAsset: Identifiable {
    let id = UUID()
    let repository: String
    let name: String
    let zhDescription: String
    let enDescription: String
    let version: String
    let source: String
    let platform: String
    let assetType: String
    let size: String
    let zhUpdated: String
    let enUpdated: String
    let stars: String
    let forks: String
    let symbol: String
    let iconColor: Color
    let verification: VerificationState
    let filter: ReleaseFilter

    func description(language: GitMarketLanguage) -> String {
        language == .zh ? zhDescription : enDescription
    }

    func updated(language: GitMarketLanguage) -> String {
        language == .zh ? zhUpdated : enUpdated
    }

    static let featured: [ReleaseAsset] = [
        ReleaseAsset(
            repository: "termux/termux-app",
            name: "termux-app_v0.119.0.apk",
            zhDescription: "Android 终端模拟器和 Linux 环境",
            enDescription: "Android terminal emulator and Linux environment",
            version: "0.119.0",
            source: "GitHub",
            platform: "APK",
            assetType: "APK",
            size: "24.8 MB",
            zhUpdated: "2 小时前",
            enUpdated: "2h ago",
            stars: "45.2k",
            forks: "7.1k",
            symbol: "terminal",
            iconColor: .black,
            verification: .verified,
            filter: .apps
        ),
        ReleaseAsset(
            repository: "2dust/v2rayNG",
            name: "v2rayNG_1.8.23.apk",
            zhDescription: "V2Ray 的 Android 图形客户端",
            enDescription: "Android GUI client for V2Ray",
            version: "1.8.23",
            source: "GitHub",
            platform: "APK",
            assetType: "APK",
            size: "17.5 MB",
            zhUpdated: "5 小时前",
            enUpdated: "5h ago",
            stars: "22.1k",
            forks: "5.0k",
            symbol: "bolt.fill",
            iconColor: .black,
            verification: .verified,
            filter: .apps
        ),
        ReleaseAsset(
            repository: "obsproject/obs-studio",
            name: "OBS-Studio-31.0.2-Windows.exe",
            zhDescription: "免费开源的直播录制软件",
            enDescription: "Free and open-source recording and streaming software",
            version: "30.1.2",
            source: "GitHub",
            platform: "Windows",
            assetType: "EXE",
            size: "134 MB",
            zhUpdated: "昨天",
            enUpdated: "Yesterday",
            stars: "36.0k",
            forks: "11.2k",
            symbol: "record.circle",
            iconColor: .black,
            verification: .verified,
            filter: .apps
        )
    ]

    static let discover: [ReleaseAsset] = featured + [
        ReleaseAsset(
            repository: "JetBrains/IntelliJ IDEA",
            name: "ideaIU-2024.1.2.dmg",
            zhDescription: "专业 Java 与 Kotlin IDE",
            enDescription: "Professional Java and Kotlin IDE",
            version: "2024.1.2",
            source: "GitHub",
            platform: "macOS",
            assetType: "DMG",
            size: "1.1 GB",
            zhUpdated: "昨天",
            enUpdated: "Yesterday",
            stars: "12.4k",
            forks: "2.1k",
            symbol: "square.stack.3d.up.fill",
            iconColor: Color(red: 0.890, green: 0.160, blue: 0.250),
            verification: .verified,
            filter: .tools
        ),
        ReleaseAsset(
            repository: "neovim/neovim",
            name: "nvim-win64.msi",
            zhDescription: "高扩展性的现代 Vim 编辑器",
            enDescription: "Hyperextensible Vim-based text editor",
            version: "v0.10.0",
            source: "GitHub",
            platform: "Windows",
            assetType: "EXE",
            size: "28.6 MB",
            zhUpdated: "2 天前",
            enUpdated: "2d ago",
            stars: "78.0k",
            forks: "5.4k",
            symbol: "n.square.fill",
            iconColor: Color(red: 0.180, green: 0.700, blue: 0.290),
            verification: .verified,
            filter: .tools
        ),
        ReleaseAsset(
            repository: "apple/axlearn",
            name: "axlearn-0.1.0.zip",
            zhDescription: "机器学习研究训练框架",
            enDescription: "Machine learning research training framework",
            version: "v0.1.0",
            source: "GitHub",
            platform: "Python",
            assetType: "ZIP",
            size: "3.2 MB",
            zhUpdated: "2 天前",
            enUpdated: "2d ago",
            stars: "3.8k",
            forks: "430",
            symbol: "apple.logo",
            iconColor: .black,
            verification: .pending,
            filter: .libraries
        ),
        ReleaseAsset(
            repository: "microsoft/PowerToys",
            name: "PowerToysSetup-0.81.0-x64.exe",
            zhDescription: "Windows 高级用户效率工具集",
            enDescription: "Utilities for Windows power users",
            version: "v0.81.0",
            source: "GitHub",
            platform: "Windows",
            assetType: "EXE",
            size: "99.1 MB",
            zhUpdated: "3 天前",
            enUpdated: "3d ago",
            stars: "104k",
            forks: "6.4k",
            symbol: "window.vertical.closed",
            iconColor: Color(red: 0.040, green: 0.365, blue: 0.850),
            verification: .verified,
            filter: .tools
        )
    ]
}

struct SourceProvider: Identifiable {
    let id = UUID()
    let name: String
    let symbol: String
    let color: Color
    let zhDescription: String
    let enDescription: String

    func description(language: GitMarketLanguage) -> String {
        language == .zh ? zhDescription : enDescription
    }

    func status(language: GitMarketLanguage) -> String {
        language.text("connected")
    }

    static let all: [SourceProvider] = [
        SourceProvider(
            name: "GitHub",
            symbol: "network",
            color: .black,
            zhDescription: "主来源，读取仓库、Release、资产与下载统计。",
            enDescription: "Primary source for repositories, releases, assets, and download stats."
        ),
        SourceProvider(
            name: "Gitee",
            symbol: "g.circle.fill",
            color: Color(red: 0.820, green: 0.120, blue: 0.100),
            zhDescription: "补充国内开源生态，适合中文项目与镜像发现。",
            enDescription: "Adds Chinese open-source projects and mirror discovery."
        ),
        SourceProvider(
            name: "GitCode",
            symbol: "g.circle.fill",
            color: Color(red: 0.900, green: 0.180, blue: 0.120),
            zhDescription: "作为搜索入口兜底，后续可接入统一多源协议。",
            enDescription: "Fallback search source before a unified multi-source protocol."
        )
    ]
}

enum SecurityState {
    case pass
    case warn

    var symbol: String {
        switch self {
        case .pass: return "checkmark"
        case .warn: return "exclamationmark"
        }
    }

    func color(theme: GitMarketTheme) -> Color {
        switch self {
        case .pass: return theme.success
        case .warn: return theme.warning
        }
    }
}

struct SecurityCheck: Identifiable {
    let id = UUID()
    let titleKey: String
    let detailKey: String
    let valueKey: String
    let state: SecurityState
    let disclosesExternal: Bool

    func title(language: GitMarketLanguage) -> String {
        language.text(titleKey)
    }

    func detail(language: GitMarketLanguage) -> String {
        language.text(detailKey)
    }

    func value(language: GitMarketLanguage) -> String {
        language.text(valueKey)
    }

    static let checks: [SecurityCheck] = [
        SecurityCheck(titleKey: "shaCheck", detailKey: "shaDetail", valueKey: "matched", state: .pass, disclosesExternal: false),
        SecurityCheck(titleKey: "signatureCheck", detailKey: "signatureDetail", valueKey: "valid", state: .pass, disclosesExternal: false),
        SecurityCheck(titleKey: "licenseCheck", detailKey: "licenseDetail", valueKey: "mitLicense", state: .pass, disclosesExternal: false),
        SecurityCheck(titleKey: "upstreamCheck", detailKey: "upstreamDetail", valueKey: "GitHub", state: .pass, disclosesExternal: true),
        SecurityCheck(titleKey: "publisherCheck", detailKey: "publisherDetail", valueKey: "highTrust", state: .pass, disclosesExternal: false),
        SecurityCheck(titleKey: "safetyCheck", detailKey: "safetyDetail", valueKey: "clean", state: .pass, disclosesExternal: false)
    ]
}
