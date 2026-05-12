use crate::data::{ApkAsset, Platform, ReleaseInfo, RepoInfo};
use reqwest::blocking::Client;

/// GitHub/Gitee API 客户端
pub struct ApiClient {
    client: Client,
    github_token: Option<String>,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("ApkGit/0.1.0")
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

    /// 获取仓库信息
    pub fn fetch_repo_info(
        &self,
        platform: Platform,
        owner: &str,
        repo: &str,
    ) -> Result<RepoInfo, String> {
        let url = format!("{}/repos/{}/{}", platform.api_base(), owner, repo);

        let mut request = self.client.get(&url);
        if platform == Platform::GitHub {
            if let Some(token) = &self.github_token {
                request = request.header("Authorization", format!("token {}", token));
            }
        }

        let response = request
            .send()
            .map_err(|e| format!("网络请求失败: {}", e))?;

        if response.status() == 403 {
            return Err("API 限流，请配置 GitHub Token（设置页面）".to_string());
        }
        if response.status() == 404 {
            return Err("仓库不存在或没有访问权限".to_string());
        }
        if !response.status().is_success() {
            return Err(format!("HTTP 错误: {}", response.status()));
        }

        response
            .json::<RepoInfo>()
            .map_err(|e| format!("JSON 解析失败: {}", e))
    }

    /// 获取 Release 列表
    pub fn fetch_releases(
        &self,
        platform: Platform,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<ReleaseInfo>, String> {
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
            .map_err(|e| format!("网络请求失败: {}", e))?;

        if response.status() == 404 {
            return Err("该仓库没有 Release".to_string());
        }
        if response.status() == 403 {
            return Err("API 限流，请配置 GitHub Token".to_string());
        }
        if !response.status().is_success() {
            return Err(format!("HTTP 错误: {}", response.status()));
        }

        let raw_releases: Vec<serde_json::Value> = response
            .json()
            .map_err(|e| format!("JSON 解析失败: {}", e))?;

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

            // 提取 APK assets
            if let Some(assets) = raw["assets"].as_array() {
                for asset in assets {
                    let name = asset["name"].as_str().unwrap_or("");
                    if name.ends_with(".apk") || name.contains(".apk") {
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

            // 仅保留包含 APK 的 Release
            if !release.assets.is_empty() {
                releases.push(release);
            }
        }

        Ok(releases)
    }
}
