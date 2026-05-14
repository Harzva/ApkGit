use crate::data::{ApkAsset, Platform, ReleaseInfo, RepoInfo, SearchRepo};
use reqwest::blocking::Client;

pub struct ApiClient {
    client: Client,
    github_token: Option<String>,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("GitMarket/0.1.8")
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            github_token: None,
        }
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.github_token = Some(token);
        self
    }

    pub fn fetch_repo_info(
        &self,
        platform: Platform,
        owner: &str,
        repo: &str,
    ) -> Result<RepoInfo, String> {
        if platform == Platform::GitCode {
            return Err("GitCode direct repository API is not stable yet. Open the source search from Discover.".to_string());
        }

        let url = format!("{}/repos/{}/{}", platform.api_base(), owner, repo);

        let mut request = self.client.get(&url);
        if platform == Platform::GitHub {
            if let Some(token) = &self.github_token {
                request = request.header("Authorization", format!("token {}", token));
            }
        }

        let response = request
            .send()
            .map_err(|e| format!("Network request failed: {}", e))?;

        if response.status() == 403 {
            return Err(
                "API rate limit reached. Configure a GitHub token in Settings.".to_string(),
            );
        }
        if response.status() == 404 {
            return Err("Repository does not exist or is not accessible.".to_string());
        }
        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        response
            .json::<RepoInfo>()
            .map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    pub fn fetch_releases(
        &self,
        platform: Platform,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<ReleaseInfo>, String> {
        if platform == Platform::GitCode {
            return Err(
                "GitCode release API is not stable yet. Open the upstream project page."
                    .to_string(),
            );
        }

        let url = format!(
            "{}/repos/{}/{}/releases?per_page=20",
            platform.api_base(),
            owner,
            repo
        );

        let mut request = self.client.get(&url);
        if platform == Platform::GitHub {
            if let Some(token) = &self.github_token {
                request = request.header("Authorization", format!("token {}", token));
            }
        }

        let response = request
            .send()
            .map_err(|e| format!("Network request failed: {}", e))?;

        if response.status() == 404 {
            return Err("This repository has no releases.".to_string());
        }
        if response.status() == 403 {
            return Err("API rate limit reached. Configure a GitHub token.".to_string());
        }
        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        let raw_releases: Vec<serde_json::Value> = response
            .json()
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let mut releases = Vec::new();
        for raw in raw_releases {
            let mut release = ReleaseInfo {
                tag_name: raw["tag_name"].as_str().unwrap_or("unknown").to_string(),
                name: raw["name"].as_str().unwrap_or("").to_string(),
                body: raw["body"].as_str().map(|s| s.chars().take(500).collect()),
                published_at: raw["published_at"].as_str().map(|s| s.to_string()),
                prerelease: raw["prerelease"].as_bool().unwrap_or(false),
                assets: Vec::new(),
            };

            if let Some(assets) = raw["assets"].as_array() {
                for asset in assets {
                    let name = asset["name"].as_str().unwrap_or("");
                    if is_supported_asset(name) {
                        release.assets.push(ApkAsset {
                            name: name.to_string(),
                            size: asset["size"].as_u64().unwrap_or(0),
                            download_count: asset["download_count"].as_u64().unwrap_or(0),
                            browser_download_url: asset["browser_download_url"]
                                .as_str()
                                .unwrap_or("")
                                .to_string(),
                            digest: asset["digest"].as_str().map(|s| s.to_string()),
                        });
                    }
                }
            }

            if !release.assets.is_empty() {
                releases.push(release);
            }
        }

        Ok(releases)
    }

    pub fn search_repositories(
        &self,
        platform: Platform,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchRepo>, String> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(curated_repositories(platform, "", limit));
        }

        match platform {
            Platform::GitHub => self.search_github(query, limit),
            Platform::Gitee => self.search_gitee(query, limit),
            Platform::GitCode => Ok(curated_repositories(Platform::GitCode, query, limit.min(2))),
        }
    }

    fn search_github(&self, query: &str, limit: usize) -> Result<Vec<SearchRepo>, String> {
        let url = format!(
            "{}/search/repositories?q={}&sort=stars&order=desc&per_page={}",
            Platform::GitHub.api_base(),
            encode_query(query),
            limit.min(30)
        );

        let mut request = self.client.get(&url);
        if let Some(token) = &self.github_token {
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = match request.send() {
            Ok(response) => response,
            Err(_) => return Ok(curated_repositories(Platform::GitHub, query, limit)),
        };

        if response.status() == 403 {
            return Ok(curated_repositories(Platform::GitHub, query, limit));
        }
        if !response.status().is_success() {
            return Ok(curated_repositories(Platform::GitHub, query, limit));
        }

        let raw: serde_json::Value = match response.json() {
            Ok(raw) => raw,
            Err(_) => return Ok(curated_repositories(Platform::GitHub, query, limit)),
        };
        let items = raw["items"].as_array().cloned().unwrap_or_default();

        Ok(items
            .into_iter()
            .map(|item| SearchRepo {
                full_name: item["full_name"].as_str().unwrap_or("unknown").to_string(),
                description: item["description"].as_str().map(|s| s.to_string()),
                stargazers_count: item["stargazers_count"].as_u64().unwrap_or(0),
                forks_count: item["forks_count"].as_u64().unwrap_or(0),
                language: item["language"].as_str().map(|s| s.to_string()),
                html_url: item["html_url"].as_str().unwrap_or("").to_string(),
                updated_at: item["updated_at"].as_str().map(|s| s.to_string()),
                topics: item["topics"]
                    .as_array()
                    .map(|topics| {
                        topics
                            .iter()
                            .filter_map(|topic| topic.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default(),
                source: Platform::GitHub.label().to_string(),
            })
            .collect())
    }

    fn search_gitee(&self, query: &str, limit: usize) -> Result<Vec<SearchRepo>, String> {
        let url = format!(
            "{}/search/repositories?q={}&sort=stars_count&order=desc&page=1&per_page={}",
            Platform::Gitee.api_base(),
            encode_query(query),
            limit.min(30)
        );

        let response = self.client.get(&url).send();

        let response = match response {
            Ok(response) => response,
            Err(_) => return Ok(curated_repositories(Platform::Gitee, query, limit)),
        };

        if !response.status().is_success() {
            return Ok(curated_repositories(Platform::Gitee, query, limit));
        }

        let raw: serde_json::Value = match response.json() {
            Ok(raw) => raw,
            Err(_) => return Ok(curated_repositories(Platform::Gitee, query, limit)),
        };
        let items = raw.as_array().cloned().unwrap_or_default();

        Ok(items
            .into_iter()
            .map(|item| {
                let full_name = item["full_name"]
                    .as_str()
                    .or_else(|| item["path_with_namespace"].as_str())
                    .or_else(|| item["human_name"].as_str())
                    .or_else(|| item["name"].as_str())
                    .unwrap_or("unknown");
                SearchRepo {
                    full_name: full_name.to_string(),
                    description: item["description"].as_str().map(|s| s.to_string()),
                    stargazers_count: item["stargazers_count"]
                        .as_u64()
                        .or_else(|| item["stars_count"].as_u64())
                        .unwrap_or(0),
                    forks_count: item["forks_count"].as_u64().unwrap_or(0),
                    language: item["language"].as_str().map(|s| s.to_string()),
                    html_url: item["html_url"]
                        .as_str()
                        .or_else(|| item["url"].as_str())
                        .unwrap_or("")
                        .to_string(),
                    updated_at: item["updated_at"]
                        .as_str()
                        .or_else(|| item["pushed_at"].as_str())
                        .map(|s| s.to_string()),
                    topics: Vec::new(),
                    source: Platform::Gitee.label().to_string(),
                }
            })
            .collect())
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

fn is_supported_asset(name: &str) -> bool {
    let name = name.to_lowercase();
    let exts = [
        ".apk",
        ".exe",
        ".msi",
        ".dmg",
        ".pkg",
        ".appimage",
        ".deb",
        ".rpm",
        ".ipa",
        ".zip",
        ".tar.gz",
        ".tgz",
        ".7z",
    ];
    exts.iter()
        .any(|ext| name.ends_with(ext) || name.contains(ext))
}

fn encode_query(query: &str) -> String {
    query
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("+")
        .replace('/', "%2F")
}

fn curated_repositories(platform: Platform, query: &str, limit: usize) -> Vec<SearchRepo> {
    let items = match platform {
        Platform::GitHub => [
            (
                "termux/termux-app",
                "Android terminal emulator",
                54800,
                "Java",
            ),
            ("2dust/v2rayNG", "Android proxy client", 55900, "Kotlin"),
            (
                "obsproject/obs-studio",
                "Streaming and recording",
                72300,
                "C",
            ),
            ("pbatard/rufus", "USB formatting utility", 36000, "C"),
            ("KeePassXC/KeePassXC", "Password manager", 25100, "C++"),
            (
                "laurent22/joplin",
                "Privacy-focused notes",
                54800,
                "TypeScript",
            ),
            (
                "PowerShell/PowerShell",
                "Shell for every system",
                49800,
                "C#",
            ),
            ("ShareX/ShareX", "Screen capture workflow", 35000, "C#"),
            ("sharkdp/fd", "Fast find alternative", 38400, "Rust"),
            ("BurntSushi/ripgrep", "Fast recursive search", 55000, "Rust"),
        ]
        .as_slice(),
        Platform::Gitee => [
            (
                "openharmony/docs",
                "OpenHarmony documentation",
                9200,
                "Markdown",
            ),
            ("dromara/hutool", "Java utility library", 31000, "Java"),
            (
                "dromara/Sa-Token",
                "Java permission framework",
                17000,
                "Java",
            ),
            ("mindspore/mindspore", "AI computing framework", 6400, "C++"),
            ("oschina/git-osc", "Gitee tools", 1800, "Java"),
            (
                "anolis/cloud-kernel",
                "Linux kernel distribution",
                1100,
                "C",
            ),
            ("src-openeuler/kernel", "openEuler kernel", 1400, "C"),
            ("dromara/Jpom", "DevOps project manager", 12000, "Java"),
            ("jeecg/jeecg-boot", "Low-code platform", 42000, "Java"),
            ("layui/layui", "Classic UI framework", 28000, "JavaScript"),
        ]
        .as_slice(),
        Platform::GitCode => [
            (
                "GitCode mirror",
                "Open GitCode source search",
                0,
                "Fallback",
            ),
            (
                "Release keyword",
                "Use upstream search for GitCode projects",
                0,
                "Fallback",
            ),
        ]
        .as_slice(),
    };

    let terms: Vec<String> = query
        .to_lowercase()
        .split_whitespace()
        .map(str::to_string)
        .collect();

    let mut repos: Vec<SearchRepo> = items
        .iter()
        .filter(|(name, desc, _, lang)| {
            if terms.is_empty() {
                return true;
            }
            let haystack =
                format!("{} {} {} {}", name, desc, lang, platform.label()).to_lowercase();
            terms.iter().any(|term| haystack.contains(term))
        })
        .map(|(name, desc, stars, lang)| SearchRepo {
            full_name: (*name).to_string(),
            description: Some((*desc).to_string()),
            stargazers_count: *stars,
            forks_count: 0,
            language: Some((*lang).to_string()),
            html_url: match platform {
                Platform::GitHub => format!("https://github.com/{}", name),
                Platform::Gitee => format!("https://gitee.com/{}", name),
                Platform::GitCode => format!(
                    "https://gitcode.com/search?keyword={}",
                    if query.is_empty() {
                        "release".to_string()
                    } else {
                        encode_query(query)
                    }
                ),
            },
            updated_at: None,
            topics: vec!["fallback".to_string(), "upstream".to_string()],
            source: platform.label().to_string(),
        })
        .take(limit)
        .collect();

    if repos.is_empty() {
        repos = items
            .iter()
            .take(limit.min(items.len()))
            .map(|(name, desc, stars, lang)| SearchRepo {
                full_name: (*name).to_string(),
                description: Some(format!("{} (local fallback for '{}')", desc, query)),
                stargazers_count: *stars,
                forks_count: 0,
                language: Some((*lang).to_string()),
                html_url: match platform {
                    Platform::GitHub => format!("https://github.com/{}", name),
                    Platform::Gitee => format!("https://gitee.com/{}", name),
                    Platform::GitCode => {
                        format!("https://gitcode.com/search?keyword={}", encode_query(query))
                    }
                },
                updated_at: None,
                topics: vec!["fallback".to_string(), "upstream".to_string()],
                source: platform.label().to_string(),
            })
            .collect();
    }

    repos
}
