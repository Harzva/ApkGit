use crate::api::ApiClient;
use crate::data::{parse_repo_url, Platform, ReleaseInfo, RepoInfo, SearchRepo};
use crate::download;
use eframe::egui;
use egui::{Color32, Margin, RichText, Rounding, Stroke};
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct GitMarketApp {
    repo_input: String,
    search_input: String,
    github_token: String,
    repo_info: Option<RepoInfo>,
    releases: Vec<ReleaseInfo>,
    discover_items: Vec<SearchRepo>,
    error_message: Option<String>,
    is_loading: bool,
    is_searching: bool,
    current_tab: Tab,
    theme: ThemeChoice,
    language: Language,
    active_source: SourceChoice,
    active_category: Category,
    download_status: Arc<Mutex<String>>,
    last_download_path: Option<PathBuf>,
    is_downloading: bool,
    tx: mpsc::Sender<AppMessage>,
    rx: mpsc::Receiver<AppMessage>,
    is_android: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Home,
    Discover,
    Repository,
    Downloads,
    Security,
    Settings,
}

#[derive(Clone, Copy, PartialEq)]
enum ThemeChoice {
    MeAgent,
    Warm,
    Clean,
    Launch,
}

#[derive(Clone, Copy, PartialEq)]
enum Language {
    Zh,
    En,
}

#[derive(Clone, Copy, PartialEq)]
enum SourceChoice {
    All,
    GitHub,
    Gitee,
    GitCode,
}

#[derive(Clone, Copy, PartialEq)]
enum Category {
    Trending,
    Editors,
    DevTools,
    Android,
    Utilities,
}

enum AppMessage {
    RepoInfo(RepoInfo),
    Releases(Vec<ReleaseInfo>),
    Discover(Vec<SearchRepo>),
    Error(String),
    DownloadComplete {
        message: String,
        path: Option<PathBuf>,
    },
    InstallComplete(String),
}

#[derive(Clone, Copy)]
struct ThemePalette {
    bg: Color32,
    panel: Color32,
    panel_alt: Color32,
    text: Color32,
    muted: Color32,
    accent: Color32,
    accent_alt: Color32,
    warning: Color32,
    danger: Color32,
    stroke: Color32,
    chip: Color32,
}

impl Default for GitMarketApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            repo_input: "termux/termux-app".to_string(),
            search_input: "release tools".to_string(),
            github_token: String::new(),
            repo_info: None,
            releases: Vec::new(),
            discover_items: Vec::new(),
            error_message: None,
            is_loading: false,
            is_searching: false,
            current_tab: Tab::Home,
            theme: ThemeChoice::MeAgent,
            language: Language::Zh,
            active_source: SourceChoice::All,
            active_category: Category::Trending,
            download_status: Arc::new(Mutex::new(String::new())),
            last_download_path: None,
            is_downloading: false,
            tx,
            rx,
            is_android: cfg!(target_os = "android"),
        }
    }
}

impl eframe::App for GitMarketApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_messages();
        self.apply_style(ctx);

        let palette = self.palette();
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(palette.bg))
            .show(ctx, |ui| {
                let available = ui.available_size();
                let mobile = self.is_android || available.x < 560.0;
                let content_height = if mobile {
                    available.y - 78.0
                } else {
                    available.y
                };

                ui.allocate_ui_with_layout(
                    egui::vec2(available.x, content_height.max(0.0)),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| self.show_page(ui));
                    },
                );

                if mobile {
                    self.bottom_nav(ui);
                } else {
                    ui.add_space(8.0);
                    self.bottom_nav(ui);
                }
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(180));
    }
}

impl GitMarketApp {
    fn show_page(&mut self, ui: &mut egui::Ui) {
        ui.add_space(self.top_padding());
        self.header(ui);
        ui.add_space(16.0);

        match self.current_tab {
            Tab::Home => self.show_home(ui),
            Tab::Discover => self.show_discover(ui),
            Tab::Repository => self.show_repository(ui),
            Tab::Downloads => self.show_downloads(ui),
            Tab::Security => self.show_security(ui),
            Tab::Settings => self.show_settings(ui),
        }
    }

    fn header(&mut self, ui: &mut egui::Ui) {
        let p = self.palette();
        let compact = self.is_android || ui.available_width() < 480.0;

        if compact {
            ui.horizontal(|ui| {
                self.logo_tile(ui);
                ui.vertical(|ui| {
                    ui.label(RichText::new("GitMarket").size(24.0).strong().color(p.text));
                    ui.add(
                        egui::Label::new(
                            RichText::new(self.t("tagline"))
                                .size(self.body_size())
                                .color(p.muted),
                        )
                        .wrap(),
                    );
                });
            });
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if self.pill_button(ui, self.t("settings")).clicked() {
                    self.current_tab = Tab::Settings;
                }
                if self.pill_button(ui, self.t("theme")).clicked() {
                    self.theme = self.next_theme();
                }
                if self.pill_button(ui, self.language_label()).clicked() {
                    self.language = match self.language {
                        Language::Zh => Language::En,
                        Language::En => Language::Zh,
                    };
                }
            });
        } else {
            ui.horizontal(|ui| {
                self.logo_tile(ui);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("GitMarket")
                            .size(self.title_size())
                            .strong()
                            .color(p.text),
                    );
                    ui.label(
                        RichText::new(self.t("tagline"))
                            .size(self.body_size())
                            .color(p.muted),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.pill_button(ui, self.t("settings")).clicked() {
                        self.current_tab = Tab::Settings;
                    }
                    if self.pill_button(ui, self.t("theme")).clicked() {
                        self.theme = self.next_theme();
                    }
                    if self.pill_button(ui, self.language_label()).clicked() {
                        self.language = match self.language {
                            Language::Zh => Language::En,
                            Language::En => Language::Zh,
                        };
                    }
                });
            });
        }
    }

    fn show_home(&mut self, ui: &mut egui::Ui) {
        self.search_box(ui);
        ui.add_space(12.0);
        self.category_row(ui);
        ui.add_space(14.0);
        self.hero_card(ui);
        ui.add_space(18.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(self.t("popular"))
                    .size(self.section_size())
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.text_button(ui, self.t("view_all")).clicked() {
                    self.current_tab = Tab::Discover;
                    self.start_discovery();
                }
            });
        });
        ui.add_space(8.0);

        for repo in self.sample_repos() {
            self.repo_row(ui, &repo);
            ui.add_space(8.0);
        }
    }

    fn show_discover(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(self.t("discover_title"))
                .size(self.title_size())
                .strong(),
        );
        ui.label(RichText::new(self.t("discover_sub")).color(self.palette().muted));
        ui.add_space(12.0);
        self.search_box(ui);
        ui.add_space(10.0);
        self.source_row(ui);
        ui.add_space(12.0);

        if self.discover_items.is_empty() && !self.is_searching {
            self.start_discovery();
        }

        if self.is_searching {
            self.loading_panel(ui, self.t("loading_sources"));
        }

        if let Some(err) = &self.error_message {
            self.message_panel(ui, err, self.palette().danger);
        }

        let items = self.discover_items.clone();
        for item in &items {
            self.search_result_card(ui, item);
            ui.add_space(8.0);
        }

        if self.active_source == SourceChoice::GitCode {
            ui.add_space(8.0);
            self.message_panel(ui, self.t("gitcode_fallback"), self.palette().warning);
            if self.primary_button(ui, self.t("open_gitcode")).clicked() {
                ui.ctx().open_url(egui::OpenUrl::new_tab(format!(
                    "https://gitcode.com/search?keyword={}",
                    self.search_input.replace(' ', "%20")
                )));
            }
        }
    }

    fn show_repository(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(self.t("repo_title"))
                .size(self.title_size())
                .strong(),
        );
        ui.label(RichText::new(self.t("repo_sub")).color(self.palette().muted));
        ui.add_space(12.0);

        self.repo_search_bar(ui);
        ui.add_space(12.0);

        if self.is_loading {
            self.loading_panel(ui, self.t("loading_repo"));
        }

        if let Some(err) = &self.error_message {
            self.message_panel(ui, err, self.palette().danger);
        }

        if let Some(repo) = self.repo_info.clone() {
            self.repo_info_card(ui, &repo);
        }

        if !self.releases.is_empty() {
            ui.add_space(14.0);
            ui.label(
                RichText::new(format!(
                    "{} ({})",
                    self.t("release_assets"),
                    self.releases.len()
                ))
                .size(self.section_size())
                .strong(),
            );
            ui.add_space(8.0);
        }

        let releases = self.releases.clone();
        for release in &releases {
            self.release_card(ui, release);
            ui.add_space(10.0);
        }
    }

    fn show_downloads(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(self.t("downloads"))
                .size(self.title_size())
                .strong(),
        );
        ui.label(RichText::new(self.t("downloads_sub")).color(self.palette().muted));
        ui.add_space(14.0);

        ui.columns(3, |columns| {
            self.metric_card(
                &mut columns[0],
                self.t("active"),
                if self.is_downloading { "1" } else { "0" },
            );
            self.metric_card(
                &mut columns[1],
                self.t("completed"),
                if self.last_download_path.is_some() {
                    "1"
                } else {
                    "0"
                },
            );
            self.metric_card(
                &mut columns[2],
                self.t("verifying"),
                if self.last_download_path.is_some() {
                    "1"
                } else {
                    "0"
                },
            );
        });
        ui.add_space(12.0);

        let status = self.download_status.lock().unwrap().clone();
        if status.is_empty() {
            self.empty_state(ui, self.t("no_downloads"), self.t("no_downloads_sub"));
        } else {
            card(ui, self.palette(), |ui| {
                ui.label(
                    RichText::new(self.t("latest_download"))
                        .size(self.section_size())
                        .strong(),
                );
                ui.add_space(8.0);
                ui.label(RichText::new(status).color(self.palette().muted));
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    if self.primary_button(ui, self.t("open_downloads")).clicked() {
                        download::open_download_dir();
                    }
                    if self.last_download_path.is_some()
                        && self.text_button(ui, self.t("install_apk")).clicked()
                    {
                        self.install_last_download();
                    }
                });
            });
        }
    }

    fn show_security(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(self.t("security"))
                .size(self.title_size())
                .strong(),
        );
        ui.label(RichText::new(self.t("security_sub")).color(self.palette().muted));
        ui.add_space(14.0);

        let rows = [
            (self.t("upstream_only"), self.t("upstream_only_desc")),
            (self.t("hashes"), self.t("hashes_desc")),
            (self.t("signatures"), self.t("signatures_desc")),
            (self.t("permissions"), self.t("permissions_desc")),
            (self.t("token_storage"), self.t("token_storage_desc")),
        ];

        for (title, desc) in rows {
            self.security_row(ui, title, desc);
            ui.add_space(8.0);
        }
    }

    fn show_settings(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(self.t("settings"))
                .size(self.title_size())
                .strong(),
        );
        ui.label(RichText::new(self.t("settings_sub")).color(self.palette().muted));
        ui.add_space(14.0);

        card(ui, self.palette(), |ui| {
            ui.label(RichText::new(self.t("theme")).strong());
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                for choice in [
                    ThemeChoice::MeAgent,
                    ThemeChoice::Warm,
                    ThemeChoice::Clean,
                    ThemeChoice::Launch,
                ] {
                    let selected = self.theme == choice;
                    if self
                        .segment(ui, self.theme_name(choice), selected)
                        .clicked()
                    {
                        self.theme = choice;
                    }
                }
            });
            ui.add_space(16.0);
            ui.label(RichText::new(self.t("language")).strong());
            ui.horizontal_wrapped(|ui| {
                if self
                    .segment(ui, "??", self.language == Language::Zh)
                    .clicked()
                {
                    self.language = Language::Zh;
                }
                if self
                    .segment(ui, "English", self.language == Language::En)
                    .clicked()
                {
                    self.language = Language::En;
                }
            });
        });

        ui.add_space(10.0);
        card(ui, self.palette(), |ui| {
            ui.label(RichText::new("GitHub Token").strong());
            ui.add_space(8.0);
            ui.text_edit_singleline(&mut self.github_token);
            ui.label(RichText::new(self.t("token_help")).color(self.palette().muted));
        });
    }

    fn search_box(&mut self, ui: &mut egui::Ui) {
        let search_label = self.t("search");
        let search_hint = self.t("search_hint");
        let go_label = self.t("go");
        let body_size = self.body_size();
        card(ui, self.palette(), |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(search_label).size(body_size + 2.0));
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.search_input)
                        .hint_text(search_hint)
                        .desired_width(f32::INFINITY),
                );
                if self.primary_button(ui, go_label).clicked()
                    || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    self.current_tab = Tab::Discover;
                    self.start_discovery();
                }
            });
        });
    }

    fn repo_search_bar(&mut self, ui: &mut egui::Ui) {
        card(ui, self.palette(), |ui| {
            ui.horizontal_wrapped(|ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.repo_input)
                        .hint_text("owner/repo or https://github.com/owner/repo")
                        .desired_width(280.0),
                );
                if self.primary_button(ui, self.t("inspect")).clicked()
                    || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    self.fetch_repo();
                }
            });
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                for example in [
                    "termux/termux-app",
                    "2dust/v2rayNG",
                    "obsproject/obs-studio",
                    "https://gitee.com/dromara/hutool",
                ] {
                    if self.text_button(ui, example).clicked() {
                        self.repo_input = example.to_string();
                    }
                }
            });
        });
    }

    fn category_row(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for category in [
                Category::Trending,
                Category::Editors,
                Category::DevTools,
                Category::Android,
                Category::Utilities,
            ] {
                let selected = self.active_category == category;
                if self
                    .segment(ui, self.category_name(category), selected)
                    .clicked()
                {
                    self.active_category = category;
                    self.search_input = self.category_query(category).to_string();
                }
            }
        });
    }

    fn source_row(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for source in [
                SourceChoice::All,
                SourceChoice::GitHub,
                SourceChoice::Gitee,
                SourceChoice::GitCode,
            ] {
                let selected = self.active_source == source;
                if self
                    .segment(ui, self.source_name(source), selected)
                    .clicked()
                {
                    self.active_source = source;
                    self.start_discovery();
                }
            }
        });
    }

    fn hero_card(&mut self, ui: &mut egui::Ui) {
        if self.theme == ThemeChoice::MeAgent {
            self.me_agent_hero_card(ui);
            return;
        }

        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel_alt)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(18.0))
            .inner_margin(Margin::same(18.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(self.t("featured")).color(p.accent).strong());
                        ui.label(RichText::new("NovaTerm").size(self.title_size()).strong());
                        ui.label(RichText::new(self.t("featured_desc")).color(p.muted));
                        ui.add_space(10.0);
                        ui.horizontal_wrapped(|ui| {
                            self.tag(ui, self.t("fast"));
                            self.tag(ui, self.t("secure"));
                            self.tag(ui, "v1.3.0");
                        });
                        ui.add_space(12.0);
                        if self.primary_button(ui, self.t("view_releases")).clicked() {
                            self.repo_input = "termux/termux-app".to_string();
                            self.current_tab = Tab::Repository;
                            self.fetch_repo();
                        }
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        self.terminal_preview(ui);
                    });
                });
            });
    }

    fn me_agent_hero_card(&mut self, ui: &mut egui::Ui) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(22.0))
            .inner_margin(Margin::same(18.0))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    self.tag(ui, self.t("today"));
                    self.tag(ui, "GitHub");
                    self.tag(ui, "Gitee");
                    self.tag(ui, "GitCode");
                });
                ui.add_space(10.0);
                ui.label(
                    RichText::new(self.t("me_hero_title"))
                        .size(self.title_size())
                        .strong()
                        .color(p.text),
                );
                ui.add(
                    egui::Label::new(RichText::new(self.t("me_hero_sub")).color(p.muted)).wrap(),
                );
                ui.add_space(14.0);

                ui.columns(3, |columns| {
                    self.mini_feature_card(
                        &mut columns[0],
                        self.t("collections"),
                        "12",
                        "APK/EXE/DMG",
                    );
                    self.mini_feature_card(
                        &mut columns[1],
                        self.t("memory"),
                        "20",
                        self.t("results"),
                    );
                    self.mini_feature_card(
                        &mut columns[2],
                        self.t("security"),
                        "SHA",
                        self.t("verified"),
                    );
                });

                ui.add_space(14.0);
                ui.horizontal_wrapped(|ui| {
                    if self.primary_button(ui, self.t("start_discover")).clicked() {
                        self.current_tab = Tab::Discover;
                        self.start_discovery();
                    }
                    if self.text_button(ui, self.t("inspect_repo")).clicked() {
                        self.current_tab = Tab::Repository;
                    }
                });
            });
    }

    fn mini_feature_card(&self, ui: &mut egui::Ui, title: &str, value: &str, desc: &str) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.chip)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(16.0))
            .inner_margin(Margin::same(10.0))
            .show(ui, |ui| {
                ui.label(RichText::new(title).size(self.body_size()).color(p.muted));
                ui.label(
                    RichText::new(value)
                        .size(self.section_size() + 4.0)
                        .strong()
                        .color(p.text),
                );
                ui.label(
                    RichText::new(desc)
                        .size(self.body_size() - 1.0)
                        .color(p.muted),
                );
            });
    }

    fn terminal_preview(&self, ui: &mut egui::Ui) {
        let p = self.palette();
        egui::Frame::none()
            .fill(Color32::from_rgb(16, 20, 24))
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(12.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.set_min_width(150.0);
                ui.label(RichText::new("release@market").monospace().color(p.green()));
                ui.label(RichText::new("> scan assets").monospace().color(p.text));
                ui.label(RichText::new("apk exe dmg deb").monospace().color(p.muted));
            });
    }

    fn repo_row(&mut self, ui: &mut egui::Ui, item: &SearchRepo) {
        card(ui, self.palette(), |ui| {
            ui.horizontal(|ui| {
                self.app_icon(ui, item);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(&item.full_name)
                                .strong()
                                .size(self.body_size() + 2.0),
                        );
                        self.tag(ui, item.language.as_deref().unwrap_or("Code"));
                    });
                    if let Some(desc) = &item.description {
                        ui.label(RichText::new(desc).color(self.palette().muted));
                    }
                    ui.horizontal_wrapped(|ui| {
                        self.tag(
                            ui,
                            &format!("{} stars", format_count(item.stargazers_count)),
                        );
                        self.tag(ui, &item.source);
                    });
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.text_button(ui, self.t("inspect")).clicked() {
                        self.repo_input = item.full_name.clone();
                        self.current_tab = Tab::Repository;
                        self.fetch_repo();
                    }
                });
            });
        });
    }

    fn search_result_card(&mut self, ui: &mut egui::Ui, item: &SearchRepo) {
        card(ui, self.palette(), |ui| {
            ui.horizontal(|ui| {
                self.app_icon(ui, item);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(&item.full_name)
                            .strong()
                            .size(self.body_size() + 3.0),
                    );
                    if let Some(desc) = &item.description {
                        ui.add(
                            egui::Label::new(RichText::new(desc).color(self.palette().muted))
                                .wrap(),
                        );
                    }
                    ui.horizontal_wrapped(|ui| {
                        self.tag(ui, &item.source);
                        self.tag(
                            ui,
                            &format!("{} stars", format_count(item.stargazers_count)),
                        );
                        if let Some(lang) = &item.language {
                            self.tag(ui, lang);
                        }
                    });
                });
            });
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if item.source == Platform::GitCode.label() {
                    if self.primary_button(ui, self.t("open_source")).clicked() {
                        ui.ctx()
                            .open_url(egui::OpenUrl::new_tab(item.html_url.clone()));
                    }
                } else {
                    if self.primary_button(ui, self.t("inspect")).clicked() {
                        self.repo_input = item.full_name.clone();
                        self.current_tab = Tab::Repository;
                        self.fetch_repo();
                    }
                    if self.text_button(ui, self.t("open_source")).clicked() {
                        ui.ctx()
                            .open_url(egui::OpenUrl::new_tab(item.html_url.clone()));
                    }
                }
            });
        });
    }

    fn repo_info_card(&mut self, ui: &mut egui::Ui, repo: &RepoInfo) {
        card(ui, self.palette(), |ui| {
            ui.label(
                RichText::new(&repo.full_name)
                    .size(self.section_size())
                    .strong(),
            );
            if let Some(desc) = &repo.description {
                ui.label(RichText::new(desc).color(self.palette().muted));
            }
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                self.tag(
                    ui,
                    &format!("{} stars", format_count(repo.stargazers_count)),
                );
                self.tag(ui, &format!("{} forks", format_count(repo.forks_count)));
                if let Some(lang) = &repo.language {
                    self.tag(ui, lang);
                }
                if self.text_button(ui, self.t("open_source")).clicked() {
                    ui.ctx()
                        .open_url(egui::OpenUrl::new_tab(repo.html_url.clone()));
                }
            });
        });
    }

    fn release_card(&mut self, ui: &mut egui::Ui, release: &ReleaseInfo) {
        card(ui, self.palette(), |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(&release.tag_name)
                            .size(self.section_size())
                            .strong(),
                    );
                    if let Some(date) = &release.published_at {
                        ui.label(
                            RichText::new(&date[..10.min(date.len())]).color(self.palette().muted),
                        );
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.tag(ui, &format!("{} assets", release.assets.len()));
                    if release.prerelease {
                        self.tag(ui, "pre-release");
                    }
                });
            });
            if let Some(body) = &release.body {
                ui.add_space(6.0);
                ui.add(egui::Label::new(RichText::new(body).color(self.palette().muted)).wrap());
            }
            ui.add_space(10.0);

            for asset in &release.assets {
                egui::Frame::none()
                    .fill(self.palette().chip)
                    .rounding(Rounding::same(10.0))
                    .inner_margin(Margin::symmetric(10.0, 8.0))
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(RichText::new(&asset.name).strong());
                            self.tag(ui, &asset.size_display());
                            if asset.download_count > 0 {
                                self.tag(
                                    ui,
                                    &format!("{} downloads", format_count(asset.download_count)),
                                );
                            }
                            if let Some(digest) = &asset.digest {
                                self.tag(ui, &format!("SHA {}", &digest[..12.min(digest.len())]));
                            }
                            let label = if self.is_downloading {
                                self.t("downloading")
                            } else {
                                self.t("download")
                            };
                            if self.primary_button(ui, label).clicked() && !self.is_downloading {
                                self.download_asset(
                                    asset.browser_download_url.clone(),
                                    asset.name.clone(),
                                );
                            }
                        });
                    });
                ui.add_space(6.0);
            }
        });
    }

    fn security_row(&self, ui: &mut egui::Ui, title: &str, desc: &str) {
        card(ui, self.palette(), |ui| {
            ui.label(RichText::new(title).strong().size(self.body_size() + 2.0));
            ui.label(RichText::new(desc).color(self.palette().muted));
        });
    }

    fn metric_card(&self, ui: &mut egui::Ui, title: &str, value: &str) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(14.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.label(RichText::new(title).color(p.muted));
                ui.label(RichText::new(value).size(26.0).strong().color(p.text));
            });
    }

    fn empty_state(&self, ui: &mut egui::Ui, title: &str, desc: &str) {
        card(ui, self.palette(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(18.0);
                ui.label(RichText::new(title).size(self.section_size()).strong());
                ui.label(RichText::new(desc).color(self.palette().muted));
                ui.add_space(18.0);
            });
        });
    }

    fn loading_panel(&self, ui: &mut egui::Ui, text: &str) {
        card(ui, self.palette(), |ui| {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(text);
            });
        });
    }

    fn message_panel(&self, ui: &mut egui::Ui, text: &str, color: Color32) {
        egui::Frame::none()
            .fill(self.palette().panel)
            .stroke(Stroke::new(1.0, color))
            .rounding(Rounding::same(12.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.label(RichText::new(text).color(color));
            });
    }

    fn bottom_nav(&mut self, ui: &mut egui::Ui) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(18.0))
            .inner_margin(Margin::symmetric(8.0, 8.0))
            .show(ui, |ui| {
                ui.columns(6, |columns| {
                    let tabs = [
                        (Tab::Home, self.t("home")),
                        (Tab::Discover, self.t("discover")),
                        (Tab::Repository, self.t("repos")),
                        (Tab::Downloads, self.t("downloads")),
                        (Tab::Security, self.t("security")),
                        (Tab::Settings, self.t("settings_short")),
                    ];
                    for (idx, (tab, label)) in tabs.iter().enumerate() {
                        let selected = self.current_tab == *tab;
                        if nav_button(&mut columns[idx], label, selected, p).clicked() {
                            self.current_tab = *tab;
                        }
                    }
                });
            });
    }

    fn logo_tile(&self, ui: &mut egui::Ui) {
        let p = self.palette();
        let (rect, _) = ui.allocate_exact_size(egui::vec2(54.0, 54.0), egui::Sense::hover());
        ui.painter()
            .rect_filled(rect, Rounding::same(14.0), p.accent);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "G",
            egui::FontId::proportional(28.0),
            Color32::WHITE,
        );
    }

    fn app_icon(&self, ui: &mut egui::Ui, item: &SearchRepo) {
        let p = self.palette();
        let seed = item
            .full_name
            .bytes()
            .fold(0u8, |acc, b| acc.wrapping_add(b));
        let color = match seed % 4 {
            0 => p.accent,
            1 => p.accent_alt,
            2 => p.warning,
            _ => p.muted,
        };
        let (rect, _) = ui.allocate_exact_size(egui::vec2(48.0, 48.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, Rounding::same(12.0), color);
        let letter = item
            .full_name
            .chars()
            .next()
            .unwrap_or('G')
            .to_uppercase()
            .to_string();
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            letter,
            egui::FontId::proportional(22.0),
            Color32::WHITE,
        );
    }

    fn tag(&self, ui: &mut egui::Ui, text: &str) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.chip)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::symmetric(7.0, 3.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(text)
                        .size(self.body_size() - 1.0)
                        .color(p.text),
                );
            });
    }

    fn primary_button(&self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        ui.add(
            egui::Button::new(RichText::new(label).strong())
                .fill(self.palette().accent)
                .rounding(Rounding::same(10.0))
                .min_size(egui::vec2(72.0, self.button_height())),
        )
    }

    fn text_button(&self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        ui.add(
            egui::Button::new(label)
                .fill(self.palette().panel_alt)
                .rounding(Rounding::same(10.0))
                .min_size(egui::vec2(52.0, self.button_height())),
        )
    }

    fn pill_button(&self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        ui.add(
            egui::Button::new(label)
                .fill(self.palette().panel)
                .rounding(Rounding::same(999.0))
                .min_size(egui::vec2(44.0, self.button_height())),
        )
    }

    fn segment(&self, ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
        let p = self.palette();
        ui.add(
            egui::Button::new(RichText::new(label).color(if selected {
                Color32::WHITE
            } else {
                p.text
            }))
            .fill(if selected { p.accent } else { p.panel })
            .rounding(Rounding::same(999.0))
            .min_size(egui::vec2(70.0, self.button_height())),
        )
    }

    fn fetch_repo(&mut self) {
        if self.is_loading {
            return;
        }

        let input = self.repo_input.trim().to_string();
        if input.is_empty() {
            self.error_message = Some(self.t("empty_repo").to_string());
            return;
        }

        let (platform, owner, repo) = match parse_repo_url(&input) {
            Some(v) => v,
            None => {
                self.error_message = Some(self.t("invalid_repo").to_string());
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

    fn start_discovery(&mut self) {
        if self.is_searching {
            return;
        }

        self.is_searching = true;
        self.error_message = None;
        self.discover_items.clear();

        let tx = self.tx.clone();
        let token = self.github_token.clone();
        let query = self.search_input.trim().to_string();
        let source = self.active_source;

        thread::spawn(move || {
            let client = if token.is_empty() {
                ApiClient::new()
            } else {
                ApiClient::new().with_token(token)
            };

            let platforms: Vec<Platform> = match source {
                SourceChoice::All => vec![Platform::GitHub, Platform::Gitee, Platform::GitCode],
                SourceChoice::GitHub => vec![Platform::GitHub],
                SourceChoice::Gitee => vec![Platform::Gitee],
                SourceChoice::GitCode => vec![Platform::GitCode],
            };

            let mut merged = Vec::new();
            let mut last_error = None;

            for platform in platforms {
                match client.search_repositories(
                    platform,
                    &query,
                    if query.is_empty() { 10 } else { 20 },
                ) {
                    Ok(mut items) => merged.append(&mut items),
                    Err(e) => last_error = Some(e),
                }
            }

            if merged.is_empty() {
                if let Some(err) = last_error {
                    let _ = tx.send(AppMessage::Error(err));
                } else {
                    let _ = tx.send(AppMessage::Discover(Vec::new()));
                }
            } else {
                merged.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
                let _ = tx.send(AppMessage::Discover(merged));
            }
        });
    }

    fn download_asset(&mut self, url: String, filename: String) {
        if self.is_downloading {
            return;
        }
        self.is_downloading = true;
        let filename = if filename.trim().is_empty() {
            download::filename_from_url(&url)
        } else {
            filename
        };
        *self.download_status.lock().unwrap() =
            format!("{} {}...", self.t("downloading"), filename);

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
                AppMessage::Discover(items) => {
                    self.discover_items = items;
                    self.is_searching = false;
                }
                AppMessage::Error(e) => {
                    self.error_message = Some(e);
                    self.is_loading = false;
                    self.is_searching = false;
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

    fn apply_style(&self, ctx: &egui::Context) {
        let p = self.palette();
        ctx.set_visuals(match self.theme {
            ThemeChoice::MeAgent | ThemeChoice::Warm | ThemeChoice::Clean => egui::Visuals::light(),
            ThemeChoice::Launch => egui::Visuals::dark(),
        });
        ctx.style_mut(|style| {
            style.visuals.panel_fill = p.bg;
            style.visuals.window_fill = p.panel;
            style.visuals.widgets.noninteractive.bg_fill = p.panel;
            style.visuals.widgets.inactive.bg_fill = p.panel;
            style.visuals.widgets.hovered.bg_fill = p.panel_alt;
            style.visuals.widgets.active.bg_fill = p.accent;
            style.spacing.item_spacing = egui::vec2(8.0, 8.0);
            style.spacing.button_padding = egui::vec2(12.0, 8.0);
            style.spacing.interact_size.y = self.button_height();
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::proportional(self.body_size()),
            );
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::proportional(self.body_size()),
            );
        });
    }

    fn palette(&self) -> ThemePalette {
        match self.theme {
            ThemeChoice::MeAgent => ThemePalette {
                bg: Color32::from_rgb(250, 251, 255),
                panel: Color32::from_rgb(255, 255, 255),
                panel_alt: Color32::from_rgb(242, 245, 255),
                text: Color32::from_rgb(21, 24, 34),
                muted: Color32::from_rgb(118, 124, 140),
                accent: Color32::from_rgb(48, 62, 104),
                accent_alt: Color32::from_rgb(108, 132, 232),
                warning: Color32::from_rgb(236, 78, 86),
                danger: Color32::from_rgb(211, 63, 72),
                stroke: Color32::from_rgb(232, 235, 244),
                chip: Color32::from_rgb(246, 248, 253),
            },
            ThemeChoice::Warm => ThemePalette {
                bg: Color32::from_rgb(255, 248, 235),
                panel: Color32::from_rgb(255, 253, 248),
                panel_alt: Color32::from_rgb(255, 226, 209),
                text: Color32::from_rgb(48, 18, 12),
                muted: Color32::from_rgb(130, 92, 79),
                accent: Color32::from_rgb(255, 104, 47),
                accent_alt: Color32::from_rgb(248, 178, 70),
                warning: Color32::from_rgb(219, 143, 32),
                danger: Color32::from_rgb(218, 68, 61),
                stroke: Color32::from_rgb(244, 214, 194),
                chip: Color32::from_rgb(255, 241, 232),
            },
            ThemeChoice::Clean => ThemePalette {
                bg: Color32::from_rgb(248, 250, 255),
                panel: Color32::from_rgb(255, 255, 255),
                panel_alt: Color32::from_rgb(232, 241, 255),
                text: Color32::from_rgb(19, 27, 44),
                muted: Color32::from_rgb(104, 113, 132),
                accent: Color32::from_rgb(35, 110, 237),
                accent_alt: Color32::from_rgb(69, 198, 155),
                warning: Color32::from_rgb(230, 164, 64),
                danger: Color32::from_rgb(220, 80, 80),
                stroke: Color32::from_rgb(221, 228, 240),
                chip: Color32::from_rgb(242, 246, 252),
            },
            ThemeChoice::Launch => ThemePalette {
                bg: Color32::from_rgb(20, 16, 13),
                panel: Color32::from_rgb(32, 25, 20),
                panel_alt: Color32::from_rgb(58, 40, 28),
                text: Color32::from_rgb(255, 240, 220),
                muted: Color32::from_rgb(196, 165, 137),
                accent: Color32::from_rgb(255, 127, 58),
                accent_alt: Color32::from_rgb(255, 202, 86),
                warning: Color32::from_rgb(255, 195, 93),
                danger: Color32::from_rgb(255, 105, 92),
                stroke: Color32::from_rgb(82, 58, 44),
                chip: Color32::from_rgb(47, 35, 29),
            },
        }
    }

    fn next_theme(&self) -> ThemeChoice {
        match self.theme {
            ThemeChoice::MeAgent => ThemeChoice::Warm,
            ThemeChoice::Warm => ThemeChoice::Clean,
            ThemeChoice::Clean => ThemeChoice::Launch,
            ThemeChoice::Launch => ThemeChoice::MeAgent,
        }
    }

    fn top_padding(&self) -> f32 {
        if self.is_android {
            38.0
        } else {
            10.0
        }
    }

    fn title_size(&self) -> f32 {
        if self.is_android {
            34.0
        } else {
            28.0
        }
    }

    fn section_size(&self) -> f32 {
        if self.is_android {
            22.0
        } else {
            19.0
        }
    }

    fn body_size(&self) -> f32 {
        if self.is_android {
            16.0
        } else {
            14.0
        }
    }

    fn button_height(&self) -> f32 {
        if self.is_android {
            42.0
        } else {
            34.0
        }
    }

    fn sample_repos(&self) -> Vec<SearchRepo> {
        vec![
            sample(
                "termux/termux-app",
                "Android terminal emulator",
                54800,
                "Java",
                "GitHub",
            ),
            sample(
                "2dust/v2rayNG",
                "Android proxy client",
                55900,
                "Kotlin",
                "GitHub",
            ),
            sample(
                "obsproject/obs-studio",
                "Streaming and recording",
                72300,
                "C",
                "GitHub",
            ),
            sample(
                "dromara/hutool",
                "Java utility library",
                31000,
                "Java",
                "Gitee",
            ),
            sample(
                "sharkdp/fd",
                "Fast find alternative",
                38400,
                "Rust",
                "GitHub",
            ),
        ]
    }

    fn category_query(&self, category: Category) -> &'static str {
        match category {
            Category::Trending => "release tools",
            Category::Editors => "editor release",
            Category::DevTools => "developer tools",
            Category::Android => "android apk",
            Category::Utilities => "utilities app",
        }
    }

    fn category_name(&self, category: Category) -> &'static str {
        match (self.language, category) {
            (Language::Zh, Category::Trending) => "??",
            (Language::Zh, Category::Editors) => "???",
            (Language::Zh, Category::DevTools) => "????",
            (Language::Zh, Category::Android) => "Android",
            (Language::Zh, Category::Utilities) => "????",
            (Language::En, Category::Trending) => "Trending",
            (Language::En, Category::Editors) => "Editors",
            (Language::En, Category::DevTools) => "Dev Tools",
            (Language::En, Category::Android) => "Android",
            (Language::En, Category::Utilities) => "Utilities",
        }
    }

    fn source_name(&self, source: SourceChoice) -> &'static str {
        match (self.language, source) {
            (Language::Zh, SourceChoice::All) => "????",
            (_, SourceChoice::GitHub) => "GitHub",
            (_, SourceChoice::Gitee) => "Gitee",
            (_, SourceChoice::GitCode) => "GitCode",
            (Language::En, SourceChoice::All) => "All Sources",
        }
    }

    fn theme_name(&self, choice: ThemeChoice) -> &'static str {
        match (self.language, choice) {
            (Language::Zh, ThemeChoice::MeAgent) => "ME Agent",
            (Language::Zh, ThemeChoice::Warm) => "????",
            (Language::Zh, ThemeChoice::Clean) => "????",
            (Language::Zh, ThemeChoice::Launch) => "????",
            (Language::En, ThemeChoice::MeAgent) => "ME Agent",
            (Language::En, ThemeChoice::Warm) => "Warm",
            (Language::En, ThemeChoice::Clean) => "Clean",
            (Language::En, ThemeChoice::Launch) => "Launch",
        }
    }

    fn language_label(&self) -> &'static str {
        match self.language {
            Language::Zh => "??",
            Language::En => "EN",
        }
    }

    fn t(&self, key: &str) -> &'static str {
        match self.language {
            Language::Zh => zh(key),
            Language::En => en(key),
        }
    }
}

fn card<R>(ui: &mut egui::Ui, p: ThemePalette, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::none()
        .fill(p.panel)
        .stroke(Stroke::new(1.0, p.stroke))
        .rounding(Rounding::same(14.0))
        .inner_margin(Margin::same(14.0))
        .show(ui, add_contents)
        .inner
}

impl ThemePalette {
    fn green(&self) -> Color32 {
        self.accent_alt
    }
}

fn nav_button(ui: &mut egui::Ui, label: &str, selected: bool, p: ThemePalette) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(label).color(if selected {
            Color32::WHITE
        } else {
            p.muted
        }))
        .fill(if selected { p.accent } else { p.panel })
        .rounding(Rounding::same(12.0))
        .min_size(egui::vec2(56.0, 42.0)),
    )
}

fn sample(name: &str, desc: &str, stars: u64, language: &str, source: &str) -> SearchRepo {
    SearchRepo {
        full_name: name.to_string(),
        description: Some(desc.to_string()),
        stargazers_count: stars,
        forks_count: 0,
        language: Some(language.to_string()),
        html_url: if source == "Gitee" {
            format!("https://gitee.com/{}", name)
        } else {
            format!("https://github.com/{}", name)
        },
        updated_at: None,
        topics: Vec::new(),
        source: source.to_string(),
    }
}

fn format_count(value: u64) -> String {
    if value >= 1000 {
        format!("{:.1}k", value as f64 / 1000.0)
    } else {
        value.to_string()
    }
}

fn zh(key: &str) -> &'static str {
    match key {
        "today" => "????",
        "me_hero_title" => "?? Release ????",
        "me_hero_sub" => "ME Agent ????????????????????????????",
        "collections" => "??",
        "memory" => "??",
        "results" => "????",
        "verified" => "???",
        "start_discover" => "????",
        "inspect_repo" => "????",
        "tagline" => "?? GitHub / Gitee / GitCode Release ??",
        "settings" => "??",
        "settings_short" => "??",
        "theme" => "??",
        "language" => "??",
        "home" => "??",
        "discover" => "??",
        "repos" => "??",
        "downloads" => "??",
        "security" => "??",
        "search" => "??",
        "search_hint" => "??????????? Release ???",
        "go" => "??",
        "popular" => "????",
        "view_all" => "????",
        "featured" => "Featured Release",
        "featured_desc" => "????????????????????? Release ???",
        "fast" => "??",
        "secure" => "??",
        "view_releases" => "?? Release",
        "discover_title" => "?????",
        "discover_sub" => "???? GitHub ? Gitee?GitCode ?????????",
        "loading_sources" => "????????...",
        "gitcode_fallback" => "GitCode ?????????????????????????? API?",
        "open_gitcode" => "?? GitCode ??",
        "repo_title" => "????",
        "repo_sub" => "?? owner/repo ??? URL??? Release ???????SHA ??????",
        "inspect" => "??",
        "loading_repo" => "??????? Release...",
        "release_assets" => "Release ??",
        "download" => "??",
        "downloading" => "???",
        "open_source" => "???",
        "downloads_sub" => "??????? SHA256??? Android ?????????",
        "active" => "???",
        "completed" => "???",
        "verifying" => "???",
        "no_downloads" => "?????",
        "no_downloads_sub" => "?????? Release ?????????????????",
        "latest_download" => "????",
        "open_downloads" => "??????",
        "install_apk" => "?? APK",
        "security_sub" => "GitMarket ?????????????????????????",
        "upstream_only" => "?????",
        "upstream_only_desc" => "????????? Release ?????????????",
        "hashes" => "SHA256 ??",
        "hashes_desc" => "??????? SHA256??????????????",
        "signatures" => "????",
        "signatures_desc" => "Android ???????????????????",
        "permissions" => "????",
        "permissions_desc" => "Android ??????????????????????",
        "token_storage" => "Token ??",
        "token_storage_desc" => "GitHub Token ????? API ??????????????????",
        "settings_sub" => "???????? GitHub API Token?",
        "token_help" => "???????? GitHub API ?????????????????",
        "empty_repo" => "????? URL ? owner/repo?",
        "invalid_repo" => "??????? github.com?gitee.com?gitcode.com ? owner/repo?",
        _ => "",
    }
}

fn en(key: &str) -> &'static str {
    match key {
        "today" => "Today",
        "me_hero_title" => "Your release intelligence board",
        "me_hero_sub" => "A ME Agent-inspired mobile workspace for repositories, versions, assets, safety signals, and downloads.",
        "collections" => "Collections",
        "memory" => "Memory",
        "results" => "Results",
        "verified" => "Verify",
        "start_discover" => "Discover",
        "inspect_repo" => "Inspect Repo",
        "tagline" => "Discover GitHub / Gitee / GitCode release assets",
        "settings" => "Settings",
        "settings_short" => "Settings",
        "theme" => "Theme",
        "language" => "Language",
        "home" => "Home",
        "discover" => "Discover",
        "repos" => "Repos",
        "downloads" => "Downloads",
        "security" => "Security",
        "search" => "Search",
        "search_hint" => "Search repositories, tools, authors, or release keywords",
        "go" => "Search",
        "popular" => "Popular Repositories",
        "view_all" => "View All",
        "featured" => "Featured Release",
        "featured_desc" => "Find open-source packages with better context, safety signals, and upstream links.",
        "fast" => "Fast",
        "secure" => "Secure",
        "view_releases" => "View Releases",
        "discover_title" => "Multi-source Discover",
        "discover_sub" => "Search GitHub and Gitee in parallel, with GitCode as a source-link fallback.",
        "loading_sources" => "Loading source results...",
        "gitcode_fallback" => "GitCode is connected as an upstream search entry. A backend proxy can make it stable later.",
        "open_gitcode" => "Open GitCode Search",
        "repo_title" => "Repository Inspector",
        "repo_sub" => "Inspect release assets, download counts, SHA values, and upstream source links.",
        "inspect" => "Inspect",
        "loading_repo" => "Loading repository and releases...",
        "release_assets" => "Release Assets",
        "download" => "Download",
        "downloading" => "Downloading",
        "open_source" => "Source",
        "downloads_sub" => "Manage downloads, verify SHA256, and continue installation.",
        "active" => "Active",
        "completed" => "Completed",
        "verifying" => "Verifying",
        "no_downloads" => "No downloads yet",
        "no_downloads_sub" => "Choose a release asset from the repository page to see progress and verification.",
        "latest_download" => "Latest Download",
        "open_downloads" => "Open Downloads",
        "install_apk" => "Install APK",
        "security_sub" => "GitMarket is a discovery and verification helper. It does not host or re-sign binaries.",
        "upstream_only" => "Upstream only",
        "upstream_only_desc" => "Every download points to the original Release asset.",
        "hashes" => "SHA256",
        "hashes_desc" => "Downloads are hashed locally; upstream checksum comparison comes next.",
        "signatures" => "Signatures",
        "signatures_desc" => "Android packages should show certificate fingerprints and signature-change warnings.",
        "permissions" => "Permission clarity",
        "permissions_desc" => "Android permissions must explain their purpose, especially network and installer access.",
        "token_storage" => "Token boundary",
        "token_storage_desc" => "GitHub tokens only raise API limits; production mobile builds should use OS credential storage.",
        "settings_sub" => "Switch theme, language, and GitHub API token.",
        "token_help" => "Optional. Only used to raise GitHub API limits. Avoid long-term storage on shared devices.",
        "empty_repo" => "Enter a repository URL or owner/repo.",
        "invalid_repo" => "Invalid format. Use github.com, gitee.com, gitcode.com, or owner/repo.",
        _ => "",
    }
}
