use bevy::prelude::*;

pub struct TextPlugin;

use crate::plugins::KeyboardAssets;
use crate::plugins::keymap::str_from_key;

impl Plugin for TextPlugin  {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
        .add_system(text_update_system.system());
    }
}

struct AlphabetText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // FIXME: should not need this
                margin: Rect {
                    left: Val::Px(250.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 320.0,
                    color: Color::GOLD,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                },
            ),
            ..Default::default()
        })
        .insert(AlphabetText);
}

fn text_update_system(keyboard_asset: Res <KeyboardAssets>, mut query: Query<&mut Text, With<AlphabetText>>) {
    for mut text in query.iter_mut() {
        if let Some(key_code) = keyboard_asset.key_code {
            if let Ok(key_str) = str_from_key(key_code) {
                text.sections[0].value = format!("{}", key_str);
            }
        }
    }
}