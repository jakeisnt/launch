use eframe::egui;
use egui::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

// for desktop entry finding
// Spec: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter, PathSource};
use std::{fs, path::PathBuf, ptr::null};

fn find_desktop_entries<'a>() -> Vec<DesktopEntry<'a>> {
    let mut entries: Vec<DesktopEntry<'a>> = vec![];

    let mut cur_p: Option<PathBuf> = None;
    let mut cur_b: Option<String> = None;

    for path in Iter::new(default_paths()) {
        // let path_src = PathSource::guess_from(&path);
        if let Ok(bytes) = fs::read_to_string(&path) {
            cur_p = Some(path.clone());
            cur_b = Some(bytes.clone());
            if let Ok(entry) = DesktopEntry::decode(&cur_p.unwrap(), &cur_b.unwrap()) {
                // println!("{:?}: {}\n---\n{}", path_src, path.display(), entry);
                entries.push(entry)
            }
        }
    }

    entries
}

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

struct MyApp<'a> {
    query: String,
    options: Vec<DesktopEntry<'a>>,
    matcher: SkimMatcherV2,
    idx: usize,
}

impl<'a> MyApp<'a> {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // TODO: Get options by scanning .desktop files and reading their names
            options: find_desktop_entries(),
            query: "".to_owned(),
            matcher: SkimMatcherV2::default(),
            idx: 0,
        }
    }
}

impl<'a> eframe::App for MyApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let opts: Vec<DesktopEntry<'a>> = self
            .options
            .clone()
            .into_iter()
            .filter(|entry| {
                if let Some(name) = entry.name(None) {
                    return self.matcher.fuzzy_match(&name, &self.query).is_some();
                }
                return false;
            })
            .collect();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.query).request_focus();
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    opts.iter().enumerate().for_each(|(i, entry)| {
                        if let Some(name) = entry.name(None) {
                            if i == self.idx {
                                ui.label(egui::RichText::new(name).underline());
                            } else {
                                ui.heading(name);
                            }
                        }
                    });
                })
        });

        let len: usize = opts.len().try_into().unwrap();

        self.idx = self.idx.min(len - 1);

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

        if ctx.input().key_pressed(Key::Enter) {
            // run the desired program
            panic!("{:?}", self.options[self.idx]);
        }

        _frame.set_fullscreen(true);
    }
}
