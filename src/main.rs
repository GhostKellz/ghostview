use eframe::egui;
use std::process::Command;
use std::fs;
use std::time::Instant;

#[derive(Default)]
struct PackageListItem {
    name: String,
    version: String,
    desc: String,
    raw: String,
}

fn get_enabled_repos() -> Vec<String> {
    let conf = fs::read_to_string("/etc/pacman.conf").unwrap_or_default();
    let mut repos = Vec::new();
    for line in conf.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let repo = line.trim_start_matches('[').trim_end_matches(']').to_string();
            if repo != "options" { repos.push(repo); }
        }
    }
    repos
}

fn parse_packages(output: &str) -> Vec<PackageListItem> {
    let mut items = Vec::new();
    for line in output.lines() {
        let mut name = String::new();
        let mut version = String::new();
        let mut desc = String::new();
        let raw = line.to_string();
        if let Some((left, d)) = line.split_once(" ") {
            if let Some((_repo_name, rest)) = left.split_once("/") {
                let mut parts = rest.split_whitespace();
                name = parts.next().unwrap_or("").to_string();
                version = parts.next().unwrap_or("").to_string();
                desc = d.trim().to_string();
            } else {
                let mut parts = left.split_whitespace();
                name = parts.next().unwrap_or("").to_string();
                version = parts.next().unwrap_or("").to_string();
                desc = d.trim().to_string();
            }
        }
        if !name.is_empty() {
            items.push(PackageListItem { name, version, desc, raw });
        }
    }
    items
}

fn get_packages(repo: &str, query: &str) -> Vec<PackageListItem> {
    let output = if query.is_empty() {
        Command::new("pacman").args(["-Sl", repo]).output()
    } else {
        Command::new("pacman").args(["-Ss", query]).output()
    };
    match output {
        Ok(out) => parse_packages(&String::from_utf8_lossy(&out.stdout)),
        Err(_) => vec![PackageListItem {
            name: "Failed to run pacman".into(),
            version: "".into(),
            desc: "".into(),
            raw: "".into(),
        }],
    }
}

fn get_package_details(pkg: &str) -> String {
    let output = Command::new("pacman").args(["-Si", pkg]).output();
    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => "Failed to get details".into(),
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "👻 Ghostview",
        options,
        Box::new(|_cc| Box::new(GhostviewApp::default())),
    );
}

struct GhostviewApp {
    search_query: String,
    package_details: String,
    package_list: Vec<PackageListItem>,
    selected_index: Option<usize>,
    repos: Vec<String>,
    selected_repo: usize,
    loading: bool,
    status: String,
    last_load: Instant,
}

impl Default for GhostviewApp {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            package_details: String::new(),
            package_list: Vec::new(),
            selected_index: None,
            repos: Vec::new(),
            selected_repo: 0,
            loading: false,
            status: String::new(),
            last_load: Instant::now(),
        }
    }
}

impl eframe::App for GhostviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let visuals = egui::Visuals {
            dark_mode: true,
            override_text_color: Some(egui::Color32::from_rgb(0x20, 0xff, 0x80)),
            window_fill: egui::Color32::from_rgb(0x0a, 0x1a, 0x2f),
            panel_fill: egui::Color32::from_rgb(0x0a, 0x1a, 0x2f),
            faint_bg_color: egui::Color32::from_rgb(0x13, 0x24, 0x3a),
            extreme_bg_color: egui::Color32::from_rgb(0x07, 0x13, 0x1f),
            ..Default::default()
        };
        ctx.set_visuals(visuals);

        if self.repos.is_empty() {
            self.repos = get_enabled_repos();
        }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            if let Some(idx) = self.selected_index {
                if idx + 1 < self.package_list.len() {
                    self.selected_index = Some(idx + 1);
                    let pkg = &self.package_list[idx + 1];
                    self.package_details = get_package_details(&pkg.name);
                }
            } else if !self.package_list.is_empty() {
                self.selected_index = Some(0);
                let pkg = &self.package_list[0];
                self.package_details = get_package_details(&pkg.name);
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            if let Some(idx) = self.selected_index {
                if idx > 0 {
                    self.selected_index = Some(idx - 1);
                    let pkg = &self.package_list[idx - 1];
                    self.package_details = get_package_details(&pkg.name);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("👻 Ghostview").color(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8)).size(32.0)); // Teal header
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Repository:").color(egui::Color32::from_rgb(0x20, 0xff, 0x80)));
                egui::ComboBox::from_id_source("repo_select")
                    .selected_text(self.repos.get(self.selected_repo).cloned().unwrap_or_default())
                    .show_ui(ui, |cb| {
                        for (i, repo) in self.repos.iter().enumerate() {
                            cb.selectable_value(&mut self.selected_repo, i, repo);
                        }
                    });
                if ui.add(
                    egui::Button::new("🔄 Refresh")
                        .fill(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8))
                        .rounding(egui::Rounding::same(8.0))
                        .min_size([32.0, 32.0].into())
                ).clicked() {
                    self.repos = get_enabled_repos();
                    self.package_list.clear();
                    self.package_details.clear();
                }
                ui.add_space(8.0);
                ui.add(egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search packages...")
                    .desired_width(220.0)
                    .font(egui::TextStyle::Monospace)
                    .text_color(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8))
                );
                if ui.add(
                    egui::Button::new("🔍 Search")
                        .fill(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8))
                        .rounding(egui::Rounding::same(8.0))
                        .min_size([32.0, 32.0].into())
                ).clicked() {
                    self.loading = true;
                    self.last_load = Instant::now();
                    let repo = self.repos.get(self.selected_repo).cloned().unwrap_or_default();
                    self.package_list = get_packages(&repo, &self.search_query);
                    self.selected_index = None;
                    self.package_details.clear();
                    self.loading = false;
                }
            });
            ui.separator();
            ui.columns(2, |cols| {
                if self.loading {
                    let dots = match Instant::now().duration_since(self.last_load).as_secs() % 3 {
                        0 => ".",
                        1 => "..",
                        _ => "...",
                    };
                    cols[0].label(egui::RichText::new(format!("Loading{}", dots)).color(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8)));
                } else {
                    egui::ScrollArea::vertical().show(&mut cols[0], |ui| {
                        for (i, pkg) in self.package_list.iter().enumerate() {
                            let selected = Some(i) == self.selected_index;
                            let label = egui::RichText::new(format!("{} {}", pkg.name, pkg.version))
                                .color(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8))
                                .size(16.0)
                                .strong();
                            let bg = if selected {
                                egui::Color32::from_rgb(0x13, 0x24, 0x3a)
                            } else {
                                egui::Color32::from_rgb(0x0a, 0x1a, 0x2f)
                            };
                            let response = egui::Frame::none()
                                .fill(bg)
                                .rounding(egui::Rounding::same(8.0))
                                .show(ui, |ui| {
                                    ui.add_sized([
                                        ui.available_width(), 38.0
                                    ], egui::SelectableLabel::new(selected, label))
                                    .on_hover_text(&pkg.desc)
                                }).inner;
                            if response.clicked() {
                                if self.selected_index != Some(i) {
                                    self.selected_index = Some(i);
                                    self.package_details = get_package_details(&pkg.name);
                                }
                            }
                        }
                    });
                }
                cols[1].vertical_centered(|ui| {
                    if let Some(idx) = self.selected_index {
                        let pkg = &self.package_list[idx];
                        ui.heading(egui::RichText::new(&pkg.name).color(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8))); // Teal
                        ui.label(egui::RichText::new(&pkg.version).color(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8)));
                        ui.label(egui::RichText::new(&pkg.desc).color(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8)));
                        ui.separator();
                        ui.add_space(8.0);
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(0x13, 0x24, 0x3a))
                            .rounding(egui::Rounding::same(8.0))
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.package_details)
                                        .font(egui::TextStyle::Monospace)
                                        .desired_rows(20)
                                        .desired_width(f32::INFINITY)
                                        .lock_focus(true)
                                        .interactive(false)
                                        .text_color(egui::Color32::from_rgb(0x20, 0xff, 0x80)) // Green text in details
                                );
                                if ui.add(
                                    egui::Button::new("📋 Copy details")
                                        .fill(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8))
                                        .rounding(egui::Rounding::same(8.0))
                                        .min_size([32.0, 32.0].into())
                                ).clicked() {
                                    ui.output_mut(|o| o.copied_text = self.package_details.clone());
                                    self.status = "Copied package details to clipboard!".into();
                                }
                            });
                    } else {
                        ui.label(egui::RichText::new("Select a package to view details.").color(egui::Color32::from_rgb(0x1e, 0xc6, 0xc8)));
                    }
                });
            });
            ui.separator();
            if !self.status.is_empty() {
                ui.label(egui::RichText::new(&self.status).color(egui::Color32::from_rgb(0x20, 0xff, 0x80)));
            }
        });
    }
}