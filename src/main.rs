use eframe::egui;
use egui::*;
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::{fs, path::PathBuf};

/// Find all of the desktop entries available on the current system.
fn find_desktop_entries() -> Vec<Entry> {
    let mut entries: Vec<Entry> = vec![];

    for path in Iter::new(default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path.clone(), &bytes.clone()) {
                match (entry.exec(), entry.name(None)) {
                    (_, Some(name)) => {
                        entries.push(Entry {
                            path: path.to_owned(),
                            name: name.as_ref().to_string(),
                        });
                    }
                    (_, _) => {}
                }
            }
        }
    }

    entries
}

fn main() -> Result<(), eframe::Error> {
    // let options = eframe::NativeOptions {
    //     // decorated: false,
    //     // NOTE: These two should open a centered pop-up. They don't!
    //     // always_on_top: true,
    //     // centered: true,
    //     // transparent: true,
    //     ..Default::default()
    // };
    //
    let options = eframe::NativeOptions::default();
    eframe::run_native("launch", options, Box::new(|cc| Box::new(Launch::new(cc))))
}

#[derive(Debug, Clone)]
struct Entry {
    name: String,
    path: PathBuf,
}

impl Entry {
    // Exec a program entry
    fn exec(&self) {
        let path = &self.path;
        let input = fs::read_to_string(path.clone()).expect("Failed to read file");
        let de =
            DesktopEntry::decode(path.as_path(), &input).expect("Error decoding desktop entry");
        de.launch(&[]).expect("Failed to run desktop entry");
        std::process::exit(1);
    }
}

struct Launch {
    query: String,
    options: Vec<Entry>,
    matcher: SkimMatcherV2,
    idx: usize,
}

impl Launch {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            options: find_desktop_entries(),
            query: "".to_owned(),
            matcher: SkimMatcherV2::default(),
            idx: 0,
        }
    }
}

trait Wrap {
    fn wrap_add(self, rhs: Self) -> Self;
    fn wrap_sub(self, rhs: Self) -> Self;
}

impl Wrap for usize {
    fn wrap_add(self, rhs: Self) -> Self {
        if self == rhs - 1 {
            0
        } else {
            self.saturating_add(1)
        }
    }

    fn wrap_sub(self, rhs: Self) -> Self {
        if self == 0 {
            rhs - 1
        } else {
            self.saturating_sub(1)
        }
    }
}

impl eframe::App for Launch {
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
                            ui.label(RichText::new(&entry.name).strong());
                        } else {
                            ui.label(RichText::new(&entry.name));
                        }
                    });
                });
        });

        let len: usize = opts.len().try_into().unwrap();
        self.idx = self.idx.min(len.saturating_sub(1));

        if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
            self.idx = self.idx.wrap_add(len);
        }

        if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
            self.idx = self.idx.wrap_sub(len);
        }

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            std::process::exit(1);
        }

        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            if opts.len() > self.idx {
                opts[self.idx].exec();
            }
        }
    }
}
