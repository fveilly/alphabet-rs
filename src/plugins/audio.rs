use bevy::{
    input::keyboard::{KeyboardInput},
    prelude::*,
};

use crate::plugins::AppState;
use crate::plugins::loader::AudioAssets;
use crate::plugins::GameContext;

use std::time::SystemTime;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin  {
    fn build(&self, app: &mut App) {
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

fn keyboard_input_system(audio_assets: Res<AudioAssets>, mut game_ctx: ResMut<GameContext>, audio: Res<Audio>, mut keyboard_input_events: EventReader<KeyboardInput>) {
    use bevy::input::ElementState;
    let sys_time = SystemTime::now();

    for event in keyboard_input_events.iter() {
        match event.state {
            ElementState::Pressed => {
                println!("Key press: {:?} ({})", event.key_code, event.scan_code);

                if let Ok(duration) = sys_time.duration_since(game_ctx.last_keypress) {
                    const KEYPRESS_LIMIT_MS: u128 = 150;
                    if duration.as_millis() < KEYPRESS_LIMIT_MS {
                        println!("too soon, waiting for {}ms...", KEYPRESS_LIMIT_MS - duration.as_millis());
                        continue;
                    }
                }

                if let Some(key_code) = event.key_code {
                    let asset = audio_assets.assets.get(&key_code);
                    if asset.is_some() {
                        audio.play(asset.unwrap().clone());
                    }
                }

                game_ctx.key_code = event.key_code;
                game_ctx.last_keypress = SystemTime::now();
            }
            ElementState::Released => {
                println!("Key release: {:?} ({})", event.key_code, event.scan_code);
            }
        }
    }
}
