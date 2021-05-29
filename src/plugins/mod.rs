use bevy::prelude::*;

mod audio;
mod loader;
mod keymap;
mod text;

pub struct GamePlugin;

use audio::InternalAudioPlugin;
use text::TextPlugin;
use loader::LoaderPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Loading,
    Ready,
}

pub struct KeyboardAssets {
    key_code: Option<KeyCode>
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ClearColor(Color::BLACK))
           .insert_resource(KeyboardAssets { key_code: None })
           .add_state(AppState::Loading)
           .add_plugin(LoaderPlugin)
           .add_plugin(InternalAudioPlugin)
           .add_plugin(TextPlugin);
    }
}