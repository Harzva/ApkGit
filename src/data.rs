use serde::{Deserialize, Serialize};

/// Repository metadata returned by an upstream code host.
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

/// Release metadata with downloadable assets.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub name: String,
    pub body: Option<String>,
    pub published_at: Option<String>,
    pub prerelease: bool,
    pub assets: Vec<ApkAsset>,
}

/// Downloadable release asset. The historic name is kept for compatibility.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApkAsset {
    pub name: String,
    pub size: u64,
    pub download_count: u64,
    pub browser_download_url: String,
    pub digest: Option<String>,
}

impl ApkAsset {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchRepo {
    pub full_name: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub language: Option<String>,
    pub html_url: String,
    pub updated_at: Option<String>,
    pub topics: Vec<String>,
    pub source: String,
}

/// Supported code hosting platforms.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    GitHub,
    Gitee,
    GitCode,
}

impl Platform {
    pub fn api_base(&self) -> &'static str {
        match self {
            Platform::GitHub => "https://api.github.com",
            Platform::Gitee => "https://gitee.com/api/v5",
            Platform::GitCode => "https://api.gitcode.com/api/v5",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Platform::GitHub => "GitHub",
            Platform::Gitee => "Gitee",
            Platform::GitCode => "GitCode",
        }
    }
}

pub fn parse_repo_url(url: &str) -> Option<(Platform, String, String)> {
    let url = url.trim();

    if let Some(rest) = url.strip_prefix("https://github.com/") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 2 {
            return Some((Platform::GitHub, parts[0].to_string(), parts[1].to_string()));
        }
    }

    if let Some(rest) = url.strip_prefix("https://gitee.com/") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 2 {
            return Some((Platform::Gitee, parts[0].to_string(), parts[1].to_string()));
        }
    }

    if let Some(rest) = url.strip_prefix("https://gitcode.com/") {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 2 {
            return Some((
                Platform::GitCode,
                parts[0].to_string(),
                parts[1].to_string(),
            ));
        }
    }

    if !url.contains("://") && url.matches('/').count() == 1 {
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            return Some((Platform::GitHub, parts[0].to_string(), parts[1].to_string()));
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

    #[test]
    fn test_parse_gitcode_url() {
        let result = parse_repo_url("https://gitcode.com/owner/project");
        assert!(result.is_some());
        let (platform, owner, repo) = result.unwrap();
        assert!(matches!(platform, Platform::GitCode));
        assert_eq!(owner, "owner");
        assert_eq!(repo, "project");
    }
}
