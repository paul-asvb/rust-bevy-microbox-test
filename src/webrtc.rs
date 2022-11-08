use bevy::{prelude::*, tasks::IoTaskPool};
use matchbox_socket::WebRtcSocket;

use crate::GameState;

pub struct WebRtcPlugin;

impl Plugin for WebRtcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::EstablishConnection)
                .with_system(create_socket)
                .with_system(init_text),
        )
        .add_system_set(
            SystemSet::on_update(GameState::EstablishConnection).with_system(accept_new_players),
        );
    }
}

fn init_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("loading...", text_style.clone()).with_alignment(text_alignment),
        transform: Transform::from_translation(Vec3::new(8., 0., 10.)),

        ..default()
    });
}

fn create_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/something_random";
    info!("connecting to matchbox server: {:?}", room_url);
    let (socket, message_loop) = WebRtcSocket::new(room_url);

    IoTaskPool::get().spawn(message_loop).detach();
    commands.insert_resource(Some(socket));
}

fn accept_new_players(
    mut socket: ResMut<Option<WebRtcSocket>>,
    mut app_state: ResMut<State<GameState>>,
) {
    let socket = socket.as_mut();

    if socket.is_none() {
        info!("none");
        return;
    }

    socket.as_mut().unwrap().accept_new_connections();

    if socket.is_some() {
        app_state.set(GameState::Lobby).unwrap();
    }
}
