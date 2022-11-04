use bevy::{prelude::*, render::camera::ScalingMode, tasks::IoTaskPool};
use matchbox_socket::WebRtcSocket;

pub struct WebRtcPlugin;

impl Plugin for WebRtcPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_matchbox_socket)
            .add_system(wait_for_players);
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/something_random?next=2";
    info!("connecting to matchbox server: {:?}", room_url);
    let (socket, message_loop) = WebRtcSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    IoTaskPool::get().spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

fn wait_for_players(mut socket: ResMut<Option<WebRtcSocket>>) {
    let socket = socket.as_mut();

    // If there is no socket we've already started the game
    if socket.is_none() {
        return;
    }

    // Check for new connections
    let peers = socket.as_mut().unwrap().accept_new_connections();
    //let players = socket.as_ref().unwrap();

    if peers.len() != 0 {
        println!(" {:#?}", peers);
    }
    // TODO
}
