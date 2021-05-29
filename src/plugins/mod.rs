use bevy::prelude::*;

mod audio;
mod loader;
mod keymap;
mod text;

pub struct GamePlugin;

use audio::InternalAudioPlugin;
use text::TextPlugin;
use loader::LoaderPlugin;
use std::time::SystemTime;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Loading,
    Ready,
}

pub struct GameContext {
    key_code: Option<KeyCode>,
    last_keypress: SystemTime
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearColor(Color::BLACK))
           .insert_resource(GameContext { key_code: None, last_keypress: SystemTime::now() })
           .add_state(AppState::Loading)
           .add_plugin(LoaderPlugin)
           .add_plugin(InternalAudioPlugin)
           .add_plugin(TextPlugin);
    }
}