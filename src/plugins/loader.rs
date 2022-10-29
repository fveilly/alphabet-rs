use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::input::keyboard::KeyCode;
use crate::plugins::AppState;
use crate::plugins::keymap::key_from_str;
use std::collections::HashMap;
use std::path::Path;

use crate::Config;

pub struct LoaderPlugin;

pub struct LoadingAssets {
    sound: HashMap<KeyCode, HandleUntyped>,
}

pub struct AudioAssets {
    pub assets: HashMap<KeyCode, Handle<AudioSource>>
}

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_loading_state.system()));
    }
}

fn start_loading(mut commands: Commands, config: Res<Config>, asset_server: Res<AssetServer>) {
    let mut sound: HashMap<KeyCode, HandleUntyped> = HashMap::new();

    if config.keyboard.is_some() {
        for (k, v) in config.keyboard.as_ref().unwrap() {
            let key = key_from_str(k);
            if key.is_ok() {
                println!("Binding key {:?} to '{:?}'", key, v);
                sound.insert(key.unwrap(), asset_server.load_untyped(Path::new(v)));
            }
        }
    }

    commands.insert_resource(LoadingAssets {
        sound
    });
}

fn check_loading_state(
    mut state: ResMut<State<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loading_assets: Res<LoadingAssets>,
) {
    if LoadState::Loaded != asset_server.get_group_load_state(loading_assets.sound.iter().map(|(_, handle)| handle.id))
    {
        return;
    }

    let mut audio_assets: HashMap<KeyCode, Handle<AudioSource>> = HashMap::new();

    for (key, untyped_asset) in loading_assets.sound.iter() {
        let asset: Handle<AudioSource> = untyped_asset.clone_weak().typed();
        audio_assets.insert(*key, asset);
    }

    commands.insert_resource(AudioAssets {
        assets: audio_assets
    });

    state.set(AppState::Ready).unwrap();
}
