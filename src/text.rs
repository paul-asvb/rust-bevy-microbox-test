use crate::GameState;
use bevy::prelude::*;

pub struct TextPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_text))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(text_input));
    }
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("translation", text_style.clone()).with_alignment(text_alignment),
        ..default()
    });
}

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
) {
    for ev in char_evr.iter() {
        println!("Got char: '{}'", ev.char);
        string.push(ev.char);
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", *string);
        string.clear();
    }
}
