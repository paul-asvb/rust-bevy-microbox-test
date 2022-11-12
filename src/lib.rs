mod actions;
mod lobby;
mod player;
mod webrtc;

use crate::actions::ActionsPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
use bevy::prelude::*;
use lobby::LobbyPlugin;
use webrtc::WebRtcPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Initalizing,
    Lobby,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Lobby)
            .add_startup_system_set_to_stage(
                StartupStage::PreStartup,
                SystemSet::new().with_system(init_camera),
            )
            .add_plugin(WebRtcPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(LobbyPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
