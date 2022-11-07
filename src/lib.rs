mod actions;
mod player;
mod text;
mod webrtc;

use crate::actions::ActionsPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
use bevy::prelude::*;
use text::TextPlugin;
use webrtc::WebRtcPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Playing)
            .add_plugin(WebRtcPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(TextPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
