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
    discover_visible: usize,
    discovery_request_id: u64,
    last_discovery_key: String,
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
    fonts_ready: bool,
    styled_theme: Option<ThemeChoice>,
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
    RepoInfo(Box<RepoInfo>),
    Releases(Vec<ReleaseInfo>),
    DiscoverFinished {
        request_id: u64,
        result: Result<Vec<SearchRepo>, String>,
    },
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
            discover_visible: 12,
            discovery_request_id: 0,
            last_discovery_key: String::new(),
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
            fonts_ready: false,
            styled_theme: None,
        }
    }
}

impl eframe::App for GitMarketApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_messages();
        self.ensure_fonts(ctx);
        if self.styled_theme != Some(self.theme) {
            self.apply_style(ctx);
            self.styled_theme = Some(self.theme);
        }

        let palette = self.palette();
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(palette.bg))
            .show(ctx, |ui| {
                let available = ui.available_size();
                let mobile = self.is_android || available.x < 760.0;

                if mobile {
                    self.mobile_shell(ui, available);
                } else {
                    self.desktop_shell(ui, available);
                }
            });

        let repaint_ms = if self.is_loading || self.is_searching || self.is_downloading {
            33
        } else {
            600
        };
        ctx.request_repaint_after(std::time::Duration::from_millis(repaint_ms));
    }
}

impl GitMarketApp {
    fn ensure_fonts(&mut self, ctx: &egui::Context) {
        if self.fonts_ready {
            return;
        }

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "gitmarket_cjk_subset".to_string(),
            egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSansSC-GitMarket.ttf")),
        );
        register_font_family(&mut fonts, "gitmarket_cjk_subset", 0);

        for (name, path) in cjk_system_font_candidates() {
            if let Ok(bytes) = std::fs::read(path) {
                fonts
                    .font_data
                    .insert(name.to_string(), egui::FontData::from_owned(bytes));
                register_font_family(&mut fonts, name, 1);
                break;
            }
        }

        ctx.set_fonts(fonts);
        self.fonts_ready = true;
    }

    fn mobile_shell(&mut self, ui: &mut egui::Ui, available: egui::Vec2) {
        let content_height = (available.y - 82.0).max(0.0);
        ui.allocate_ui_with_layout(
            egui::vec2(available.x, content_height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| self.show_page(ui));
            },
        );
        self.bottom_nav(ui);
    }

    fn desktop_shell(&mut self, ui: &mut egui::Ui, available: egui::Vec2) {
        let p = self.palette();
        let side_w = 236.0;
        let show_inspector = available.x >= 1180.0;
        let inspector_w = if show_inspector { 292.0 } else { 0.0 };
        let gaps = if show_inspector { 28.0 } else { 14.0 };
        let main_w = (available.x - side_w - inspector_w - gaps).max(520.0);

        ui.allocate_ui_with_layout(
            available,
            egui::Layout::left_to_right(egui::Align::Min),
            |ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(side_w, available.y),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| self.desktop_sidebar(ui),
                );
                ui.add_space(8.0);

                egui::Frame::none()
                    .fill(p.panel)
                    .stroke(Stroke::new(1.0, p.stroke))
                    .rounding(Rounding::same(10.0))
                    .inner_margin(Margin::same(18.0))
                    .show(ui, |ui| {
                        ui.set_width(main_w);
                        ui.set_min_height((available.y - 24.0).max(0.0));
                        self.desktop_topbar(ui);
                        ui.add_space(14.0);
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| self.show_active_tab(ui));
                    });

                if show_inspector {
                    ui.add_space(8.0);
                    ui.allocate_ui_with_layout(
                        egui::vec2(inspector_w, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| self.insight_panel(ui),
                    );
                }
            },
        );
    }

    fn show_page(&mut self, ui: &mut egui::Ui) {
        ui.add_space(self.top_padding());
        self.header(ui);
        ui.add_space(16.0);
        self.show_active_tab(ui);
    }

    fn show_active_tab(&mut self, ui: &mut egui::Ui) {
        match self.current_tab {
            Tab::Home => self.show_home(ui),
            Tab::Discover => self.show_discover(ui),
            Tab::Repository => self.show_repository(ui),
            Tab::Downloads => self.show_downloads(ui),
            Tab::Security => self.show_security(ui),
            Tab::Settings => self.show_settings(ui),
        }
    }

    fn desktop_sidebar(&mut self, ui: &mut egui::Ui) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::symmetric(16.0, 16.0))
            .show(ui, |ui| {
                ui.set_min_height(ui.available_height() - 4.0);
                ui.horizontal(|ui| {
                    self.logo_tile_sized(ui, 46.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("GitMarket").size(20.0).strong().color(p.text));
                        ui.label(RichText::new("Release OS").size(12.0).color(p.muted));
                    });
                });

                ui.add_space(24.0);
                for (tab, label, badge) in [
                    (Tab::Home, self.t("home"), ""),
                    (Tab::Discover, self.t("discover"), ""),
                    (Tab::Repository, self.t("repos"), ""),
                    (Tab::Downloads, self.t("downloads"), ""),
                    (Tab::Security, self.t("security"), ""),
                    (Tab::Settings, self.t("settings_short"), ""),
                ] {
                    if self.sidebar_nav_item(ui, tab, label, badge).clicked() {
                        self.current_tab = tab;
                    }
                    ui.add_space(5.0);
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    self.sidebar_status(ui);
                    ui.add_space(10.0);
                    ui.horizontal_wrapped(|ui| {
                        if self.pill_button(ui, self.language_label()).clicked() {
                            self.language = match self.language {
                                Language::Zh => Language::En,
                                Language::En => Language::Zh,
                            };
                        }
                        if self.pill_button(ui, self.t("theme")).clicked() {
                            self.theme = self.next_theme();
                        }
                    });
                });
            });
    }

    fn desktop_topbar(&mut self, ui: &mut egui::Ui) {
        let p = self.palette();
        let search_label = self.t("search");
        let search_hint = self.t("search_hint");
        let go_label = if self.is_searching {
            self.t("searching")
        } else {
            self.t("go")
        };
        let inspect_label = self.t("inspect_repo");
        egui::Frame::none()
            .fill(p.panel_alt)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::symmetric(14.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(self.t("discover_title"))
                            .size(16.0)
                            .strong()
                            .color(p.text),
                    );
                    ui.label(
                        RichText::new(self.t("discover_sub"))
                            .size(12.0)
                            .color(p.muted),
                    );
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(search_label).strong().color(p.muted));
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.search_input)
                            .hint_text(search_hint)
                            .desired_width((ui.available_width() - 230.0).max(320.0)),
                    );
                    if self.primary_button(ui, go_label).clicked()
                        || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        self.current_tab = Tab::Discover;
                        self.start_discovery();
                    }
                    if self.text_button(ui, inspect_label).clicked() {
                        self.current_tab = Tab::Repository;
                    }
                });
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(self.t("source")).strong().color(p.muted));
                    for source in [
                        SourceChoice::All,
                        SourceChoice::GitHub,
                        SourceChoice::Gitee,
                        SourceChoice::GitCode,
                    ] {
                        let selected = self.active_source == source;
                        if self
                            .compact_segment(ui, self.source_name(source), selected)
                            .clicked()
                        {
                            self.active_source = source;
                            self.current_tab = Tab::Discover;
                            self.start_discovery();
                        }
                    }
                });
            });
    }

    fn insight_panel(&mut self, ui: &mut egui::Ui) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::same(16.0))
            .show(ui, |ui| {
                ui.set_min_height(ui.available_height() - 4.0);
                ui.label(
                    RichText::new(self.t("insight_title"))
                        .size(18.0)
                        .strong()
                        .color(p.text),
                );
                ui.label(RichText::new(self.t("insight_sub")).color(p.muted));
                ui.add_space(14.0);

                self.insight_metric(
                    ui,
                    self.t("source_coverage"),
                    "3",
                    "GitHub / Gitee / GitCode",
                );
                self.insight_metric(
                    ui,
                    self.t("release_assets"),
                    "APK EXE DMG",
                    self.t("upstream_only"),
                );
                self.insight_metric(ui, "SHA256", self.t("verified"), self.t("hashes_desc"));

                ui.add_space(12.0);
                egui::Frame::none()
                    .fill(p.chip)
                    .rounding(Rounding::same(16.0))
                    .inner_margin(Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.label(RichText::new(self.t("next_focus")).strong().color(p.text));
                        ui.add_space(8.0);
                        for item in [
                            self.t("focus_ui"),
                            self.t("focus_lazy"),
                            self.t("focus_signatures"),
                        ] {
                            ui.label(RichText::new(format!("• {}", item)).color(p.muted));
                        }
                    });
            });
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
        if self.compact_layout(ui) {
            self.search_box(ui);
            ui.add_space(12.0);
        }
        self.category_row(ui);
        ui.add_space(14.0);
        self.hero_card(ui);
        ui.add_space(18.0);

        if !self.compact_layout(ui) {
            self.home_metric_strip(ui);
            ui.add_space(18.0);
        }

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

        if self.compact_layout(ui) {
            for repo in self.sample_repos() {
                self.repo_row(ui, &repo);
                ui.add_space(8.0);
            }
        } else {
            self.popular_repo_table(ui);
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

        if self.discover_items.is_empty() && !self.is_searching {
            self.start_discovery();
        }

        if self.compact_layout(ui) {
            self.search_box(ui);
            ui.add_space(10.0);
            self.source_row(ui);
        } else {
            let total = self.discover_items.len();
            self.discover_summary_bar(ui, total);
        }
        ui.add_space(12.0);

        if self.is_searching {
            self.loading_panel(ui, self.t("loading_sources"));
        }

        if let Some(err) = &self.error_message {
            self.message_panel(ui, err, self.palette().danger);
        }

        let items = self.discover_items.clone();
        let visible_count = self.discover_visible.min(items.len());
        let visible_items = &items[..visible_count];
        self.search_results_view(ui, visible_items);

        if items.len() > visible_count {
            ui.add_space(6.0);
            let label = format!(
                "{} {} / {}",
                self.t("show_more"),
                visible_count,
                items.len()
            );
            if self.text_button(ui, &label).clicked() {
                self.discover_visible = (self.discover_visible + 12).min(items.len());
            }
        }

        if items.is_empty() && !self.is_searching {
            self.empty_state(ui, self.t("no_results"), self.t("no_results_sub"));
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
                    .segment(ui, "中文", self.language == Language::Zh)
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
        let go_label = if self.is_searching {
            self.t("searching")
        } else {
            self.t("go")
        };
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

    fn discover_summary_bar(&self, ui: &mut egui::Ui, total: usize) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel_alt)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::symmetric(12.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    self.tag(ui, self.source_name(self.active_source));
                    let query = self.search_input.trim();
                    self.tag(
                        ui,
                        &format!(
                            "{} {}",
                            self.t("current_query"),
                            if query.is_empty() {
                                self.t("default_query")
                            } else {
                                query
                            }
                        ),
                    );
                    self.tag(ui, &format!("{} {}", self.t("results"), total));
                    self.tag(
                        ui,
                        if self.is_searching {
                            self.t("searching")
                        } else {
                            self.t("ready_status")
                        },
                    );
                });
            });
    }

    fn search_results_view(&mut self, ui: &mut egui::Ui, items: &[SearchRepo]) {
        if !self.compact_layout(ui) && ui.available_width() > 760.0 {
            for chunk in items.chunks(2) {
                ui.columns(2, |columns| {
                    for (idx, item) in chunk.iter().enumerate() {
                        self.search_result_card(&mut columns[idx], item);
                    }
                });
                ui.add_space(8.0);
            }
        } else {
            for item in items {
                self.search_result_card(ui, item);
                ui.add_space(8.0);
            }
        }
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
        let wide = ui.available_width() > 720.0;
        egui::Frame::none()
            .fill(p.panel)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(22.0))
            .inner_margin(Margin::same(20.0))
            .show(ui, |ui| {
                if wide {
                    let first_col_width = (ui.available_width() * 0.52).max(360.0);
                    ui.columns(2, |columns| {
                        columns[0].set_width(first_col_width);
                        self.hero_copy(&mut columns[0]);
                        self.release_stack_visual(&mut columns[1]);
                    });
                } else {
                    self.hero_copy(ui);
                    ui.add_space(14.0);
                    self.release_stack_visual(ui);
                }
            });
    }

    fn hero_copy(&mut self, ui: &mut egui::Ui) {
        let p = self.palette();
        ui.horizontal_wrapped(|ui| {
            self.tag(ui, self.t("today"));
            self.tag(ui, "GitHub");
            self.tag(ui, "Gitee");
            self.tag(ui, "GitCode");
        });
        ui.add_space(14.0);
        ui.label(
            RichText::new(self.t("me_hero_title"))
                .size(self.title_size() + 4.0)
                .strong()
                .color(p.text),
        );
        ui.add(egui::Label::new(RichText::new(self.t("me_hero_sub")).color(p.muted)).wrap());
        ui.add_space(18.0);

        ui.horizontal_wrapped(|ui| {
            if self.primary_button(ui, self.t("start_discover")).clicked() {
                self.current_tab = Tab::Discover;
                self.start_discovery();
            }
            if self.text_button(ui, self.t("inspect_repo")).clicked() {
                self.current_tab = Tab::Repository;
            }
        });
    }

    fn release_stack_visual(&self, ui: &mut egui::Ui) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel_alt)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(20.0))
            .inner_margin(Margin::same(14.0))
            .show(ui, |ui| {
                ui.set_min_width(260.0);
                ui.horizontal(|ui| {
                    self.logo_tile_sized(ui, 38.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Release Radar").strong().color(p.text));
                        ui.label(
                            RichText::new("assets, hashes, source")
                                .size(12.0)
                                .color(p.muted),
                        );
                    });
                });
                ui.add_space(12.0);
                for (name, platform, score, color) in [
                    ("termux/termux-app", "Android · GitHub", 0.84, p.accent),
                    ("2dust/v2rayNG", "Kotlin · GitHub", 0.76, p.accent_alt),
                    ("dromara/hutool", "Java · Gitee", 0.58, p.warning),
                ] {
                    self.release_visual_row(ui, name, platform, score, color);
                    ui.add_space(8.0);
                }
            });
    }

    fn release_visual_row(
        &self,
        ui: &mut egui::Ui,
        name: &str,
        platform: &str,
        score: f32,
        color: Color32,
    ) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(14.0))
            .inner_margin(Margin::same(10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (icon_rect, _) =
                        ui.allocate_exact_size(egui::vec2(34.0, 34.0), egui::Sense::hover());
                    ui.painter()
                        .rect_filled(icon_rect, Rounding::same(10.0), color);
                    ui.painter().text(
                        icon_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        name.chars()
                            .next()
                            .unwrap_or('G')
                            .to_uppercase()
                            .to_string(),
                        egui::FontId::proportional(16.0),
                        Color32::WHITE,
                    );
                    ui.vertical(|ui| {
                        ui.label(RichText::new(name).strong().color(p.text));
                        ui.label(RichText::new(platform).size(12.0).color(p.muted));
                    });
                });
                ui.add_space(8.0);
                let width = ui.available_width();
                let (bar_rect, _) =
                    ui.allocate_exact_size(egui::vec2(width, 7.0), egui::Sense::hover());
                ui.painter()
                    .rect_filled(bar_rect, Rounding::same(999.0), p.stroke);
                let filled = egui::Rect::from_min_size(
                    bar_rect.min,
                    egui::vec2(bar_rect.width() * score.clamp(0.0, 1.0), bar_rect.height()),
                );
                ui.painter()
                    .rect_filled(filled, Rounding::same(999.0), color);
            });
    }

    fn home_metric_strip(&self, ui: &mut egui::Ui) {
        ui.columns(4, |columns| {
            self.metric_card(&mut columns[0], self.t("collections"), "12");
            self.metric_card(&mut columns[1], self.t("results"), "20");
            self.metric_card(&mut columns[2], self.t("source_coverage"), "3");
            self.metric_card(&mut columns[3], "SHA256", self.t("verified"));
        });
    }

    fn popular_repo_table(&mut self, ui: &mut egui::Ui) {
        let p = self.palette();
        card(ui, p, |ui| {
            egui::Grid::new("popular_repo_table")
                .num_columns(5)
                .striped(true)
                .spacing(egui::vec2(14.0, 10.0))
                .show(ui, |ui| {
                    ui.label(RichText::new(self.t("repository")).strong().color(p.muted));
                    ui.label(RichText::new(self.t("source")).strong().color(p.muted));
                    ui.label(RichText::new(self.t("language")).strong().color(p.muted));
                    ui.label(RichText::new("Stars").strong().color(p.muted));
                    ui.label("");
                    ui.end_row();

                    for repo in self.sample_repos() {
                        ui.horizontal(|ui| {
                            self.app_icon(ui, &repo);
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&repo.full_name).strong().color(p.text));
                                if let Some(desc) = &repo.description {
                                    ui.label(RichText::new(desc).size(12.0).color(p.muted));
                                }
                            });
                        });
                        self.tag(ui, &repo.source);
                        self.tag(ui, repo.language.as_deref().unwrap_or("Code"));
                        ui.label(
                            RichText::new(format_count(repo.stargazers_count))
                                .strong()
                                .color(p.text),
                        );
                        if self.text_button(ui, self.t("inspect")).clicked() {
                            self.repo_input = repo.full_name.clone();
                            self.current_tab = Tab::Repository;
                            self.fetch_repo();
                        }
                        ui.end_row();
                    }
                });
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
                        if item.topics.iter().any(|topic| topic == "fallback") {
                            self.tag(ui, self.t("local_fallback"));
                        }
                        if let Some(updated) = &item.updated_at {
                            self.tag(
                                ui,
                                &format!(
                                    "{} {}",
                                    self.t("updated"),
                                    &updated[..10.min(updated.len())]
                                ),
                            );
                        }
                    });
                    if !item.topics.is_empty() {
                        ui.add_space(4.0);
                        ui.horizontal_wrapped(|ui| {
                            for topic in item
                                .topics
                                .iter()
                                .filter(|topic| topic.as_str() != "fallback")
                                .take(5)
                            {
                                self.tag(ui, topic);
                            }
                        });
                    }
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
                if let Some(license) = &repo.license {
                    if let Some(name) = license
                        .spdx_id
                        .as_deref()
                        .filter(|value| *value != "NOASSERTION")
                        .or(license.name.as_deref())
                    {
                        self.tag(ui, &format!("{} {}", self.t("license"), name));
                    }
                }
                if let Some(branch) = &repo.default_branch {
                    self.tag(ui, &format!("{} {}", self.t("branch"), branch));
                }
                if let Some(updated) = &repo.updated_at {
                    self.tag(
                        ui,
                        &format!(
                            "{} {}",
                            self.t("updated"),
                            &updated[..10.min(updated.len())]
                        ),
                    );
                }
                if self.text_button(ui, self.t("open_source")).clicked() {
                    ui.ctx()
                        .open_url(egui::OpenUrl::new_tab(repo.html_url.clone()));
                }
            });
            if !repo.topics.is_empty() {
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    for topic in repo.topics.iter().take(8) {
                        self.tag(ui, topic);
                    }
                });
            }
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
                            self.tag(ui, self.t("official_asset"));
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
        self.logo_tile_sized(ui, 54.0);
    }

    fn logo_tile_sized(&self, ui: &mut egui::Ui, size: f32) {
        let p = self.palette();
        let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
        let round = (size * 0.26).round();
        ui.painter()
            .rect_filled(rect, Rounding::same(round), p.accent);

        let glow = egui::Rect::from_center_size(
            rect.center() + egui::vec2(size * 0.16, -size * 0.16),
            egui::vec2(size * 0.42, size * 0.42),
        );
        ui.painter().circle_filled(
            glow.center(),
            size * 0.22,
            Color32::from_rgba_unmultiplied(
                p.accent_alt.r(),
                p.accent_alt.g(),
                p.accent_alt.b(),
                180,
            ),
        );

        let stack_a = egui::Rect::from_min_size(
            rect.left_top() + egui::vec2(size * 0.18, size * 0.22),
            egui::vec2(size * 0.42, size * 0.12),
        );
        let stack_b = stack_a.translate(egui::vec2(size * 0.10, size * 0.14));
        ui.painter().rect_filled(
            stack_a,
            Rounding::same(size * 0.04),
            Color32::from_rgba_unmultiplied(255, 255, 255, 225),
        );
        ui.painter().rect_filled(
            stack_b,
            Rounding::same(size * 0.04),
            Color32::from_rgba_unmultiplied(255, 255, 255, 170),
        );

        ui.painter().text(
            rect.center() + egui::vec2(size * 0.03, size * 0.10),
            egui::Align2::CENTER_CENTER,
            "G",
            egui::FontId::proportional(size * 0.43),
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
            egui::Button::new(RichText::new(label).strong().color(Color32::WHITE))
                .fill(self.palette().accent)
                .stroke(Stroke::new(1.0, self.palette().accent))
                .rounding(Rounding::same(8.0))
                .min_size(egui::vec2(86.0, self.button_height() + 2.0)),
        )
    }

    fn text_button(&self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        ui.add(
            egui::Button::new(label)
                .fill(self.palette().panel_alt)
                .stroke(Stroke::new(1.0, self.palette().stroke))
                .rounding(Rounding::same(8.0))
                .min_size(egui::vec2(72.0, self.button_height() + 2.0)),
        )
    }

    fn pill_button(&self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        ui.add(
            egui::Button::new(label)
                .fill(self.palette().panel)
                .stroke(Stroke::new(1.0, self.palette().stroke))
                .rounding(Rounding::same(8.0))
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
            .stroke(Stroke::new(1.0, if selected { p.accent } else { p.stroke }))
            .rounding(Rounding::same(8.0))
            .min_size(egui::vec2(70.0, self.button_height())),
        )
    }

    fn compact_segment(&self, ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
        let p = self.palette();
        ui.add(
            egui::Button::new(
                RichText::new(label)
                    .size(self.body_size() - 1.0)
                    .color(if selected { Color32::WHITE } else { p.text }),
            )
            .fill(if selected { p.accent } else { p.panel })
            .stroke(Stroke::new(1.0, if selected { p.accent } else { p.stroke }))
            .rounding(Rounding::same(8.0))
            .min_size(egui::vec2(72.0, 32.0)),
        )
    }

    fn sidebar_nav_item(
        &self,
        ui: &mut egui::Ui,
        tab: Tab,
        label: &str,
        _badge: &str,
    ) -> egui::Response {
        let p = self.palette();
        let selected = self.current_tab == tab;
        let fill = if selected { p.accent } else { p.panel_alt };
        let text = if selected { Color32::WHITE } else { p.text };

        ui.add(
            egui::Button::new(
                RichText::new(label)
                    .size(self.body_size())
                    .strong()
                    .color(text),
            )
            .fill(fill)
            .stroke(Stroke::new(1.0, if selected { p.accent } else { p.stroke }))
            .rounding(Rounding::same(8.0))
            .min_size(egui::vec2(ui.available_width(), 40.0)),
        )
    }

    fn sidebar_status(&self, ui: &mut egui::Ui) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.chip)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(16.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(self.t("upstream_first"))
                        .strong()
                        .color(p.text),
                );
                ui.add(
                    egui::Label::new(RichText::new(self.t("upstream_first_desc")).color(p.muted))
                        .wrap(),
                );
            });
    }

    fn insight_metric(&self, ui: &mut egui::Ui, title: &str, value: &str, desc: &str) {
        let p = self.palette();
        egui::Frame::none()
            .fill(p.panel_alt)
            .stroke(Stroke::new(1.0, p.stroke))
            .rounding(Rounding::same(16.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.label(RichText::new(title).color(p.muted));
                ui.label(RichText::new(value).size(22.0).strong().color(p.text));
                ui.add(egui::Label::new(RichText::new(desc).size(12.0).color(p.muted)).wrap());
            });
        ui.add_space(10.0);
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
                    let _ = tx.send(AppMessage::RepoInfo(Box::new(info)));
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
        let query = self.search_input.trim().to_string();
        let source = self.active_source;
        let discovery_key = format!("{}|{}", source_id(source), query.to_lowercase());
        if self.is_searching && self.last_discovery_key == discovery_key {
            return;
        }

        self.discovery_request_id = self.discovery_request_id.wrapping_add(1);
        let request_id = self.discovery_request_id;
        self.last_discovery_key = discovery_key;
        self.is_searching = true;
        self.error_message = None;
        self.discover_visible = 12;
        if self.discover_items.is_empty() {
            self.discover_items = self.sample_repos();
        }

        let tx = self.tx.clone();
        let token = self.github_token.clone();

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

            let per_source_limit = if query.is_empty() { 10 } else { 20 };
            let handles: Vec<_> = platforms
                .into_iter()
                .map(|platform| {
                    let client = client.clone();
                    let query = query.clone();
                    thread::spawn(move || {
                        (
                            platform,
                            client.search_repositories(platform, &query, per_source_limit),
                        )
                    })
                })
                .collect();

            let mut merged = Vec::new();
            let mut errors = Vec::new();

            for handle in handles {
                match handle.join() {
                    Ok((_, Ok(mut items))) => merged.append(&mut items),
                    Ok((platform, Err(e))) => {
                        errors.push(format!("{}: {}", platform.label(), e));
                    }
                    Err(_) => errors.push("A source worker stopped unexpectedly.".to_string()),
                }
            }

            let result = if merged.is_empty() {
                Err(if errors.is_empty() {
                    "No repositories matched this query. Try another keyword or source.".to_string()
                } else {
                    errors.join("\n")
                })
            } else {
                merged.sort_by_key(|item| std::cmp::Reverse(item.stargazers_count));
                merged.dedup_by(|a, b| a.full_name == b.full_name && a.source == b.source);
                merged.truncate(if query.is_empty() { 18 } else { 36 });
                Ok(merged)
            };

            let _ = tx.send(AppMessage::DiscoverFinished { request_id, result });
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
                    self.repo_info = Some(*info);
                    self.is_loading = false;
                }
                AppMessage::Releases(releases) => {
                    self.releases = releases;
                    self.is_loading = false;
                }
                AppMessage::DiscoverFinished { request_id, result } => {
                    if request_id != self.discovery_request_id {
                        continue;
                    }
                    self.is_searching = false;
                    match result {
                        Ok(items) => {
                            self.discover_items = items;
                            self.discover_visible = 12;
                        }
                        Err(message) => {
                            self.error_message = Some(message);
                        }
                    }
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
                bg: Color32::from_rgb(244, 247, 252),
                panel: Color32::from_rgb(255, 255, 255),
                panel_alt: Color32::from_rgb(239, 244, 253),
                text: Color32::from_rgb(12, 20, 35),
                muted: Color32::from_rgb(103, 113, 134),
                accent: Color32::from_rgb(39, 80, 186),
                accent_alt: Color32::from_rgb(74, 203, 164),
                warning: Color32::from_rgb(244, 156, 63),
                danger: Color32::from_rgb(224, 68, 82),
                stroke: Color32::from_rgb(222, 230, 243),
                chip: Color32::from_rgb(247, 249, 253),
            },
            ThemeChoice::Warm => ThemePalette {
                bg: Color32::from_rgb(255, 247, 236),
                panel: Color32::from_rgb(255, 252, 246),
                panel_alt: Color32::from_rgb(255, 232, 214),
                text: Color32::from_rgb(55, 27, 15),
                muted: Color32::from_rgb(135, 92, 72),
                accent: Color32::from_rgb(238, 92, 42),
                accent_alt: Color32::from_rgb(247, 184, 72),
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
                bg: Color32::from_rgb(8, 13, 26),
                panel: Color32::from_rgb(15, 23, 43),
                panel_alt: Color32::from_rgb(24, 34, 66),
                text: Color32::from_rgb(237, 244, 255),
                muted: Color32::from_rgb(139, 154, 189),
                accent: Color32::from_rgb(100, 80, 255),
                accent_alt: Color32::from_rgb(40, 217, 188),
                warning: Color32::from_rgb(255, 178, 78),
                danger: Color32::from_rgb(255, 90, 112),
                stroke: Color32::from_rgb(42, 55, 92),
                chip: Color32::from_rgb(18, 28, 54),
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

    fn compact_layout(&self, ui: &egui::Ui) -> bool {
        self.is_android || ui.available_width() < 760.0
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
            (Language::Zh, Category::Trending) => "趋势",
            (Language::Zh, Category::Editors) => "编辑器",
            (Language::Zh, Category::DevTools) => "开发工具",
            (Language::Zh, Category::Android) => "Android",
            (Language::Zh, Category::Utilities) => "实用工具",
            (Language::En, Category::Trending) => "Trending",
            (Language::En, Category::Editors) => "Editors",
            (Language::En, Category::DevTools) => "Dev Tools",
            (Language::En, Category::Android) => "Android",
            (Language::En, Category::Utilities) => "Utilities",
        }
    }

    fn source_name(&self, source: SourceChoice) -> &'static str {
        match (self.language, source) {
            (Language::Zh, SourceChoice::All) => "全部来源",
            (_, SourceChoice::GitHub) => "GitHub",
            (_, SourceChoice::Gitee) => "Gitee",
            (_, SourceChoice::GitCode) => "GitCode",
            (Language::En, SourceChoice::All) => "All Sources",
        }
    }

    fn theme_name(&self, choice: ThemeChoice) -> &'static str {
        match (self.language, choice) {
            (Language::Zh, ThemeChoice::MeAgent) => "ME Agent",
            (Language::Zh, ThemeChoice::Warm) => "暖色卡片",
            (Language::Zh, ThemeChoice::Clean) => "清爽蓝白",
            (Language::Zh, ThemeChoice::Launch) => "橙色发布",
            (Language::En, ThemeChoice::MeAgent) => "ME Agent",
            (Language::En, ThemeChoice::Warm) => "Warm",
            (Language::En, ThemeChoice::Clean) => "Clean",
            (Language::En, ThemeChoice::Launch) => "Launch",
        }
    }

    fn language_label(&self) -> &'static str {
        match self.language {
            Language::Zh => "中文",
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
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::same(16.0))
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
        .stroke(Stroke::new(1.0, if selected { p.accent } else { p.stroke }))
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

fn register_font_family(fonts: &mut egui::FontDefinitions, name: &str, index: usize) {
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        let entries = fonts.families.entry(family).or_default();
        if entries.iter().any(|entry| entry == name) {
            continue;
        }
        entries.insert(index.min(entries.len()), name.to_string());
    }
}

fn cjk_system_font_candidates() -> &'static [(&'static str, &'static str)] {
    &[
        ("noto_sans_sc_windows", "C:/Windows/Fonts/NotoSansSC-VF.ttf"),
        ("noto_sans_sc_android_vf", "/system/fonts/NotoSansSC-VF.ttf"),
        (
            "noto_sans_sc_android_regular",
            "/system/fonts/NotoSansSC-Regular.otf",
        ),
        (
            "android_droid_fallback",
            "/system/fonts/DroidSansFallback.ttf",
        ),
        (
            "source_han_sans_cn",
            "/system/fonts/SourceHanSansCN-Regular.otf",
        ),
    ]
}

fn source_id(source: SourceChoice) -> &'static str {
    match source {
        SourceChoice::All => "all",
        SourceChoice::GitHub => "github",
        SourceChoice::Gitee => "gitee",
        SourceChoice::GitCode => "gitcode",
    }
}

fn zh(key: &str) -> &'static str {
    match key {
        "today" => "今日看板",
        "me_hero_title" => "你的 Release 智能看板",
        "me_hero_sub" => "ME Agent 式移动工作台：聚合仓库、版本、资产、安全信号和下载进度。",
        "collections" => "合集",
        "memory" => "记忆",
        "results" => "搜索结果",
        "verified" => "待校验",
        "start_discover" => "开始发现",
        "inspect_repo" => "检查仓库",
        "tagline" => "发现 GitHub / Gitee / GitCode Release 资产",
        "settings" => "设置",
        "settings_short" => "设置",
        "theme" => "主题",
        "language" => "语言",
        "repository" => "仓库",
        "source" => "来源",
        "insight_title" => "发布洞察",
        "insight_sub" => "把 Release、来源与安全信号放到一屏。",
        "source_coverage" => "来源覆盖",
        "next_focus" => "下一步重点",
        "focus_ui" => "移动端真实界面继续向原生体验靠拢",
        "focus_lazy" => "搜索结果分页与懒加载，避免卡顿",
        "focus_signatures" => "签名指纹、许可证和校验链补齐",
        "upstream_first" => "上游优先",
        "upstream_first_desc" => "只链接官方 Release，不托管、不重签二进制。",
        "home" => "首页",
        "discover" => "发现",
        "repos" => "仓库",
        "downloads" => "下载",
        "security" => "安全",
        "search" => "搜索",
        "search_hint" => "搜索仓库、工具、作者或 Release 关键词",
        "go" => "搜索",
        "searching" => "搜索中",
        "ready_status" => "就绪",
        "current_query" => "关键词",
        "default_query" => "默认发现",
        "show_more" => "显示更多",
        "popular" => "热门仓库",
        "view_all" => "查看全部",
        "featured" => "Featured Release",
        "featured_desc" => "更漂亮、更安全地发现开源软件包，并回到上游 Release 下载。",
        "fast" => "快速",
        "secure" => "安全",
        "view_releases" => "查看 Release",
        "discover_title" => "多来源发现",
        "discover_sub" => "并发搜索 GitHub 与 Gitee，GitCode 提供源站跳转兜底。",
        "loading_sources" => "正在加载来源结果...",
        "no_results" => "没有可展示结果",
        "no_results_sub" => "接口不可用时会自动使用本地兜底集合；你也可以换一个关键词或来源。",
        "license" => "许可证",
        "branch" => "默认分支",
        "updated" => "更新",
        "official_asset" => "官方 Release 资产",
        "local_fallback" => "本地兜底结果",
        "gitcode_fallback" => "GitCode 当前作为源站搜索入口接入，后续可通过后端代理提供稳定 API。",
        "open_gitcode" => "打开 GitCode 搜索",
        "repo_title" => "仓库检查",
        "repo_sub" => "输入 owner/repo 或上游 URL，检查 Release 资产、下载量、SHA 与源码地址。",
        "inspect" => "检查",
        "loading_repo" => "正在读取仓库和 Release...",
        "release_assets" => "Release 资产",
        "download" => "下载",
        "downloading" => "下载中",
        "open_source" => "源代码",
        "downloads_sub" => "管理下载、校验 SHA256，并在 Android 或桌面上继续安装。",
        "active" => "进行中",
        "completed" => "已完成",
        "verifying" => "待校验",
        "no_downloads" => "还没有下载",
        "no_downloads_sub" => "在仓库页选择 Release 资产后，这里会显示进度和校验信息。",
        "latest_download" => "最近下载",
        "open_downloads" => "打开下载目录",
        "install_apk" => "安装 APK",
        "security_sub" => "GitMarket 是发现和校验助手，不托管、不重签、不替代安全审计。",
        "upstream_only" => "只链接上游",
        "upstream_only_desc" => "所有下载都指向官方 Release 资产，不托管第三方二进制。",
        "hashes" => "SHA256 校验",
        "hashes_desc" => "下载后自动计算 SHA256，后续会与上游校验文件比对。",
        "signatures" => "签名指纹",
        "signatures_desc" => "Android 包需要展示签名证书指纹和历史变更警告。",
        "permissions" => "权限透明",
        "permissions_desc" => "Android 权限必须解释用途，特别是网络和安装相关权限。",
        "token_storage" => "Token 边界",
        "token_storage_desc" => "GitHub Token 仅用于提高 API 限额，正式移动端应使用系统凭据存储。",
        "settings_sub" => "切换主题、语言与 GitHub API Token。",
        "token_help" => "可选。仅用于提高 GitHub API 频率限制，不应在共享设备长期保存。",
        "empty_repo" => "请输入仓库 URL 或 owner/repo。",
        "invalid_repo" => "格式无效。支持 github.com、gitee.com、gitcode.com 或 owner/repo。",
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
        "repository" => "Repository",
        "source" => "Source",
        "insight_title" => "Release Insight",
        "insight_sub" => "Source, assets, and safety signals in one working view.",
        "source_coverage" => "Sources",
        "next_focus" => "Next Focus",
        "focus_ui" => "Move mobile UI closer to native product quality",
        "focus_lazy" => "Paginate and lazy-load search results",
        "focus_signatures" => "Add certificate fingerprints, licenses, and checksum chains",
        "upstream_first" => "Upstream first",
        "upstream_first_desc" => "Link official Releases only. No hosting, mirroring, or re-signing.",
        "home" => "Home",
        "discover" => "Discover",
        "repos" => "Repos",
        "downloads" => "Downloads",
        "security" => "Security",
        "search" => "Search",
        "search_hint" => "Search repositories, tools, authors, or release keywords",
        "go" => "Search",
        "searching" => "Searching",
        "ready_status" => "Ready",
        "current_query" => "Query",
        "default_query" => "Default discover",
        "show_more" => "Show more",
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
        "no_results" => "No displayable results",
        "no_results_sub" => "GitMarket falls back to a local source set when APIs are unavailable. Try another keyword or source.",
        "license" => "License",
        "branch" => "Default branch",
        "updated" => "Updated",
        "official_asset" => "Official Release asset",
        "local_fallback" => "Local fallback",
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
