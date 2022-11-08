use bevy::{prelude::*, tasks::IoTaskPool};
use matchbox_socket::WebRtcSocket;

use crate::{lobby::TestText, GameState};

pub struct WebRtcPlugin;

impl Plugin for WebRtcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Lobby).with_system(start_matchbox_socket),
        )
        .add_system_set(SystemSet::on_update(GameState::Lobby).with_system(accept_new_players));
        // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(accept_new_players));
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/something_random";
    info!("connecting to matchbox server: {:?}", room_url);
    let (socket, message_loop) = WebRtcSocket::new(room_url);

    IoTaskPool::get().spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

fn accept_new_players(mut socket: ResMut<Option<WebRtcSocket>>) {
    let socket = socket.as_mut();

    if socket.is_none() {
        return;
    }

    socket.as_mut().unwrap().accept_new_connections();
}
