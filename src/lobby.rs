use crate::GameState;
use bevy::prelude::*;

pub struct LobbyPlugin;

#[derive(Component)]
pub struct TestText;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Lobby).with_system(init_lobby));
        // .add_system_set(
        //     SystemSet::on_update(GameState::Lobby).with_system(receive_input), // .with_system(receive_input),
        // )
    }
}

fn init_lobby(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("TEXT", text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(128., 0., 10.)),

            ..default()
        })
        .insert(TestText);
}
