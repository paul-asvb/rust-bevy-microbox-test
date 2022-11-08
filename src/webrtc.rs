use bevy::{prelude::*, tasks::IoTaskPool};
use matchbox_socket::WebRtcSocket;

use crate::text::TestText;

pub struct WebRtcPlugin;

impl Plugin for WebRtcPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_matchbox_socket)
            .add_system(wait_for_players)
            .add_system(send_input);
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/something_random";
    info!("connecting to matchbox server: {:?}", room_url);
    let (socket, message_loop) = WebRtcSocket::new(room_url);

    IoTaskPool::get().spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

fn wait_for_players(mut socket: ResMut<Option<WebRtcSocket>>) {
    let socket = socket.as_mut();

    if socket.is_none() {
        return;
    }

    let peers = socket.as_mut().unwrap().accept_new_connections();

    if peers.len() != 0 {
        info!(" {:#?}", peers);
    }
}

fn send_input(
    mut socket: ResMut<Option<WebRtcSocket>>,
    mut query: Query<&mut Text, With<TestText>>,
) {
    let socket = socket.as_mut();

    if socket.is_none() {
        return;
    }

    let peers = socket.as_mut().unwrap().accept_new_connections();
    
    if peers.len() != 0 {
        info!(" {:#?}", peers);
    }

    if !query.is_empty() {
        let mut text = query.single_mut();
        text.sections[0].value = peers.len().to_string();
    }
}
