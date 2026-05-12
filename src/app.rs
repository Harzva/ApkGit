use crate::api::ApiClient;
use crate::data::{parse_repo_url, ReleaseInfo, RepoInfo};
use crate::download;
use eframe::egui;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ReleaseMarketApp {
    repo_input: String,
    github_token: String,
    repo_info: Option<RepoInfo>,
    releases: Vec<ReleaseInfo>,
    error_message: Option<String>,
    is_loading: bool,
    current_tab: Tab,
    show_settings: bool,
    download_status: Arc<Mutex<String>>,
    last_download_path: Option<std::path::PathBuf>,
    is_downloading: bool,
    tx: mpsc::Sender<AppMessage>,
    rx: mpsc::Receiver<AppMessage>,
    is_android: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Repo,
    Hot,
}

enum AppMessage {
    RepoInfo(RepoInfo),
    Releases(Vec<ReleaseInfo>),
    Error(String),
    DownloadComplete {
        message: String,
        path: Option<std::path::PathBuf>,
    },
    InstallComplete(String),
}

impl Default for ReleaseMarketApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        let is_android = cfg!(target_os = "android");

        Self {
            repo_input: String::new(),
            github_token: String::new(),
            repo_info: None,
            releases: Vec::new(),
            error_message: None,
            is_loading: false,
            current_tab: Tab::Repo,
            show_settings: false,
            download_status: Arc::new(Mutex::new(String::new())),
            last_download_path: None,
            is_downloading: false,
            tx,
            rx,
            is_android,
        }
    }
}

impl ReleaseMarketApp {
    fn text_size(&self) -> f32 {
        if self.is_android {
            18.0
        } else {
            14.0
        }
    }

    fn button_height(&self) -> f32 {
        if self.is_android {
            44.0
        } else {
            28.0
        }
    }

    fn fetch_repo(&mut self) {
        if self.is_loading {
            return;
        }

        let input = self.repo_input.trim().to_string();
        if input.is_empty() {
            self.error_message = Some("Enter a repository URL or owner/repo.".to_string());
            return;
        }

        let (platform, owner, repo) = match parse_repo_url(&input) {
            Some(v) => v,
            None => {
                self.error_message = Some(
                    "Invalid format. Use https://github.com/owner/repo, https://gitee.com/owner/repo, or owner/repo."
                        .to_string(),
                );
                return;
            }
        };

        self.is_loading = true;
        self.error_message = None;
        self.repo_info = None;
        self.releases.clear();

        let tx = self.tx.clone();
        let token = self.github_token.clone();

        thread::spawn(move || {
            let client = if token.is_empty() {
                ApiClient::new()
            } else {
                ApiClient::new().with_token(token)
            };

            match client.fetch_repo_info(platform, &owner, &repo) {
                Ok(info) => {
                    let _ = tx.send(AppMessage::RepoInfo(info));
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::Error(e));
                    return;
                }
            }

            match client.fetch_releases(platform, &owner, &repo) {
                Ok(releases) => {
                    let _ = tx.send(AppMessage::Releases(releases));
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::Error(e));
                }
            }
        });
    }

    fn download_apk(&mut self, url: String, filename: String) {
        if self.is_downloading {
            return;
        }
        self.is_downloading = true;
        let filename = if filename.trim().is_empty() {
            download::filename_from_url(&url)
        } else {
            filename
        };
        *self.download_status.lock().unwrap() = format!("Downloading {}...", filename);

        let tx = self.tx.clone();
        thread::spawn(move || {
            let dir = download::get_download_dir();
            let path = dir.join(&filename);
            let result = download::download_file(&url, &path);
            let message = match result {
                Ok(_) => match download::calc_sha256(&path) {
                    Ok(hash) => format!("Download complete: {}\nSHA256: {}", path.display(), hash),
                    Err(e) => format!(
                        "Download complete: {}\nSHA256 calculation failed: {}",
                        path.display(),
                        e
                    ),
                },
                Err(e) => format!("Download failed: {}", e),
            };
            let ok_path = path.exists().then_some(path);
            let _ = tx.send(AppMessage::DownloadComplete {
                message,
                path: ok_path,
            });
        });
    }

    fn install_last_download(&mut self) {
        let Some(path) = self.last_download_path.clone() else {
            *self.download_status.lock().unwrap() = "No downloaded APK is available.".to_string();
            return;
        };

        let tx = self.tx.clone();
        *self.download_status.lock().unwrap() = format!("Installing {}...", path.display());
        thread::spawn(move || {
            let msg = match download::install_apk_native(&path) {
                Ok(msg) => msg,
                Err(e) => e,
            };
            let _ = tx.send(AppMessage::InstallComplete(msg));
        });
    }

    fn check_messages(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AppMessage::RepoInfo(info) => {
                    self.repo_info = Some(info);
                    self.is_loading = false;
                }
                AppMessage::Releases(releases) => {
                    self.releases = releases;
                    self.is_loading = false;
                }
                AppMessage::Error(e) => {
                    self.error_message = Some(e);
                    self.is_loading = false;
                }
                AppMessage::DownloadComplete { message, path } => {
                    self.last_download_path = path;
                    *self.download_status.lock().unwrap() = message;
                    self.is_downloading = false;
                }
                AppMessage::InstallComplete(msg) => {
                    *self.download_status.lock().unwrap() = msg;
                }
            }
        }
    }
}

impl eframe::App for ReleaseMarketApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_messages();

        if self.is_android {
            ctx.style_mut(|style| {
                style
                    .text_styles
                    .insert(egui::TextStyle::Body, egui::FontId::proportional(18.0));
                style
                    .text_styles
                    .insert(egui::TextStyle::Button, egui::FontId::proportional(18.0));
                style.spacing.interact_size.y = 44.0;
            });
        }

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("ReleaseMarket")
                        .size(self.text_size() + 6.0)
                        .strong(),
                );
                ui.label("GitHub/Gitee Release discovery");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(egui::RichText::new("Settings").size(self.text_size()))
                        .clicked()
                    {
                        self.show_settings = !self.show_settings;
                    }
                });
            });
        });

        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            let text_size = self.text_size();
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.current_tab,
                    Tab::Repo,
                    egui::RichText::new("Repository").size(text_size),
                );
                ui.selectable_value(
                    &mut self.current_tab,
                    Tab::Hot,
                    egui::RichText::new("Hot list").size(text_size),
                );
            });
        });

        if self.show_settings {
            egui::Window::new("Settings").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("GitHub Token:");
                    ui.text_edit_singleline(&mut self.github_token);
                });
                ui.small("Optional. Use a GitHub personal access token to raise API rate limits.");
                if ui.button("Save").clicked() {
                    self.show_settings = false;
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Repo => self.show_repo_tab(ui),
                Tab::Hot => self.show_hot_tab(ui),
            }

            let status = self.download_status.lock().unwrap().clone();
            if !status.is_empty() {
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label(&status);
                    ui.horizontal(|ui| {
                        if ui.button("Open downloads").clicked() {
                            download::open_download_dir();
                        }
                        if self.last_download_path.is_some()
                            && ui.button("Install with ADB").clicked()
                        {
                            self.install_last_download();
                        }
                    });
                    #[cfg(target_os = "android")]
                    if status.contains("Download complete") {
                        ui.colored_label(
                            egui::Color32::YELLOW,
                            "On Android, install manually or use ADB.",
                        );
                    }
                });
            }
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

impl ReleaseMarketApp {
    fn show_repo_tab(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Repository:").size(self.text_size()));
                    let edit = ui.text_edit_singleline(&mut self.repo_input);
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("Inspect").size(self.text_size()),
                            )
                            .min_size(egui::vec2(60.0, self.button_height())),
                        )
                        .clicked()
                        || (edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.fetch_repo();
                    }
                    ui.menu_button("Examples", |ui| {
                        let examples = [
                            "termux/termux-app",
                            "2dust/v2rayNG",
                            "MatsuriDayo/NekoBoxForAndroid",
                            "zhanghai/MaterialFiles",
                            "tuyafeng/Watt",
                        ];
                        for name in examples {
                            if ui.button(name).clicked() {
                                self.repo_input = name.to_string();
                            }
                        }
                    });
                });
            });

            if let Some(err) = &self.error_message {
                ui.add_space(5.0);
                ui.colored_label(egui::Color32::RED, err);
            }

            if self.is_loading {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Loading...");
                });
                return;
            }

            if let Some(repo) = &self.repo_info {
                ui.add_space(10.0);
                self.show_repo_info(ui, repo);
            }

            if !self.releases.is_empty() {
                ui.add_space(10.0);
                ui.separator();
                ui.heading(format!("Releases ({})", self.releases.len()));

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (idx, release) in self.releases.clone().iter().enumerate() {
                        self.show_release(ui, idx, release);
                    }
                });
            }
        });
    }

    fn show_repo_info(&self, ui: &mut egui::Ui, repo: &RepoInfo) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(repo.full_name.as_str())
                        .size(self.text_size() + 2.0)
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.hyperlink_to("Open on GitHub", &repo.html_url);
                });
            });

            if let Some(desc) = &repo.description {
                ui.label(desc);
            }

            ui.horizontal(|ui| {
                ui.label(format!("{} stars", repo.stargazers_count));
                ui.label(format!("{} forks", repo.forks_count));
                if let Some(lang) = &repo.language {
                    ui.label(lang.as_str());
                }
            });
        });
    }

    fn show_release(&mut self, ui: &mut egui::Ui, _idx: usize, release: &ReleaseInfo) {
        egui::CollapsingHeader::new(format!(
            "{} {} ({} APKs)",
            if release.prerelease {
                "[pre-release] "
            } else {
                ""
            },
            release.tag_name,
            release.assets.len()
        ))
        .show(ui, |ui| {
            if let Some(date) = &release.published_at {
                ui.small(format!("Published: {}", &date[..10.min(date.len())]));
            }

            for asset in &release.assets {
                ui.horizontal(|ui| {
                    ui.label(asset.name.as_str());
                    ui.label(asset.size_display());
                    if asset.download_count > 0 {
                        ui.small(format!("{} downloads", asset.download_count));
                    }

                    let btn_label = if self.is_downloading {
                        "Downloading..."
                    } else {
                        "Download"
                    };
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(btn_label).size(self.text_size()),
                            )
                            .min_size(egui::vec2(60.0, self.button_height())),
                        )
                        .clicked()
                        && !self.is_downloading
                    {
                        self.download_apk(asset.browser_download_url.clone(), asset.name.clone());
                    }

                    if let Some(digest) = &asset.digest {
                        ui.small(format!("SHA: {}", &digest[..16.min(digest.len())]));
                    }
                });
            }
        });
    }

    fn show_hot_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("GitHub Release hot list");
        ui.small("Live discovery through the GitHub Search API");
        ui.add_space(10.0);

        let topics = [
            "Android tools",
            "Privacy tools",
            "AI tools",
            "Network tools",
            "Media",
            "File managers",
        ];
        ui.horizontal_wrapped(|ui| {
            for label in topics {
                if ui
                    .button(egui::RichText::new(label).size(self.text_size()))
                    .clicked()
                {
                    self.error_message = Some(format!(
                        "Search preset: {}. Enhanced hot lists are planned.",
                        label
                    ));
                }
            }
        });

        ui.add_space(20.0);
        ui.colored_label(
            egui::Color32::GRAY,
            "Tip: enter a GitHub repository in the Repository tab to inspect Release assets.",
        );
        ui.colored_label(
            egui::Color32::GRAY,
            "Enhanced ranking by stars, downloads, recency, and safety signals is planned.",
        );
    }
}
