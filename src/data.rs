use serde::{Deserialize, Serialize};

/// GitHub/Gitee 仓库信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RepoInfo {
    pub full_name: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub language: Option<String>,
    pub html_url: String,
    pub updated_at: Option<String>,
    pub topics: Vec<String>,
}

/// Release 信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
    pub published_at: Option<String>,
    pub prerelease: bool,
    pub assets: Vec<ApkAsset>,
}

/// APK 资源文件
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApkAsset {
    pub name: String,
    pub size: u64,
    pub download_count: u64,
    pub browser_download_url: String,
    pub digest: Option<String>,
}

impl ApkAsset {
    /// 格式化文件大小
    pub fn size_display(&self) -> String {
        if self.size < 1024 {
            format!("{} B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.1} KB", self.size as f64 / 1024.0)
        } else {
            format!("{:.1} MB", self.size as f64 / (1024.0 * 1024.0))
        }
    }
}

/// 支持的代码平台
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    GitHub,
    Gitee,
}

impl Platform {
    pub fn api_base(&self) -> &'static str {
        match self {
            Platform::GitHub => "https://api.github.com",
            Platform::Gitee => "https://gitee.com/api/v5",
        }
    }
}

/// 解析用户输入的仓库 URL，提取平台和 owner/repo
pub fn parse_repo_url(url: &str) -> Option<(Platform, String, String)> {
    let url = url.trim();

    // GitHub 格式: https://github.com/owner/repo
    if let Some(rest) = url.strip_prefix("https://github.com/") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 2 {
            return Some((
                Platform::GitHub,
                parts[0].to_string(),
                parts[1].to_string(),
            ));
        }
    }

    // GitHub 短格式: owner/repo
    if !url.contains("://") && url.matches('/').count() == 1 {
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            return Some((
                Platform::GitHub,
                parts[0].to_string(),
                parts[1].to_string(),
            ));
        }
    }

    // Gitee 格式: https://gitee.com/owner/repo
    if let Some(rest) = url.strip_prefix("https://gitee.com/") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 2 {
            return Some((
                Platform::Gitee,
                parts[0].to_string(),
                parts[1].to_string(),
            ));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_full_url() {
        let result = parse_repo_url("https://github.com/termux/termux-app");
        assert!(result.is_some());
        let (platform, owner, repo) = result.unwrap();
        assert!(matches!(platform, Platform::GitHub));
        assert_eq!(owner, "termux");
        assert_eq!(repo, "termux-app");
    }

    #[test]
    fn test_parse_short_format() {
        let result = parse_repo_url("termux/termux-app");
        assert!(result.is_some());
        let (platform, owner, repo) = result.unwrap();
        assert!(matches!(platform, Platform::GitHub));
        assert_eq!(owner, "termux");
        assert_eq!(repo, "termux-app");
    }

    #[test]
    fn test_parse_gitee_url() {
        let result = parse_repo_url("https://gitee.com/mirrors/termux-app");
        assert!(result.is_some());
        let (platform, owner, repo) = result.unwrap();
        assert!(matches!(platform, Platform::Gitee));
        assert_eq!(owner, "mirrors");
        assert_eq!(repo, "termux-app");
    }
}
