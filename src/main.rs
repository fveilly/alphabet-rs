extern crate bevy;
extern crate serde;

use bevy::prelude::*;

use std::collections::HashMap;
use std::fs;

mod plugins;

use plugins::GamePlugin;

#[derive(Debug, serde::Deserialize)]
struct Config {
    keyboard: Option<HashMap<String, String>>,
}

fn main() {
    let config_data = fs::read_to_string("keymap.toml").unwrap();
    let config: Config = toml::from_str(&config_data).unwrap();

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Alphabet".to_string(),
            width: 640.,
            height: 480.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .insert_resource(config)
        .run();
}
