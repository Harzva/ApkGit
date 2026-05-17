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

            ConsoleView(language: language)
                .tabItem { Label(text("discover"), systemImage: "magnifyingglass") }
                .tag(GitMarketTab.discover)

            SourcesView(language: language)
                .tabItem { Label(text("sources"), systemImage: "square.stack.3d.up.fill") }
                .tag(GitMarketTab.sources)

            SecurityView(language: language)
                .tabItem { Label(text("security"), systemImage: "checkmark.shield.fill") }
                .tag(GitMarketTab.security)

            SettingsView(language: $language, theme: $theme)
                .tabItem { Label(text("settings"), systemImage: "slider.horizontal.3") }
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

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 18) {
                    header
                    metrics
                    quickActions
                    repositoryList
                }
                .padding(20)
            }
            .background(theme.background.ignoresSafeArea())
            .navigationTitle("GitMarket")
            .toolbarBackground(theme.background, for: .navigationBar)
        }
    }

    private var header: some View {
        VStack(alignment: .leading, spacing: 14) {
            HStack(spacing: 12) {
                Image("GitMarketLogo")
                    .resizable()
                    .frame(width: 48, height: 48)
                    .clipShape(RoundedRectangle(cornerRadius: 12, style: .continuous))
                    .shadow(color: theme.accent.opacity(0.22), radius: 18, y: 8)

                VStack(alignment: .leading, spacing: 4) {
                    Text(language.text("tagline"))
                        .font(.caption.weight(.semibold))
                        .foregroundStyle(theme.accent)
                    Text(language.text("heroTitle"))
                        .font(.largeTitle.bold())
                        .foregroundStyle(theme.primaryText)
                }
            }

            Text(language.text("heroBody"))
                .font(.body)
                .foregroundStyle(theme.secondaryText)
                .lineSpacing(4)
        }
        .padding(18)
        .background(theme.card)
        .clipShape(RoundedRectangle(cornerRadius: 22, style: .continuous))
    }

    private var metrics: some View {
        HStack(spacing: 10) {
            MetricCard(value: "3", label: language.text("sources"), theme: theme)
            MetricCard(value: "12", label: "Release", theme: theme)
            MetricCard(value: "SHA", label: language.text("verify"), theme: theme)
        }
    }

    private var quickActions: some View {
        HStack(spacing: 12) {
            Button {
                selectedTab = .discover
            } label: {
                Label(language.text("openConsole"), systemImage: "magnifyingglass")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)

            Button {
                selectedTab = .security
            } label: {
                Label(language.text("safety"), systemImage: "checkmark.shield")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.bordered)
        }
        .controlSize(.large)
    }

    private var repositoryList: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text(language.text("popularRepos"))
                .font(.headline)
                .foregroundStyle(theme.primaryText)

            ForEach(SampleRepository.all) { repo in
                RepositoryCard(repo: repo, language: language, theme: theme)
            }
        }
    }
}

private struct ConsoleView: View {
    let language: GitMarketLanguage

    var body: some View {
        NavigationStack {
            GitMarketWebView(url: URL(string: "https://harzva.github.io/GitReleaseMarket/app.html")!)
                .ignoresSafeArea(edges: .bottom)
                .navigationTitle(language.text("discover"))
                .navigationBarTitleDisplayMode(.inline)
        }
    }
}

private struct SourcesView: View {
    let language: GitMarketLanguage

    var body: some View {
        NavigationStack {
            List {
                SourceRow(name: "GitHub", description: language.text("githubDesc"), color: .green)
                SourceRow(name: "Gitee", description: language.text("giteeDesc"), color: .orange)
                SourceRow(name: "GitCode", description: language.text("gitcodeDesc"), color: .blue)
            }
            .navigationTitle(language.text("sources"))
        }
    }
}

private struct SecurityView: View {
    let language: GitMarketLanguage

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 14) {
                    ForEach(SecurityNote.notes(language: language)) { note in
                        HStack(alignment: .top, spacing: 12) {
                            Image(systemName: note.symbol)
                                .font(.title3)
                                .foregroundStyle(.green)
                                .frame(width: 28)
                            VStack(alignment: .leading, spacing: 4) {
                                Text(note.title)
                                    .font(.headline)
                                Text(note.body)
                                    .font(.subheadline)
                                    .foregroundStyle(.secondary)
                            }
                        }
                        .padding()
                        .background(.thinMaterial)
                        .clipShape(RoundedRectangle(cornerRadius: 18, style: .continuous))
                    }
                }
                .padding()
            }
            .navigationTitle(language.text("security"))
        }
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
                    LabeledContent("GitMarket", value: "0.2.4 iOS Preview")
                    LabeledContent(language.text("mode"), value: "SwiftUI + WKWebView")
                }
            }
            .navigationTitle(language.text("settings"))
        }
    }
}

private struct MetricCard: View {
    let value: String
    let label: String
    let theme: GitMarketTheme

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(value)
                .font(.title.bold())
                .foregroundStyle(theme.primaryText)
            Text(label)
                .font(.caption)
                .foregroundStyle(theme.secondaryText)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        .background(theme.card)
        .clipShape(RoundedRectangle(cornerRadius: 18, style: .continuous))
    }
}

private struct RepositoryCard: View {
    let repo: SampleRepository
    let language: GitMarketLanguage
    let theme: GitMarketTheme

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack {
                Text(repo.name)
                    .font(.headline)
                    .foregroundStyle(theme.primaryText)
                Spacer()
                Text(repo.source)
                    .font(.caption.weight(.semibold))
                    .padding(.horizontal, 8)
                    .padding(.vertical, 5)
                    .background(theme.accent.opacity(0.14))
                    .foregroundStyle(theme.accent)
                    .clipShape(Capsule())
            }
            Text(repo.description(language: language))
                .font(.subheadline)
                .foregroundStyle(theme.secondaryText)
            HStack {
                Label(repo.version, systemImage: "tag")
                Label(repo.asset, systemImage: "shippingbox")
                Spacer()
            }
            .font(.caption)
            .foregroundStyle(theme.secondaryText)
        }
        .padding()
        .background(theme.card)
        .clipShape(RoundedRectangle(cornerRadius: 18, style: .continuous))
    }
}

private struct SourceRow: View {
    let name: String
    let description: String
    let color: Color

    var body: some View {
        HStack(spacing: 12) {
            Circle().fill(color).frame(width: 12, height: 12)
            VStack(alignment: .leading, spacing: 4) {
                Text(name).font(.headline)
                Text(description).font(.subheadline).foregroundStyle(.secondary)
            }
        }
        .padding(.vertical, 6)
    }
}
