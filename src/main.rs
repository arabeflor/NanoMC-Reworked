#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod i18n;
mod news;

use eframe::egui::{
    self, Color32, CornerRadius, FontId, Image, Layout, Margin, RichText, Stroke, Vec2,
};
use eframe::egui::{TextureHandle, TextureOptions};
use i18n::{Language, Texts};
use serde::{Deserialize, Serialize};
use std::fs;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

const GAME_VERSION: &str = "1.12.2";
const FORGE_VERSION: &str = "14.23.5.2859";
const REQUIRED_JARS: &[&str] = &[
    "forge-1.12.2-14.23.5.2859-universal.jar",
    "launchwrapper-1.12.jar",
    "asm-debug-all-5.2.jar",
    "jopt-simple-5.0.3.jar",
    "guava-21.0.jar",
    "gson-2.8.0.jar",
    "authlib-1.5.25.jar",
    "realms-1.10.22.jar",
    "commons-lang3-3.5.jar",
    "commons-io-2.5.jar",
    "commons-codec-1.10.jar",
    "commons-compress-1.8.1.jar",
    "httpclient-4.3.3.jar",
    "commons-logging-1.1.3.jar",
    "httpcore-4.3.2.jar",
    "netty-all-4.1.9.Final.jar",
    "fastutil-7.1.0.jar",
    "jline-3.5.1.jar",
    "jna-4.4.0.jar",
    "jna-platform-4.4.0.jar",
    "oshi-core-1.1.jar",
    "icu4j-core-mojang-51.2.jar",
    "patchy-1.3.9.jar",
    "trove4j-3.0.3.jar",
    "vecmath-1.5.2.jar",
    "lzma-0.0.1.jar",
    "jinput-2.0.5.jar",
    "jutils-1.0.0.jar",
    "lwjgl-2.9.4-nightly-20150209.jar",
    "lwjgl_util-2.9.4-nightly-20150209.jar",
    "codecjorbis-20101023.jar",
    "codecwav-20101023.jar",
    "libraryjavasound-20101123.jar",
    "librarylwjglopenal-20100824.jar",
    "soundsystem-20120107.jar",
    "text2speech-1.10.3.jar",
    "log4j-api-2.8.1.jar",
    "log4j-core-2.8.1.jar",
    "akka-actor_2.11-2.3.3.jar",
    "config-1.2.1.jar",
    "scala-actors-migration_2.11-1.1.0.jar",
    "scala-compiler-2.11.1.jar",
    "scala-continuations-library_2.11-1.0.2_mc.jar",
    "scala-continuations-plugin_2.11.1-1.0.2_mc.jar",
    "scala-library-2.11.1.jar",
    "scala-parser-combinators_2.11-1.0.1.jar",
    "scala-reflect-2.11.1.jar",
    "scala-swing_2.11-1.0.1.jar",
    "scala-xml_2.11-1.0.2.jar",
    "maven-artifact-3.5.3.jar",
];

const BACKGROUND: Color32 = Color32::from_rgb(7, 12, 23);
const PANEL: Color32 = Color32::from_rgb(12, 19, 34);
const CARD: Color32 = Color32::from_rgb(16, 24, 43);
const MUTED: Color32 = Color32::from_rgb(139, 158, 198);
const TEXT: Color32 = Color32::from_rgb(229, 235, 250);
const PURPLE: Color32 = Color32::from_rgb(102, 91, 255);
const BORDER: Color32 = Color32::from_rgb(34, 45, 72);
const HOVER: Color32 = Color32::from_rgb(31, 42, 69);

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Mods,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    language: String,
    theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en".to_owned(),
            theme: "default".to_owned(),
        }
    }
}

struct LauncherApp {
    root: PathBuf,
    game_dir: PathBuf,
    settings_path: PathBuf,
    settings: Settings,
    page: Page,
    player_name: String,
    mods: Vec<String>,
    mod_search: String,
    status_message: Option<String>,
    status_is_error: bool,
    child: Option<Child>,
    launch_started_at: Option<Instant>,
    hero_texture: Option<TextureHandle>,
    news_texture: Option<TextureHandle>,
    assets_loaded: bool,
}

impl LauncherApp {
    fn new() -> Self {
        let executable_dir = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(Path::to_path_buf));
        let root = executable_dir
            .as_ref()
            .and_then(|directory| {
                directory
                    .ancestors()
                    .find(|ancestor| ancestor.join("mcdata").is_dir())
                    .map(Path::to_path_buf)
            })
            .or_else(|| {
                std::env::current_dir()
                    .ok()
                    .filter(|directory| directory.join("mcdata").is_dir())
            })
            .or(executable_dir)
            .unwrap_or_else(|| PathBuf::from("."));
        let game_dir = root.join("mcdata");
        let settings_path = settings_path(&root);
        let (settings, status_message) = load_settings(&settings_path);
        let status_is_error = status_message.is_some();
        let mut app = Self {
            root,
            game_dir,
            settings_path,
            settings,
            page: Page::Home,
            player_name: "Player".to_owned(),
            mods: Vec::new(),
            mod_search: String::new(),
            status_message,
            status_is_error,
            child: None,
            launch_started_at: None,
            hero_texture: None,
            news_texture: None,
            assets_loaded: false,
        };
        if let Err(error) = app.refresh_mods() {
            app.set_error(format!("Could not read installed mods: {error}"));
        }
        app
    }

    fn language(&self) -> Language {
        Language::from_code(&self.settings.language)
    }

    fn refresh_mods(&mut self) -> Result<(), String> {
        let mods_dir = self.game_dir.join("mods");
        let entries =
            fs::read_dir(&mods_dir).map_err(|error| format!("{}: {error}", mods_dir.display()))?;
        let mut mods = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|error| error.to_string())?;
            let path = entry.path();
            if path.is_file()
                && path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("jar"))
            {
                if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                    mods.push(name.to_owned());
                }
            }
        }
        mods.sort_by_key(|name| name.to_lowercase());
        self.mods = mods;
        Ok(())
    }

    fn set_error(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_is_error = true;
    }

    fn set_notice(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_is_error = false;
    }

    fn save_settings(&mut self) {
        let result = (|| {
            if let Some(parent) = self.settings_path.parent() {
                fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            let contents =
                serde_json::to_string_pretty(&self.settings).map_err(|error| error.to_string())?;
            fs::write(&self.settings_path, contents).map_err(|error| error.to_string())
        })();
        match result {
            Ok(()) => self.status_message = None,
            Err(error) => self.set_error(format!("Could not save launcher settings: {error}")),
        }
    }

    fn load_assets(&mut self, ctx: &egui::Context) {
        if self.assets_loaded {
            return;
        }
        self.assets_loaded = true;
        let hero_path = self.root.join("img").join("hero.png");
        let news_path = self.root.join("img").join("latest-news.png");
        match load_texture(ctx, &hero_path, "launcher-hero") {
            Ok(texture) => self.hero_texture = texture,
            Err(error) => {
                self.set_error(error);
                return;
            }
        }
        match load_texture(ctx, &news_path, "launcher-news") {
            Ok(texture) => self.news_texture = texture,
            Err(error) => self.set_error(error),
        }
    }

    fn launch_game(&mut self) {
        let texts = self.language().texts();
        if self.child.is_some() {
            self.set_notice(texts.already_running.to_owned());
            return;
        }
        let java = self
            .game_dir
            .join("java")
            .join("jre-legacy")
            .join("bin")
            .join("java.exe");
        if !java.is_file() {
            self.set_error(format!("{}: {}", texts.java_missing, java.display()));
            return;
        }

        let classpath = match build_classpath(&self.game_dir) {
            Ok(classpath) => classpath,
            Err(error) => {
                self.set_error(format!("{}: {error}", texts.launch_failed));
                return;
            }
        };
        let natives = self.game_dir.join("natives").join(GAME_VERSION);
        let assets = self.game_dir.join("assets");
        let username = if self.player_name.trim().is_empty() {
            "Player"
        } else {
            self.player_name.trim()
        };
        let mut command = Command::new(&java);
        command
            .current_dir(&self.game_dir)
            .arg(format!("-Djava.library.path={}", natives.display()))
            .arg("-Dfml.ignoreInvalidMinecraftCertificates=true")
            .arg("-Dfml.ignorePatchDiscrepancies=true")
            .arg("-Dlog4j2.formatMsgNoLookups=true")
            .arg("-cp")
            .arg(classpath)
            .arg("net.minecraft.launchwrapper.Launch")
            .arg("--username")
            .arg(username)
            .arg("--version")
            .arg(GAME_VERSION)
            .arg("--gameDir")
            .arg(".")
            .arg("--assetsDir")
            .arg(assets)
            .arg("--assetIndex")
            .arg("1.12")
            .arg("--accessToken")
            .arg("0")
            .arg("--userProperties")
            .arg("{}")
            .arg("--uuid")
            .arg("00000000000000000000000000000000")
            .arg("--userType")
            .arg("legacy")
            .arg("--versionType")
            .arg("Forge")
            .arg("--tweakClass")
            .arg("net.minecraftforge.fml.common.launcher.FMLTweaker");
        #[cfg(windows)]
        command.creation_flags(0x08000000);
        let result = command.spawn();

        match result {
            Ok(child) => {
                self.child = Some(child);
                self.launch_started_at = Some(Instant::now());
                self.set_notice(texts.launching.to_owned());
            }
            Err(error) => self.set_error(format!("{}: {error}", texts.launch_failed)),
        }
    }

    fn open_folder(&mut self, folder: &str, label: &str) {
        let path = self.game_dir.join(folder);
        if let Err(error) = Command::new("explorer").arg(&path).spawn() {
            self.set_error(format!("{label}: {error} ({})", path.display()));
        }
    }

    fn set_theme(&mut self, theme: &str) {
        self.settings.theme = theme.to_owned();
        self.save_settings();
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_assets(ctx);
        if let Some(child) = self.child.as_mut() {
            ctx.request_repaint_after(Duration::from_millis(250));
            match child.try_wait() {
                Ok(Some(exit)) => {
                    self.child = None;
                    self.launch_started_at = None;
                    self.set_notice(format!(
                        "{} ({})",
                        self.language().texts().game_closed,
                        exit
                    ));
                }
                Ok(None) => {}
                Err(error) => {
                    self.child = None;
                    self.launch_started_at = None;
                    self.set_error(format!("Could not monitor Minecraft: {error}"));
                }
            }
        }
        if self
            .launch_started_at
            .is_some_and(|started| started.elapsed() >= Duration::from_secs(30))
        {
            self.launch_started_at = None;
            self.set_notice(self.language().texts().game_running.to_owned());
        }

        let accent = match self.settings.theme.as_str() {
            "midnight" => Color32::from_rgb(65, 145, 255),
            "violet" => Color32::from_rgb(177, 91, 255),
            _ => PURPLE,
        };
        ctx.set_visuals(egui::Visuals::dark());
        let mut visuals = ctx.style().visuals.clone();
        visuals.panel_fill = BACKGROUND;
        visuals.window_fill = PANEL;
        visuals.widgets.noninteractive.bg_fill = PANEL;
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(20, 29, 50);
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(20, 29, 50);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, BORDER);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT);
        visuals.widgets.inactive.corner_radius = CornerRadius::same(8);
        visuals.widgets.hovered.bg_fill = HOVER;
        visuals.widgets.hovered.weak_bg_fill = HOVER;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, accent.gamma_multiply(0.75));
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(8);
        visuals.widgets.active.bg_fill = accent.gamma_multiply(0.34);
        visuals.widgets.active.weak_bg_fill = accent.gamma_multiply(0.34);
        visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, accent);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
        visuals.widgets.active.corner_radius = CornerRadius::same(8);
        visuals.widgets.open.bg_fill = Color32::from_rgb(24, 34, 58);
        visuals.widgets.open.weak_bg_fill = Color32::from_rgb(24, 34, 58);
        visuals.widgets.open.bg_stroke = Stroke::new(1.0_f32, accent.gamma_multiply(0.65));
        visuals.widgets.open.corner_radius = CornerRadius::same(8);
        visuals.selection.bg_fill = accent;
        visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);
        ctx.set_visuals(visuals);
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = Vec2::new(9.0, 9.0);
        style.spacing.button_padding = Vec2::new(12.0, 8.0);
        style.spacing.interact_size.y = 34.0;
        ctx.set_style(style);

        let texts = self.language().texts();
        self.show_sidebar(ctx, texts, accent);
        self.show_topbar(ctx, texts);
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BACKGROUND)
                    .inner_margin(Margin::same(20)),
            )
            .show(ctx, |ui| match self.page {
                Page::Home => self.show_home(ui, texts, accent),
                Page::Mods => self.show_mods(ui, texts),
            });
    }
}

impl LauncherApp {
    fn show_sidebar(&mut self, ctx: &egui::Context, texts: &Texts, accent: Color32) {
        egui::SidePanel::left("navigation")
            .exact_width(224.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(PANEL)
                    .inner_margin(Margin::symmetric(14, 20))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(25, 36, 59))),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("NanoMC").size(21.0).strong().color(TEXT));
                        ui.label(RichText::new("Reworked").size(13.0).strong().color(accent));
                    });
                });
                ui.add_space(34.0);
                self.nav_button(ui, Page::Home, texts.home, accent);
                ui.add_space(6.0);
                self.nav_button(ui, Page::Mods, texts.mods, accent);

                ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("NanoMC Reworked v{}", news::LATEST_NEWS.version))
                            .size(11.0)
                            .color(MUTED),
                    );
                    ui.horizontal(|ui| {
                        status_indicator(ui, Color32::from_rgb(0, 205, 145));
                        ui.label(
                            RichText::new(self.status_message.as_deref().unwrap_or(texts.ready))
                                .size(12.0)
                                .color(if self.status_is_error {
                                    Color32::from_rgb(255, 117, 134)
                                } else {
                                    MUTED
                                }),
                        );
                    });
                });
            });
    }

    fn nav_button(&mut self, ui: &mut egui::Ui, page: Page, label: &str, accent: Color32) {
        let active = self.page == page;
        if ui
            .add_sized(
                [ui.available_width(), 46.0],
                egui::Button::new(RichText::new(label).size(14.0).strong().color(if active {
                    TEXT
                } else {
                    MUTED
                }))
                .fill(if active {
                    accent.gamma_multiply(0.19)
                } else {
                    PANEL
                })
                .stroke(if active {
                    Stroke::new(1.0_f32, accent.gamma_multiply(0.55))
                } else {
                    Stroke::new(1.0_f32, Color32::TRANSPARENT)
                })
                .corner_radius(CornerRadius::same(10)),
            )
            .clicked()
        {
            self.page = page;
            if page == Page::Mods {
                if let Err(error) = self.refresh_mods() {
                    self.set_error(format!("Could not read installed mods: {error}"));
                }
            }
        }
    }

    fn show_topbar(&mut self, ctx: &egui::Context, texts: &Texts) {
        egui::TopBottomPanel::top("topbar")
            .exact_height(46.0)
            .frame(
                egui::Frame::new()
                    .fill(PANEL)
                    .inner_margin(Margin::symmetric(22, 8))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(25, 36, 59))),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("Minecraft {GAME_VERSION} + Forge {FORGE_VERSION}"))
                            .size(12.0)
                            .color(MUTED),
                    );
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::ComboBox::from_id_salt("language")
                            .selected_text(self.language().label())
                            .width(174.0)
                            .show_ui(ui, |ui| {
                                for language in Language::ALL {
                                    if ui
                                        .selectable_label(
                                            self.language() == language,
                                            language.label(),
                                        )
                                        .clicked()
                                    {
                                        self.settings.language = language.code().to_owned();
                                        self.save_settings();
                                    }
                                }
                            });
                        ui.label(RichText::new(texts.language).color(MUTED).size(12.0));
                    });
                });
            });
    }

    fn show_home(&mut self, ui: &mut egui::Ui, texts: &Texts, accent: Color32) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let available_width = ui.available_width();
            let side_width = 254.0_f32.min((available_width * 0.31).max(232.0));
            let main_width = (available_width - side_width - 18.0).max(330.0);
            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    Vec2::new(main_width, ui.available_height()),
                    Layout::top_down(egui::Align::Min),
                    |ui| {
                        self.show_hero(ui, texts, accent);
                        ui.add_space(18.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(texts.latest_news)
                                    .size(16.0)
                                    .strong()
                                    .color(TEXT),
                            );
                        });
                        ui.add_space(8.0);
                        self.show_news(ui, texts, accent);
                    },
                );
                ui.allocate_ui_with_layout(
                    Vec2::new(side_width, ui.available_height()),
                    Layout::top_down(egui::Align::Min),
                    |ui| self.show_quick_panel(ui, texts),
                );
            });
        });
    }

    fn show_hero(&mut self, ui: &mut egui::Ui, texts: &Texts, accent: Color32) {
        let frame = egui::Frame::new()
            .fill(Color32::from_rgb(19, 27, 53))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(41, 50, 91)))
            .corner_radius(CornerRadius::same(14))
            .inner_margin(Margin::same(24));
        frame.show(ui, |ui| {
            ui.set_min_height(207.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add_space(12.0);
                    ui.label(RichText::new(texts.welcome_back).size(19.0).color(MUTED));
                    egui::Frame::new()
                        .fill(Color32::from_rgb(12, 20, 40))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(42, 52, 88)))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::symmetric(9, 4))
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.player_name)
                                    .desired_width(187.0)
                                    .font(FontId::proportional(30.0))
                                    .text_color(accent)
                                    .frame(false)
                                    .hint_text(texts.player),
                            );
                        });
                    ui.label(RichText::new(texts.adventure).size(15.0).color(MUTED));
                    ui.add_space(15.0);
                    if self.child.is_some() {
                        ui.horizontal(|ui| {
                            let loading = self.launch_started_at.is_some();
                            if loading {
                                ui.spinner();
                            }
                            let label = if loading {
                                texts.launching
                            } else {
                                texts.game_running
                            };
                            let button = egui::Button::new(
                                RichText::new(label)
                                    .size(15.0)
                                    .strong()
                                    .color(Color32::WHITE),
                            )
                            .fill(accent)
                            .corner_radius(CornerRadius::same(8));
                            ui.add_enabled_ui(false, |ui| {
                                ui.add_sized([ui.available_width().min(180.0), 50.0], button);
                            });
                        });
                    } else {
                        let play_button = egui::Button::new(
                            RichText::new(format!("▶   {}", texts.play))
                                .size(16.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(accent)
                        .stroke(Stroke::new(1.0_f32, accent.gamma_multiply(0.82)))
                        .corner_radius(CornerRadius::same(8));
                        if ui
                            .add_sized([ui.available_width().min(208.0), 50.0], play_button)
                            .clicked()
                        {
                            self.launch_game();
                        }
                    }
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(texture) = &self.hero_texture {
                        let image_width = (ui.available_width() * 0.43).clamp(120.0, 240.0);
                        ui.add(
                            Image::from_texture(texture)
                                .fit_to_exact_size(Vec2::new(image_width, 166.0))
                                .corner_radius(CornerRadius::same(10)),
                        );
                    } else {
                        landscape_art(ui, accent);
                    }
                });
            });
        });
    }

    fn show_quick_panel(&mut self, ui: &mut egui::Ui, texts: &Texts) {
        card(ui, |ui| {
            ui.label(RichText::new(texts.game_info).strong().color(TEXT));
            ui.add_space(12.0);
            info_row(ui, texts.version, GAME_VERSION);
            info_row(ui, texts.forge, FORGE_VERSION);
            info_row(ui, texts.java, "8 (x64)");
            ui.horizontal(|ui| {
                ui.label(RichText::new(texts.status).size(12.0).color(MUTED));
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(texts.ready)
                            .size(12.0)
                            .color(Color32::from_rgb(0, 205, 145)),
                    );
                    status_indicator(ui, Color32::from_rgb(0, 205, 145));
                });
            });
        });
        ui.add_space(10.0);
        card(ui, |ui| {
            ui.label(RichText::new(texts.quick_access).strong().color(TEXT));
            ui.add_space(8.0);
            self.folder_button(ui, "mods", texts.open_mods);
            self.folder_button(ui, "resourcepacks", texts.open_resourcepacks);
            self.folder_button(ui, "config", texts.open_config);
            self.folder_button(ui, "saves", texts.open_saves);
        });
        ui.add_space(10.0);
        card(ui, |ui| {
            ui.label(RichText::new(texts.theme).strong().color(TEXT));
            ui.add_space(4.0);
            egui::ComboBox::from_id_salt("theme")
                .selected_text(theme_label(&self.settings.theme, texts))
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for (value, label) in [
                        ("default", texts.theme_default),
                        ("midnight", texts.theme_midnight),
                        ("violet", texts.theme_violet),
                    ] {
                        if ui
                            .selectable_label(self.settings.theme == value, label)
                            .clicked()
                        {
                            self.set_theme(value);
                        }
                    }
                });
            ui.add_space(14.0);
            ui.label(RichText::new(texts.language).strong().color(TEXT));
            ui.add_space(4.0);
            egui::ComboBox::from_id_salt("language_side")
                .selected_text(self.language().label())
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    for language in Language::ALL {
                        if ui
                            .selectable_label(self.language() == language, language.label())
                            .clicked()
                        {
                            self.settings.language = language.code().to_owned();
                            self.save_settings();
                        }
                    }
                });
        });
    }

    fn folder_button(&mut self, ui: &mut egui::Ui, folder: &str, label: &str) {
        if ui
            .add_sized(
                [ui.available_width(), 38.0],
                egui::Button::new(RichText::new(label).size(11.5).strong().color(TEXT))
                    .fill(Color32::from_rgb(20, 29, 50))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(29, 40, 64)))
                    .corner_radius(CornerRadius::same(8)),
            )
            .clicked()
        {
            self.open_folder(folder, label);
        }
    }

    fn show_news(&mut self, ui: &mut egui::Ui, texts: &Texts, accent: Color32) {
        egui::Frame::new()
            .fill(CARD)
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(38, 47, 78)))
            .corner_radius(CornerRadius::same(11))
            .inner_margin(Margin::same(13))
            .show(ui, |ui| {
                ui.set_min_height(115.0);
                ui.horizontal(|ui| {
                    if let Some(texture) = &self.news_texture {
                        ui.add(
                            Image::from_texture(texture)
                                .fit_to_exact_size(Vec2::new(144.0, 100.0))
                                .corner_radius(CornerRadius::same(8)),
                        );
                    } else {
                        news_art(ui, accent);
                    }
                    ui.add_space(9.0);
                    ui.vertical(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new(news::LATEST_NEWS.title)
                                .size(18.0)
                                .strong()
                                .color(TEXT),
                        );
                        ui.label(
                            RichText::new(format!("v{}", news::LATEST_NEWS.version))
                                .size(20.0)
                                .strong()
                                .color(accent),
                        );
                        ui.label(
                            RichText::new(texts.news_description)
                                .size(12.0)
                                .color(MUTED),
                        );
                    });
                });
            });
    }

    fn show_mods(&mut self, ui: &mut egui::Ui, texts: &Texts) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(RichText::new(texts.mods).color(TEXT));
                ui.label(
                    RichText::new(format!("{} {}", self.mods.len(), texts.installed_mods))
                        .color(MUTED),
                );
            });
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(texts.open_mods).clicked() {
                    self.open_folder("mods", texts.folder_failed);
                }
                if ui.button(format!("↻  {}", texts.refresh)).clicked() {
                    match self.refresh_mods() {
                        Ok(()) => self.set_notice(texts.mods_refreshed.to_owned()),
                        Err(error) => self.set_error(format!("{}: {error}", texts.mods_failed)),
                    }
                }
            });
        });
        ui.add_space(14.0);
        ui.add_sized(
            [ui.available_width().min(440.0), 36.0],
            egui::TextEdit::singleline(&mut self.mod_search)
                .hint_text(texts.search_mods)
                .margin(Margin::symmetric(10, 8)),
        );
        ui.add_space(11.0);
        let search = self.mod_search.trim().to_lowercase();
        let mods: Vec<String> = self
            .mods
            .iter()
            .filter(|name| name.to_lowercase().contains(&search))
            .cloned()
            .collect();
        egui::ScrollArea::vertical().show(ui, |ui| {
            if mods.is_empty() {
                egui::Frame::new()
                    .fill(CARD)
                    .corner_radius(CornerRadius::same(10))
                    .inner_margin(Margin::same(24))
                    .show(ui, |ui| {
                        ui.label(RichText::new(texts.no_mods).size(17.0).strong().color(TEXT));
                        ui.add_space(5.0);
                        ui.label(RichText::new(texts.mods_hint).color(MUTED));
                    });
            } else {
                for name in mods {
                    egui::Frame::new()
                        .fill(CARD)
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(31, 41, 66)))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::symmetric(14, 10))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(display_mod_name(&name)).strong().color(TEXT),
                                    );
                                    ui.label(RichText::new(name).size(11.0).color(MUTED));
                                });
                            });
                        });
                    ui.add_space(6.0);
                }
            }
        });
    }
}

fn card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(CARD)
        .stroke(Stroke::new(1.0_f32, BORDER))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::same(15))
        .show(ui, add_contents);
}

fn info_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).size(12.0).color(MUTED));
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(value).size(12.0).color(TEXT));
        });
    });
}

fn status_indicator(ui: &mut egui::Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(9.0), egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), 3.5, color);
}

fn landscape_art(ui: &mut egui::Ui, accent: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(240.0, 150.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, CornerRadius::same(9), Color32::from_rgb(19, 29, 68));
    painter.circle_filled(
        egui::pos2(rect.right() - 47.0, rect.top() + 40.0),
        23.0,
        Color32::from_rgb(255, 147, 160),
    );
    painter.add(egui::Shape::convex_polygon(
        vec![
            egui::pos2(rect.left(), rect.bottom() - 52.0),
            egui::pos2(rect.left() + 56.0, rect.top() + 51.0),
            egui::pos2(rect.left() + 112.0, rect.bottom() - 42.0),
            egui::pos2(rect.left() + 165.0, rect.top() + 73.0),
            egui::pos2(rect.right(), rect.bottom() - 45.0),
            egui::pos2(rect.right(), rect.bottom()),
            egui::pos2(rect.left(), rect.bottom()),
        ],
        Color32::from_rgb(31, 47, 91),
        Stroke::NONE,
    ));
    painter.rect_filled(
        egui::Rect::from_min_max(
            egui::pos2(rect.left(), rect.bottom() - 31.0),
            rect.right_bottom(),
        ),
        0,
        accent.gamma_multiply(0.22),
    );
}

fn news_art(ui: &mut egui::Ui, accent: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(144.0, 100.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(34, 17, 63));
    painter.circle_filled(
        egui::pos2(rect.center().x, rect.center().y),
        30.0,
        accent.gamma_multiply(0.32),
    );
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "N",
        FontId::proportional(42.0),
        Color32::from_rgb(202, 151, 255),
    );
}

fn display_mod_name(filename: &str) -> String {
    filename
        .strip_suffix(".jar")
        .or_else(|| filename.strip_suffix(".JAR"))
        .unwrap_or(filename)
        .replace(['_', '-'], " ")
}

fn theme_label<'a>(theme: &str, texts: &'a Texts) -> &'a str {
    match theme {
        "midnight" => texts.theme_midnight,
        "violet" => texts.theme_violet,
        _ => texts.theme_default,
    }
}

fn settings_path(root: &Path) -> PathBuf {
    root.join("launcher.json")
}

fn load_settings(path: &Path) -> (Settings, Option<String>) {
    match fs::read_to_string(path) {
        Ok(contents) => match serde_json::from_str::<Settings>(&contents) {
            Ok(settings) => (settings, None),
            Err(error) => (
                Settings::default(),
                Some(format!("Could not read launcher settings: {error}")),
            ),
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => (Settings::default(), None),
        Err(error) => (
            Settings::default(),
            Some(format!("Could not read launcher settings: {error}")),
        ),
    }
}

fn load_texture(
    ctx: &egui::Context,
    path: &Path,
    texture_name: &str,
) -> Result<Option<TextureHandle>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let image = image::open(path)
        .map_err(|error| format!("Could not load {}: {error}", path.display()))?
        .to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw());
    Ok(Some(ctx.load_texture(
        texture_name,
        color_image,
        TextureOptions::LINEAR,
    )))
}

fn build_classpath(game_dir: &Path) -> Result<String, String> {
    let client = game_dir
        .join("libraries")
        .join("minecraft-1.12.2-client.jar");
    if !client.is_file() {
        return Err(format!(
            "Minecraft client library is missing: {}",
            client.display()
        ));
    }
    let libraries = game_dir.join("libraries");
    let mut classpath = vec![client.to_string_lossy().into_owned()];
    let mut missing = Vec::new();
    for name in REQUIRED_JARS {
        match find_jar(&libraries, name)? {
            Some(path) => classpath.push(path.to_string_lossy().into_owned()),
            None => missing.push(*name),
        }
    }
    if !missing.is_empty() {
        return Err(format!("Missing libraries: {}", missing.join(", ")));
    }
    let separator = if cfg!(windows) { ";" } else { ":" };
    Ok(classpath.join(separator))
}

fn find_jar(directory: &Path, name: &str) -> Result<Option<PathBuf>, String> {
    let entries =
        fs::read_dir(directory).map_err(|error| format!("{}: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_file() && path.file_name().and_then(|file| file.to_str()) == Some(name) {
            return Ok(Some(path));
        }
        if path.is_dir() {
            if let Some(found) = find_jar(&path, name)? {
                return Ok(Some(found));
            }
        }
    }
    Ok(None)
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("NanoMC Reworked")
            .with_inner_size([1365.0, 768.0])
            .with_min_inner_size([920.0, 620.0])
            .with_icon(window_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "NanoMC Reworked",
        options,
        Box::new(|_cc| Ok(Box::new(LauncherApp::new()))),
    )
}

fn window_icon() -> egui::IconData {
    let image = image::load_from_memory(include_bytes!("../img/app-icon.png"))
        .expect("bundled launcher icon should be a valid PNG")
        .to_rgba8();
    egui::IconData {
        rgba: image.into_raw(),
        width: 256,
        height: 256,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn mod_display_name_removes_archive_suffix_and_separators() {
        assert_eq!(
            display_mod_name("OptiFine_1.12.2_HD_U_G5.JAR"),
            "OptiFine 1.12.2 HD U G5"
        );
    }

    #[test]
    fn launcher_settings_are_kept_in_the_portable_folder() {
        let portable_root = Path::new("portable-launcher");
        assert_eq!(
            settings_path(portable_root),
            portable_root.join("launcher.json")
        );
    }

    #[test]
    fn classpath_includes_the_client_and_all_required_libraries() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX epoch")
            .as_nanos();
        let temporary_root = std::env::temp_dir().join(format!(
            "nanomc-classpath-test-{}-{unique}",
            std::process::id()
        ));
        let libraries = temporary_root.join("libraries");
        fs::create_dir_all(&libraries).expect("create temporary libraries");
        fs::write(libraries.join("minecraft-1.12.2-client.jar"), []).expect("create client jar");
        for jar in REQUIRED_JARS {
            fs::write(libraries.join(jar), []).expect("create dependency jar");
        }

        let classpath = build_classpath(&temporary_root).expect("all classpath entries exist");
        assert!(classpath.contains("minecraft-1.12.2-client.jar"));
        for jar in REQUIRED_JARS {
            assert!(classpath.contains(jar));
        }
        fs::remove_dir_all(temporary_root).expect("remove temporary test folder");
    }
}
