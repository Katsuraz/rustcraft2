use eframe::egui;
use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}, process::Command};

#[derive(Clone, Serialize, Deserialize)]
struct Settings {
    game_dir: String,
    java_path: String,
    ram_gb: u32,
    selected_version: String,
}
impl Default for Settings {
    fn default() -> Self {
        let dir = std::env::var("APPDATA").map(|v| PathBuf::from(v).join(".minecraft"))
            .unwrap_or_else(|_| PathBuf::from(".minecraft"));
        Self { game_dir: dir.display().to_string(), java_path: "javaw.exe".into(), ram_gb: 4, selected_version: String::new() }
    }
}
fn config_path() -> PathBuf {
    std::env::var("APPDATA").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."))
        .join("RustCraftLauncher").join("settings.json")
}
fn read_settings() -> Settings {
    fs::read_to_string(config_path()).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default()
}
fn save_settings(s: &Settings) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() { fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
    let text = serde_json::to_string_pretty(s).map_err(|e| e.to_string())?;
    fs::write(path, text).map_err(|e| e.to_string())
}
fn installed_versions(dir: &str) -> Vec<String> {
    let path = Path::new(dir).join("versions");
    let mut result: Vec<String> = fs::read_dir(path).ok().into_iter().flatten()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.path().join(format!("{name}.json")).is_file() { Some(name) } else { None }
        }).collect();
    result.sort(); result
}
fn find_official_launcher() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    for key in ["ProgramFiles(x86)", "ProgramFiles"] {
        if let Ok(base) = std::env::var(key) {
            candidates.push(PathBuf::from(&base).join("Minecraft Launcher").join("MinecraftLauncher.exe"));
            candidates.push(PathBuf::from(&base).join("Minecraft Launcher").join("Minecraft.exe"));
        }
    }
    candidates.into_iter().find(|p| p.is_file())
}
fn open_official_launcher() -> Result<(), String> {
    if let Some(exe) = find_official_launcher() {
        return Command::new(exe).spawn().map(|_| ()).map_err(|e| e.to_string());
    }
    // Windows URI protocol registered by the official Microsoft Store launcher, when available.
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer.exe").arg("minecraft:").spawn()
            .map(|_| ()).map_err(|e| format!("Could not open Minecraft protocol: {e}"))
    }
    #[cfg(not(target_os = "windows"))]
    { Err("This launcher milestone targets Windows.".into()) }
}
struct Launcher {
    settings: Settings,
    versions: Vec<String>,
    status: String,
    tab: Tab,
}
#[derive(PartialEq)]
enum Tab { Home, Installations, Settings, About }
impl Launcher {
    fn new() -> Self {
        let settings = read_settings();
        let versions = installed_versions(&settings.game_dir);
        Self { settings, versions, status: "Ready. Minecraft ownership and sign-in are handled by the official launcher.".into(), tab: Tab::Home }
    }
    fn refresh(&mut self) {
        self.versions = installed_versions(&self.settings.game_dir);
        if !self.versions.contains(&self.settings.selected_version) {
            self.settings.selected_version = self.versions.first().cloned().unwrap_or_default();
        }
    }
}
impl eframe::App for Launcher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        ctx.set_style(style);
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("RUSTCRAFT").size(27.0).strong().color(egui::Color32::from_rgb(105, 220, 148)));
                ui.label("LAUNCHER  •  v0.1");
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Home, "Home");
                ui.selectable_value(&mut self.tab, Tab::Installations, "Installations");
                ui.selectable_value(&mut self.tab, Tab::Settings, "Settings");
                ui.selectable_value(&mut self.tab, Tab::About, "About");
            });
            ui.add_space(8.0);
        });
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.add_space(6.0); ui.label(&self.status); ui.add_space(6.0);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            match self.tab {
                Tab::Home => {
                    ui.heading("Minecraft: Java Edition");
                    ui.label("Launch your legitimately installed Minecraft through the official launcher.");
                    ui.add_space(24.0);
                    ui.label("Detected local version");
                    if self.versions.is_empty() { ui.label("No local version metadata found. Install Minecraft through the official launcher first."); }
                    else {
                        egui::ComboBox::from_id_salt("versions")
                            .selected_text(if self.settings.selected_version.is_empty() { "Select version" } else { &self.settings.selected_version })
                            .show_ui(ui, |ui| { for v in &self.versions { ui.selectable_value(&mut self.settings.selected_version, v.clone(), v); } });
                    }
                    ui.add_space(22.0);
                    if ui.add_sized([280.0, 56.0], egui::Button::new(egui::RichText::new("▶  OPEN MINECRAFT LAUNCHER").size(17.0))).clicked() {
                        self.status = match open_official_launcher() {
                            Ok(()) => "Opening the official launcher. Choose your version and press Play there.".into(),
                            Err(e) => format!("Could not open launcher: {e}"),
                        };
                    }
                    ui.add_space(12.0);
                    ui.small("Note: Version selection here is informational in v0.1; it does not override the official launcher's selected installation.");
                }
                Tab::Installations => {
                    ui.heading("Local installations");
                    if ui.button("↻ Refresh installed versions").clicked() { self.refresh(); }
                    ui.separator();
                    if self.versions.is_empty() { ui.label("No versions found in the selected game directory."); }
                    for v in &self.versions { ui.horizontal(|ui| { ui.label("▣"); ui.label(v); }); }
                }
                Tab::Settings => {
                    ui.heading("Settings");
                    ui.label("Minecraft game directory");
                    ui.text_edit_singleline(&mut self.settings.game_dir);
                    ui.add_space(12.0);
                    ui.label("Java executable (reserved for direct launch in a future milestone)");
                    ui.text_edit_singleline(&mut self.settings.java_path);
                    ui.add_space(12.0);
                    ui.label("RAM allocation (reserved for future direct launch)");
                    ui.add(egui::Slider::new(&mut self.settings.ram_gb, 1..=32).suffix(" GB"));
                    ui.add_space(16.0);
                    if ui.button("Save settings and refresh").clicked() {
                        self.status = match save_settings(&self.settings) { Ok(()) => "Settings saved.".into(), Err(e) => format!("Save failed: {e}") };
                        self.refresh();
                    }
                }
                Tab::About => {
                    ui.heading("About RustCraft Launcher");
                    ui.label("An independent, open-source Rust desktop launcher project.");
                    ui.label("This initial release detects local Minecraft versions and opens the official launcher.");
                    ui.label("Microsoft login, asset downloads, and direct Java launch are not implemented yet.");
                    ui.label("Not affiliated with Mojang or Microsoft.");
                }
            }
        });
    }
}
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 570.0]).with_min_inner_size([680.0, 440.0]),
        ..Default::default()
    };
    eframe::run_native("RustCraft Launcher", options, Box::new(|_cc| Ok(Box::new(Launcher::new()))))
}
