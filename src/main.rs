use eframe::egui;
use egui::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

// for desktop entry finding
// Spec: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter, PathSource};
use std::{fs, path::PathBuf, ptr::null};

/// Find all of the desktop entries available on the current system.
fn find_desktop_entries<'a>(entries: &'a mut Vec<DesktopEntry<'a>>) -> () {}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("launch", options, Box::new(|cc| Box::new(MyApp::new(cc))))
}

// let de = DesktopEntry::decode(path.as_path(), &input).expect("Error decoding desktop entry");
// de.launch(&[]).expect("Failed to run desktop entry");

// this launcher looks awesome: https://github.com/Biont/sway-launcher-desktop

// rlaunch looks here/:
// scanning ("/home/jake/.local/share/applications", "")
// scanning ("/etc/profiles/per-user/jake/share/applications", "")
// scanning ("/run/current-system/sw/share/applications", "")
// Finished reading all 29 applications (0.001078074s)

// https://github.com/pop-os/freedesktop-desktop-entry/pull/5/files

struct MyApp {
    query: String,
    options: Vec<Entry>,
    matcher: SkimMatcherV2,
    idx: usize,
}

#[derive(Debug, Clone)]
struct Entry {
    name: String,
    // exec: String,
    path: PathBuf,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut entries: Vec<Entry> = vec![];

        for path in Iter::new(default_paths()) {
            if let Ok(bytes) = fs::read_to_string(&path) {
                if let Ok(entry) = DesktopEntry::decode(&path.clone(), &bytes.clone()) {
                    match (entry.exec(), entry.name(None)) {
                        (Some(exec), Some(name)) => {
                            entries.push(Entry {
                                path: path.to_owned(),
                                // exec: exec.to_string(),
                                name: name.as_ref().to_string(),
                            });
                        }
                        (_, _) => {}
                    }
                }
            }
        }

        Self {
            // TODO: Get options by scanning .desktop files and reading their names
            options: entries,
            query: "".to_owned(),
            matcher: SkimMatcherV2::default(),
            idx: 0,
        }
    }
}

// Exec a program entry
fn exec_entry(e: &Entry) {
    let path = &e.path;
    let input = fs::read_to_string(e.path.clone()).expect("Failed to read file");
    let de = DesktopEntry::decode(path.as_path(), &input).expect("Error decoding desktop entry");
    de.launch(&[]).expect("Failed to run desktop entry");
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let opts: Vec<Entry> = self
            .options
            .clone()
            .into_iter()
            .filter(|entry| self.matcher.fuzzy_match(&entry.name, &self.query).is_some())
            .collect();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.query).request_focus();
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    opts.iter().enumerate().for_each(|(i, entry)| {
                        if i == self.idx {
                            ui.label(egui::RichText::new(&entry.name).underline());
                        } else {
                            ui.heading(&entry.name);
                        }
                    });
                })
        });

        let len: usize = opts.len().try_into().unwrap();

        self.idx = self.idx.min(len.saturating_sub(1));

        if ctx.input().key_pressed(Key::ArrowDown) {
            // TODO: wrapping trait?
            if self.idx == len - 1 {
                self.idx = 0;
            } else {
                self.idx += 1;
            }
        }

        if ctx.input().key_pressed(Key::ArrowUp) {
            if self.idx == 0 {
                self.idx = len - 1;
            } else {
                self.idx -= 1;
            }
        }

        if ctx.input().key_pressed(Key::Escape) {
            std::process::exit(1);
        }

        if ctx.input().key_pressed(Key::Enter) {
            if opts.len() > self.idx {
                exec_entry(&opts[self.idx]);
            }
        }
    }
}
