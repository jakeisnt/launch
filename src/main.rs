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

struct MyApp {
    query: String,
    options: Vec<String>,
    matcher: SkimMatcherV2,
    idx: usize,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            options: vec!["Firefox".into(), "Amazon".into(), "Google Chrome".into()],
            query: "".to_owned(),
            matcher: SkimMatcherV2::default(),
            idx: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.query).request_focus();
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    self.options
                        .iter()
                        .filter(|opt| self.matcher.fuzzy_match(opt, &self.query).is_some())
                        .enumerate()
                        .for_each(|(i, x)| {
                            if i == self.idx {
                                ui.label(egui::RichText::new(x).underline());
                            } else {
                                ui.heading(x);
                            }
                        });
                })
        });

        if ctx.input().key_pressed(Key::ArrowDown) {
            self.idx += 1;
        }

        if ctx.input().key_pressed(Key::ArrowUp) {
            self.idx -= 1;
        }

        if ctx.input().key_pressed(Key::Enter) {
            panic!("{:?}", self.options[self.idx]);
        }

        // idx starts at launching app 0.
        // if the arrow keys go down, we select the next one
        // if the arrow keys go up, we wrap to the previous
        // if enter is pressed, run the current idx item
        // otherwise, enter into the text box

        _frame.set_fullscreen(true);
    }
}
