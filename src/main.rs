use eframe::egui;
use std::process::Command;
use std::fs;

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

fn get_packages(repo: &str, query: &str) -> Vec<String> {
    let output = if query.is_empty() {
        Command::new("pacman").args(["-Sl", repo]).output()
    } else {
        Command::new("pacman").args(["-Ss", query]).output()
    };
    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect(),
        Err(_) => vec!["Failed to run pacman".into()],
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

#[derive(Default)]
struct GhostviewApp {
    search_query: String,
    package_details: String,
    package_list: Vec<String>,
    selected_index: Option<usize>,
    repos: Vec<String>,
    selected_repo: usize,
    loading: bool,
}

impl eframe::App for GhostviewApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.repos.is_empty() {
            self.repos = get_enabled_repos();
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("👻 Ghostview");
            ui.horizontal(|ui| {
                ui.label("Repository:");
                egui::ComboBox::from_id_source("repo_select")
                    .selected_text(self.repos.get(self.selected_repo).cloned().unwrap_or_default())
                    .show_ui(ui, |cb| {
                        for (i, repo) in self.repos.iter().enumerate() {
                            cb.selectable_value(&mut self.selected_repo, i, repo);
                        }
                    });
                if ui.button("🔄 Refresh").clicked() {
                    self.repos = get_enabled_repos();
                    self.package_list.clear();
                    self.package_details.clear();
                }
                ui.add(egui::TextEdit::singleline(&mut self.search_query).hint_text("Search packages..."));
                if ui.button("🔍 Search").clicked() {
                    self.loading = true;
                    let repo = self.repos.get(self.selected_repo).cloned().unwrap_or_default();
                    self.package_list = get_packages(&repo, &self.search_query);
                    self.selected_index = None;
                    self.package_details.clear();
                    self.loading = false;
                }
            });
            ui.separator();
            ui.columns(2, |cols| {
                // Left: Package list
                if self.loading {
                    cols[0].label("Loading...");
                } else {
                    egui::ScrollArea::vertical().show(&mut cols[0], |ui| {
                        for (i, pkg) in self.package_list.iter().enumerate() {
                            let selected = Some(i) == self.selected_index;
                            if ui.selectable_label(selected, pkg).clicked() {
                                self.selected_index = Some(i);
                                let pkg_name = pkg.split_whitespace().nth(1).unwrap_or(pkg);
                                self.package_details = get_package_details(pkg_name);
                            }
                        }
                    });
                }
                // Right: Package details
                cols[1].vertical_centered(|ui| {
                    if let Some(idx) = self.selected_index {
                        ui.heading("Package Details");
                        ui.separator();
                        ui.label(&self.package_details);
                    } else {
                        ui.label("Select a package to view details.");
                    }
                });
            });
        });
    }
}