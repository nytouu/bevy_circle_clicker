use bevy::prelude::*;

mod config;
use config::ConfigPlugin;

// mod editor;
// use editor::PlsEditorPlugin;

mod game;
use game::GamePlugin;

mod ui;
use ui::UiPlugin;

mod utils;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            ConfigPlugin,
            UiPlugin,
            // PlsEditorPlugin,
            GamePlugin,
        ))
        .run();
}
