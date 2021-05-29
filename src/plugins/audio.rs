use bevy::{
    input::keyboard::{KeyboardInput},
    prelude::*,
};

use crate::plugins::AppState;
use crate::plugins::loader::AudioAssets;
use crate::plugins::KeyboardAssets;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin  {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Ready).with_system(start_audio.system()))
            .add_system_set(
                SystemSet::on_update(AppState::Ready)
                    .with_system(keyboard_input_system.system())
            )
            .add_system_set(SystemSet::on_exit(AppState::Ready).with_system(stop_audio.system()));
    }
}

fn start_audio(_audio_assets: Res<AudioAssets>, _audio: Res<Audio>) {
}

fn stop_audio(_audio: Res<Audio>) {
}

fn keyboard_input_system(audio_assets: Res<AudioAssets>, mut keyboard_asset: ResMut <KeyboardAssets>, audio: Res<Audio>, mut keyboard_input_events: EventReader<KeyboardInput>) {
    use bevy::input::ElementState;

    for event in keyboard_input_events.iter() {
        match event.state {
            ElementState::Pressed => {
                println!("Key press: {:?} ({})", event.key_code, event.scan_code);

                if let Some(key_code) = event.key_code {
                    let asset = audio_assets.assets.get(&key_code);
                    if asset.is_some() {
                        audio.play(asset.unwrap().clone());
                    }
                }

                keyboard_asset.key_code = event.key_code;
            }
            ElementState::Released => {
                println!("Key release: {:?} ({})", event.key_code, event.scan_code);
            }
        }
    }
}