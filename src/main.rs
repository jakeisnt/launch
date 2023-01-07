use eframe::egui;
use egui::*;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native("launch", options, Box::new(|cc| Box::new(MyApp::new(cc))))
}

// this launcher looks awesome: https://github.com/Biont/sway-launcher-desktop

// rlaunch looks here/:
// scanning ("/home/jake/.local/share/applications", "")
// scanning ("/etc/profiles/per-user/jake/share/applications", "")
// scanning ("/run/current-system/sw/share/applications", "")
// Finished reading all 29 applications (0.001078074s)

// https://github.com/pop-os/freedesktop-desktop-entry/pull/5/files
//   # https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html

struct MyApp {
    query: String,
    options: Vec<String>,
    matcher: SkimMatcherV2,
    idx: usize,
}

// NOTE:
// https://github.com/fiveawe/desktopentries/tree/b991b8c70f967a6537c16314188377ff51821bf5/
// has the desktop entries specification.

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // TODO: Get options by scanning .desktop files and reading their names
            options: vec!["Firefox".into(), "Amazon".into(), "Google Chrome".into()],
            query: "".to_owned(),
            matcher: SkimMatcherV2::default(),
            idx: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // iterator that owns its references ->
        // this was likely the issue i was having before

        let opts: Vec<String> = self
            .options
            .to_owned()
            .into_iter()
            .filter(|opt| self.matcher.fuzzy_match(opt, &self.query).is_some())
            .collect();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.query).request_focus();
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    opts.iter().enumerate().for_each(|(i, x)| {
                        if i == self.idx {
                            ui.label(egui::RichText::new(x).underline());
                        } else {
                            ui.heading(x);
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
