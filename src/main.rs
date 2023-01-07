// Copyright 2021 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of various text layout features.
//!
//! I would like to make this a bit fancier (like the flex demo) but for now
//! lets keep it simple.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use std::sync::Arc;

// NOTE: Bad sign 1: have to use a custom implementation of a vector.
use druid::im::{vector, Vector};
use druid::widget::{Flex, Label, List, Scroll, TextBox};
use druid::{
    AppLauncher, Color, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc, WindowId,
};

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Text Options");

#[derive(Clone, Data, Lens)]
struct AppState {
    query: Arc<String>,
    // items to search through
    items: Vector<String>,
    // cursor index
    idx: u32,
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 600.0))
        // TODO: How do we force the window to float?
        .set_window_state(druid::WindowState::Restored);

    let items: Vector<String> = vector!["the", "quick", "brown", "fox"]
        .iter()
        .map(|s| (*s).into())
        .collect();
    // .map(|st| st.to_string())
    // .to_vec()
    // .into();

    // create the initial app state
    let initial_state = AppState {
        query: "".to_string().into(),
        idx: 0,
        items: items.into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

// I should probably not use Druid... it's too experimental, and I want to avoid immediate mode.

fn build_root_widget() -> impl Widget<AppState> {
    let reflect: Label<Arc<String>> = Label::dynamic(|data, _| format!("{}", data));

    let child = TextBox::multiline()
        .with_placeholder("Search...")
        .lens(AppState::query)
        .expand_width();

    let mut lists = Flex::row().cross_axis_alignment(druid::widget::CrossAxisAlignment::Start);

    // Build a simple list
    lists.add_flex_child(
        Scroll::new(List::new(|| {
            Label::new(|msg: &&str, _env: &_| format!("List item {:?}", msg))
        }))
        .vertical()
        // AppState::items
        .lens(druid::lens::Field::new(
            |v: AppState| {
                v.items
                    .iter()
                    .filter(|elem| (*elem).eq(&*v.query))
                    .collect()
            },
            |v: AppState| {
                v.items
                    .iter()
                    .filter(|elem| (*elem).eq(&*v.query))
                    .collect()
            },
        )),
        1.0,
    );

    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_flex_child(child, 1.0)
        .with_flex_child(reflect.lens(AppState::query), 1.0)
        .with_flex_child(lists, 1.0)
        .padding(8.0)
}
