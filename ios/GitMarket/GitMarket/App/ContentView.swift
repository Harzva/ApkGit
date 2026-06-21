import SwiftUI

struct ContentView: View {
    @State private var selectedTab: GitMarketTab = .home
    @State private var language: GitMarketLanguage = .zh
    @State private var theme: GitMarketTheme = .meAgent

    var body: some View {
        TabView(selection: $selectedTab) {
            HomeView(language: language, theme: theme, selectedTab: $selectedTab)
                .tabItem { Label(text("home"), systemImage: "house.fill") }
                .tag(GitMarketTab.home)

            DiscoverView(language: language, theme: theme)
                .tabItem { Label(text("discover"), systemImage: "magnifyingglass") }
                .tag(GitMarketTab.discover)

            SourcesView(language: language, theme: theme)
                .tabItem { Label(text("sources"), systemImage: "square.stack.3d.up.fill") }
                .tag(GitMarketTab.sources)

            SecurityView(language: language, theme: theme)
                .tabItem { Label(text("security"), systemImage: "checkmark.shield.fill") }
                .tag(GitMarketTab.security)

            SettingsView(language: $language, theme: $theme)
                .tabItem { Label(text("settings"), systemImage: "gearshape.fill") }
                .tag(GitMarketTab.settings)
        }
        .tint(theme.accent)
    }

    private func text(_ key: String) -> String {
        language.text(key)
    }
}

private struct HomeView: View {
    let language: GitMarketLanguage
    let theme: GitMarketTheme
    @Binding var selectedTab: GitMarketTab

    private let quickColumns = Array(repeating: GridItem(.flexible(), spacing: 10), count: 4)

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 22) {
                    HomeTopBar(language: language, theme: theme)

                    DashboardSectionHeader(
                        title: language.text("overview"),
                        detail: language.text("updatedTwoMinutes"),
                        theme: theme
                    )

                    metrics
                    quickActions
                    sourceStrip
                    popularRepositories
                }
                .padding(.horizontal, 16)
                .padding(.top, 12)
                .padding(.bottom, 28)
            }
            .background(theme.background.ignoresSafeArea())
            .navigationBarHidden(true)
        }
    }

    private var metrics: some View {
        HStack(spacing: 10) {
            MetricTile(symbol: "shippingbox", value: "128", label: language.text("repos"), color: theme.accent, theme: theme)
            MetricTile(symbol: "arrow.down.circle", value: "1,246", label: language.text("availableUpdates"), color: theme.success, theme: theme)
            MetricTile(symbol: "star", value: "42", label: language.text("watchedRepos"), color: theme.warning, theme: theme)
        }
    }

    private var quickActions: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text(language.text("quickActions"))
                .font(.headline)
                .foregroundStyle(theme.primaryText)

            LazyVGrid(columns: quickColumns, spacing: 10) {
                QuickActionButton(
                    title: language.text("search"),
                    symbol: "magnifyingglass",
                    badge: nil,
                    theme: theme
                ) {
                    selectedTab = .discover
                }

                QuickActionButton(
                    title: language.text("pendingVerify"),
                    symbol: "shield",
                    badge: "3",
                    theme: theme
                ) {
                    selectedTab = .security
                }

                QuickActionButton(
                    title: language.text("subscribe"),
                    symbol: "bookmark",
                    badge: nil,
                    theme: theme
                ) {}

                QuickActionButton(
                    title: language.text("settings"),
                    symbol: "gearshape",
                    badge: nil,
                    theme: theme
                ) {
                    selectedTab = .settings
                }
            }
        }
    }

    private var sourceStrip: some View {
        VStack(alignment: .leading, spacing: 12) {
            SectionTitle(title: language.text("sources"), action: language.text("all"), theme: theme)

            HStack(spacing: 10) {
                ForEach(SourceProvider.all) { source in
                    SourceChip(source: source, language: language, theme: theme)
                }
            }
        }
    }

    private var popularRepositories: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text(language.text("hotRepositories"))
                .font(.headline)
                .foregroundStyle(theme.primaryText)

            VStack(spacing: 10) {
                ForEach(ReleaseAsset.featured) { asset in
                    ReleaseCompactCard(asset: asset, language: language, theme: theme)
                }
            }
        }
    }
}

private struct DiscoverView: View {
    let language: GitMarketLanguage
    let theme: GitMarketTheme

    @State private var query = ""
    @State private var selectedFilter: ReleaseFilter = .all

    private var visibleAssets: [ReleaseAsset] {
        ReleaseAsset.discover.filter { asset in
            let matchesFilter = selectedFilter == .all || asset.filter == selectedFilter
            let matchesQuery = query.isEmpty || asset.name.localizedCaseInsensitiveContains(query) || asset.repository.localizedCaseInsensitiveContains(query)
            return matchesFilter && matchesQuery
        }
    }

    var body: some View {
        NavigationStack {
            VStack(spacing: 12) {
                searchBar
                filterTabs

                ScrollView {
                    LazyVStack(spacing: 10) {
                        ForEach(visibleAssets) { asset in
                            ReleaseResultCard(asset: asset, language: language, theme: theme)
                        }
                    }
                    .padding(.horizontal, 16)
                    .padding(.bottom, 28)
                }
                .scrollIndicators(.hidden)
            }
            .padding(.top, 12)
            .background(theme.background.ignoresSafeArea())
            .navigationBarHidden(true)
        }
    }

    private var searchBar: some View {
        HStack(spacing: 10) {
            HStack(spacing: 8) {
                Image(systemName: "magnifyingglass")
                    .foregroundStyle(theme.mutedText)

                TextField(language.text("searchPlaceholder"), text: $query)
                    .textInputAutocapitalization(.never)
                    .autocorrectionDisabled()
                    .font(.subheadline)
            }
            .padding(.horizontal, 12)
            .frame(height: 42)
            .background(theme.surface)
            .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
            .overlay {
                RoundedRectangle(cornerRadius: 8, style: .continuous)
                    .stroke(theme.hairline, lineWidth: 1)
            }

            Button {} label: {
                Image(systemName: "slider.horizontal.3")
                    .font(.system(size: 18, weight: .semibold))
                    .frame(width: 42, height: 42)
                    .foregroundStyle(theme.primaryText)
                    .background(theme.surface)
                    .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
                    .overlay {
                        RoundedRectangle(cornerRadius: 8, style: .continuous)
                            .stroke(theme.hairline, lineWidth: 1)
                    }
            }
        }
        .padding(.horizontal, 16)
    }

    private var filterTabs: some View {
        Picker(language.text("filter"), selection: $selectedFilter) {
            ForEach(ReleaseFilter.allCases) { filter in
                Text(filter.title(language: language)).tag(filter)
            }
        }
        .pickerStyle(.segmented)
        .padding(.horizontal, 16)
    }
}

private struct SourcesView: View {
    let language: GitMarketLanguage
    let theme: GitMarketTheme

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 14) {
                    HomeTopBar(language: language, theme: theme, compactTitle: language.text("sources"))

                    ForEach(SourceProvider.all) { source in
                        SourceDetailRow(source: source, language: language, theme: theme)
                    }
                }
                .padding(16)
            }
            .background(theme.background.ignoresSafeArea())
            .navigationBarHidden(true)
        }
    }
}

private struct SecurityView: View {
    let language: GitMarketLanguage
    let theme: GitMarketTheme

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 16) {
                    securityHeader
                    verificationList
                    pendingAsset
                    securityFootnote
                }
                .padding(16)
                .padding(.bottom, 28)
            }
            .background(theme.background.ignoresSafeArea())
            .navigationBarHidden(true)
        }
    }

    private var securityHeader: some View {
        VStack(alignment: .leading, spacing: 14) {
            HStack {
                Button {} label: {
                    Image(systemName: "chevron.left")
                        .font(.system(size: 18, weight: .semibold))
                        .foregroundStyle(theme.primaryText)
                }

                Spacer()

                Text(language.text("securityVerify"))
                    .font(.headline)
                    .foregroundStyle(theme.primaryText)

                Spacer()

                Button {} label: {
                    Image(systemName: "square.and.arrow.up")
                        .font(.system(size: 18, weight: .semibold))
                        .foregroundStyle(theme.primaryText)
                }
            }

            HStack(spacing: 14) {
                AppIconTile(symbol: "terminal", color: .black, theme: theme)

                VStack(alignment: .leading, spacing: 5) {
                    Text("termux-app_v0.119.0.apk")
                        .font(.headline)
                        .foregroundStyle(theme.primaryText)
                        .lineLimit(1)
                        .minimumScaleFactor(0.82)

                    Text("24.8 MB · \(language.text("uploadedTwoHours"))")
                        .font(.subheadline)
                        .foregroundStyle(theme.secondaryText)

                    HStack(spacing: 10) {
                        Label("termux/termux-app", systemImage: "shippingbox")
                        Label("GitHub", systemImage: "network")
                    }
                    .font(.caption)
                    .foregroundStyle(theme.mutedText)
                }
            }
        }
    }

    private var verificationList: some View {
        VStack(spacing: 0) {
            ForEach(SecurityCheck.checks) { check in
                SecurityCheckRow(check: check, language: language, theme: theme)

                if check.id != SecurityCheck.checks.last?.id {
                    Divider()
                        .padding(.leading, 56)
                }
            }
        }
        .padding(.vertical, 6)
        .background(theme.surface)
        .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
        .overlay {
            RoundedRectangle(cornerRadius: 8, style: .continuous)
                .stroke(theme.hairline, lineWidth: 1)
        }
    }

    private var pendingAsset: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack(spacing: 10) {
                Image(systemName: "exclamationmark.circle")
                    .font(.title3)
                    .foregroundStyle(theme.warning)

                Text(language.text("unverifiedAsset"))
                    .font(.headline)
                    .foregroundStyle(theme.warning)

                Spacer()

                Image(systemName: "chevron.up")
                    .font(.caption.weight(.bold))
                    .foregroundStyle(theme.secondaryText)
            }

            Text(language.text("unverifiedBody"))
                .font(.subheadline)
                .foregroundStyle(theme.secondaryText)

            HStack(spacing: 12) {
                VStack(alignment: .leading, spacing: 6) {
                    Text("termux-app_v0.119.0_arm7.apk")
                        .font(.subheadline.weight(.semibold))
                        .foregroundStyle(theme.primaryText)
                        .lineLimit(1)

                    Text("22.1 MB · ARMv7")
                        .font(.caption)
                        .foregroundStyle(theme.secondaryText)
                }

                Spacer()

                StatusBadge(title: language.text("pendingVerify"), color: theme.warning, fill: theme.warning.opacity(0.12))

                DownloadIconButton(theme: theme)
            }
            .padding(12)
            .background(theme.surface)
            .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
            .overlay {
                RoundedRectangle(cornerRadius: 8, style: .continuous)
                    .stroke(theme.warning.opacity(0.28), lineWidth: 1)
            }
        }
        .padding(14)
        .background(theme.warning.opacity(0.08))
        .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
        .overlay {
            RoundedRectangle(cornerRadius: 8, style: .continuous)
                .stroke(theme.warning.opacity(0.32), lineWidth: 1)
        }
    }

    private var securityFootnote: some View {
        HStack(spacing: 8) {
            Image(systemName: "checkmark.shield.fill")
                .foregroundStyle(theme.success)

            Text(language.text("securityFootnote"))
                .font(.caption)
                .foregroundStyle(theme.secondaryText)
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 9)
        .background(theme.subtleSurface)
        .clipShape(Capsule())
    }
}

private struct SettingsView: View {
    @Binding var language: GitMarketLanguage
    @Binding var theme: GitMarketTheme

    var body: some View {
        NavigationStack {
            Form {
                Picker(language.text("language"), selection: $language) {
                    ForEach(GitMarketLanguage.allCases) { item in
                        Text(item.label).tag(item)
                    }
                }

                Picker(language.text("theme"), selection: $theme) {
                    ForEach(GitMarketTheme.allCases) { item in
                        Text(item.label(language: language)).tag(item)
                    }
                }

                Section(language.text("about")) {
                    LabeledContent("GitMarket", value: "0.1.7 iOS Preview")
                    LabeledContent(language.text("mode"), value: "SwiftUI Native")
                }
            }
            .navigationTitle(language.text("settings"))
        }
    }
}

private struct HomeTopBar: View {
    let language: GitMarketLanguage
    let theme: GitMarketTheme
    var compactTitle: String?

    var body: some View {
        HStack(spacing: 12) {
            if let compactTitle {
                Text(compactTitle)
                    .font(.title2.bold())
                    .foregroundStyle(theme.primaryText)
            } else {
                HStack(spacing: 0) {
                    Text("Git")
                        .font(.title2.bold())
                        .foregroundStyle(theme.primaryText)

                    Text("Market")
                        .font(.title2.bold())
                        .foregroundStyle(theme.accent)
                }
            }

            Spacer()

            HStack(spacing: 5) {
                Image(systemName: "shield.lefthalf.filled")
                    .font(.caption)
                Text(language.text("safeRunning"))
                    .font(.caption.weight(.semibold))
            }
            .foregroundStyle(theme.success)
            .padding(.horizontal, 10)
            .padding(.vertical, 6)
            .background(theme.success.opacity(0.12))
            .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))

            Button {} label: {
                Image(systemName: "bell")
                    .font(.system(size: 18, weight: .medium))
                    .foregroundStyle(theme.primaryText)
            }
        }
    }
}

private struct DashboardSectionHeader: View {
    let title: String
    let detail: String
    let theme: GitMarketTheme

    var body: some View {
        HStack(spacing: 6) {
            Text(title)
                .font(.headline)
                .foregroundStyle(theme.primaryText)

            Text(detail)
                .font(.caption)
                .foregroundStyle(theme.mutedText)

            Image(systemName: "arrow.clockwise")
                .font(.caption)
                .foregroundStyle(theme.mutedText)
        }
    }
}

private struct SectionTitle: View {
    let title: String
    let action: String
    let theme: GitMarketTheme

    var body: some View {
        HStack {
            Text(title)
                .font(.headline)
                .foregroundStyle(theme.primaryText)

            Spacer()

            HStack(spacing: 4) {
                Text(action)
                Image(systemName: "chevron.right")
            }
            .font(.caption)
            .foregroundStyle(theme.secondaryText)
        }
    }
}

private struct MetricTile: View {
    let symbol: String
    let value: String
    let label: String
    let color: Color
    let theme: GitMarketTheme

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Image(systemName: symbol)
                .font(.system(size: 18, weight: .semibold))
                .foregroundStyle(color)
                .frame(width: 30, height: 30)
                .background(color.opacity(0.12))
                .clipShape(Circle())

            Text(value)
                .font(.title3.bold())
                .foregroundStyle(theme.primaryText)
                .lineLimit(1)
                .minimumScaleFactor(0.8)

            Text(label)
                .font(.caption)
                .foregroundStyle(theme.secondaryText)
                .lineLimit(1)
                .minimumScaleFactor(0.8)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(12)
        .background(theme.surface)
        .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
        .overlay {
            RoundedRectangle(cornerRadius: 8, style: .continuous)
                .stroke(theme.hairline, lineWidth: 1)
        }
    }
}

private struct QuickActionButton: View {
    let title: String
    let symbol: String
    let badge: String?
    let theme: GitMarketTheme
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 9) {
                ZStack(alignment: .topTrailing) {
                    Image(systemName: symbol)
                        .font(.system(size: 24, weight: .regular))
                        .foregroundStyle(theme.primaryText)
                        .frame(width: 44, height: 44)

                    if let badge {
                        Text(badge)
                            .font(.caption2.bold())
                            .foregroundStyle(.white)
                            .frame(width: 18, height: 18)
                            .background(theme.warning)
                            .clipShape(Circle())
                            .offset(x: 7, y: -5)
                    }
                }

                Text(title)
                    .font(.caption)
                    .foregroundStyle(theme.secondaryText)
                    .lineLimit(1)
                    .minimumScaleFactor(0.8)
            }
            .frame(maxWidth: .infinity)
            .frame(height: 84)
            .background(theme.surface)
            .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
            .overlay {
                RoundedRectangle(cornerRadius: 8, style: .continuous)
                    .stroke(theme.hairline, lineWidth: 1)
            }
        }
        .buttonStyle(.plain)
    }
}

private struct SourceChip: View {
    let source: SourceProvider
    let language: GitMarketLanguage
    let theme: GitMarketTheme

    var body: some View {
        HStack(spacing: 9) {
            AppIconTile(symbol: source.symbol, color: source.color, theme: theme, size: 34)

            VStack(alignment: .leading, spacing: 3) {
                Text(source.name)
                    .font(.subheadline.weight(.semibold))
                    .foregroundStyle(theme.primaryText)

                HStack(spacing: 4) {
                    Circle()
                        .fill(theme.success)
                        .frame(width: 5, height: 5)

                    Text(source.status(language: language))
                        .font(.caption2)
                        .foregroundStyle(theme.secondaryText)
                }
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(10)
        .background(theme.surface)
        .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
        .overlay {
            RoundedRectangle(cornerRadius: 8, style: .continuous)
                .stroke(theme.hairline, lineWidth: 1)
        }
    }
}

private struct ReleaseCompactCard: View {
    let asset: ReleaseAsset
    let language: GitMarketLanguage
    let theme: GitMarketTheme

    var body: some View {
        HStack(spacing: 12) {
            AppIconTile(symbol: asset.symbol, color: asset.iconColor, theme: theme)

            VStack(alignment: .leading, spacing: 6) {
                HStack {
                    Text(asset.repository)
                        .font(.subheadline.weight(.semibold))
                        .foregroundStyle(theme.primaryText)
                        .lineLimit(1)

                    Spacer()

                    Text(asset.updated(language: language))
                        .font(.caption)
                        .foregroundStyle(theme.mutedText)
                }

                Text(asset.description(language: language))
                    .font(.caption)
                    .foregroundStyle(theme.secondaryText)
                    .lineLimit(1)

                HStack(spacing: 12) {
                    Label(asset.stars, systemImage: "star")
                    Label(asset.forks, systemImage: "point.3.connected.trianglepath.dotted")
                    VersionBadge(title: asset.version, theme: theme)
                }
                .font(.caption)
                .foregroundStyle(theme.secondaryText)
            }

            Image(systemName: "checkmark.shield.fill")
                .foregroundStyle(theme.success)
        }
        .padding(10)
        .background(theme.surface)
        .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
        .overlay {
            RoundedRectangle(cornerRadius: 8, style: .continuous)
                .stroke(theme.hairline, lineWidth: 1)
        }
    }
}

private struct ReleaseResultCard: View {
    let asset: ReleaseAsset
    let language: GitMarketLanguage
    let theme: GitMarketTheme

    var body: some View {
        HStack(spacing: 12) {
            AppIconTile(symbol: asset.symbol, color: asset.iconColor, theme: theme)

            VStack(alignment: .leading, spacing: 7) {
                HStack(spacing: 8) {
                    Text(asset.repository)
                        .font(.subheadline.weight(.semibold))
                        .foregroundStyle(theme.primaryText)
                        .lineLimit(1)

                    Spacer()

                    Image(systemName: "chevron.right")
                        .font(.caption)
                        .foregroundStyle(theme.mutedText)
                }

                HStack(spacing: 6) {
                    Text(asset.version)
                    Text("·")
                    Text(asset.updated(language: language))
                    StatusBadge(title: asset.verification.title(language: language), color: asset.verification.color(theme: theme), fill: asset.verification.fill(theme: theme))
                }
                .font(.caption)
                .foregroundStyle(theme.secondaryText)

                HStack(spacing: 8) {
                    Label(asset.source, systemImage: "network")
                    VersionBadge(title: asset.platform, theme: theme)
                    VersionBadge(title: asset.assetType, theme: theme)
                    Text(asset.size)
                }
                .font(.caption)
                .foregroundStyle(theme.secondaryText)
            }

            DownloadIconButton(theme: theme)
        }
        .padding(10)
        .background(theme.surface)
        .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
        .overlay {
            RoundedRectangle(cornerRadius: 8, style: .continuous)
                .stroke(theme.hairline, lineWidth: 1)
        }
    }
}

private struct SourceDetailRow: View {
    let source: SourceProvider
    let language: GitMarketLanguage
    let theme: GitMarketTheme

    var body: some View {
        HStack(alignment: .top, spacing: 12) {
            AppIconTile(symbol: source.symbol, color: source.color, theme: theme)

            VStack(alignment: .leading, spacing: 6) {
                HStack {
                    Text(source.name)
                        .font(.headline)
                        .foregroundStyle(theme.primaryText)

                    Spacer()

                    StatusBadge(title: source.status(language: language), color: theme.success, fill: theme.success.opacity(0.12))
                }

                Text(source.description(language: language))
                    .font(.subheadline)
                    .foregroundStyle(theme.secondaryText)
            }
        }
        .padding(12)
        .background(theme.surface)
        .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
        .overlay {
            RoundedRectangle(cornerRadius: 8, style: .continuous)
                .stroke(theme.hairline, lineWidth: 1)
        }
    }
}

private struct SecurityCheckRow: View {
    let check: SecurityCheck
    let language: GitMarketLanguage
    let theme: GitMarketTheme

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: check.state.symbol)
                .font(.system(size: 16, weight: .bold))
                .foregroundStyle(.white)
                .frame(width: 30, height: 30)
                .background(check.state.color(theme: theme))
                .clipShape(Circle())

            VStack(alignment: .leading, spacing: 4) {
                Text(check.title(language: language))
                    .font(.subheadline.weight(.semibold))
                    .foregroundStyle(theme.primaryText)

                Text(check.detail(language: language))
                    .font(.caption)
                    .foregroundStyle(theme.secondaryText)
                    .lineLimit(1)
                    .truncationMode(.middle)
            }

            Spacer()

            Text(check.value(language: language))
                .font(.caption.weight(.semibold))
                .foregroundStyle(check.state.color(theme: theme))
                .lineLimit(1)

            Image(systemName: check.disclosesExternal ? "arrow.up.right.square" : "chevron.down")
                .font(.caption)
                .foregroundStyle(theme.secondaryText)
        }
        .padding(.horizontal, 14)
        .padding(.vertical, 12)
    }
}

private struct AppIconTile: View {
    let symbol: String
    let color: Color
    let theme: GitMarketTheme
    var size: CGFloat = 52

    var body: some View {
        Image(systemName: symbol)
            .font(.system(size: size * 0.42, weight: .bold))
            .foregroundStyle(color)
            .frame(width: size, height: size)
            .background(color.opacity(0.12))
            .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
            .overlay {
                RoundedRectangle(cornerRadius: 8, style: .continuous)
                    .stroke(theme.hairline, lineWidth: 1)
            }
    }
}

private struct StatusBadge: View {
    let title: String
    let color: Color
    let fill: Color

    var body: some View {
        Text(title)
            .font(.caption2.weight(.semibold))
            .foregroundStyle(color)
            .padding(.horizontal, 6)
            .padding(.vertical, 3)
            .background(fill)
            .clipShape(RoundedRectangle(cornerRadius: 5, style: .continuous))
    }
}

private struct VersionBadge: View {
    let title: String
    let theme: GitMarketTheme

    var body: some View {
        Text(title)
            .font(.caption2.weight(.medium))
            .foregroundStyle(theme.accent)
            .padding(.horizontal, 6)
            .padding(.vertical, 3)
            .background(theme.accent.opacity(0.10))
            .clipShape(RoundedRectangle(cornerRadius: 5, style: .continuous))
    }
}

private struct DownloadIconButton: View {
    let theme: GitMarketTheme

    var body: some View {
        Button {} label: {
            Image(systemName: "arrow.down.to.line")
                .font(.system(size: 16, weight: .semibold))
                .foregroundStyle(theme.accent)
                .frame(width: 34, height: 34)
                .background(theme.accent.opacity(0.08))
                .clipShape(RoundedRectangle(cornerRadius: 8, style: .continuous))
                .overlay {
                    RoundedRectangle(cornerRadius: 8, style: .continuous)
                        .stroke(theme.accent.opacity(0.18), lineWidth: 1)
                }
        }
        .buttonStyle(.plain)
    }
}
