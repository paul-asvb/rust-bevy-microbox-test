use crate::GameState;
use bevy::prelude::*;
use matchbox_socket::WebRtcSocket;

pub struct LobbyPlugin;

#[derive(Component)]
pub struct TestText;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Lobby).with_system(init_lobby))
            .add_system_set(SystemSet::on_update(GameState::Lobby).with_system(send_input))
            .add_system_set(SystemSet::on_update(GameState::Lobby).with_system(receive_input));
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
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("TEXT", text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(128., 0., 10.)),

            ..default()
        })
        .insert(TestText);
}

fn receive_input(mut socket: ResMut<WebRtcSocket>) {
    for (peer_id, payload) in socket.receive() {
        info!("{} {:?}", peer_id, payload);
    }
}

fn send_input(
    mut socket: ResMut<Option<WebRtcSocket>>,
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
) {
    for ev in char_evr.iter() {
        let peers = socket.unwrap().connected_peers();
        for peer in peers.iter() {
            let c = ev.char.to_string().as_bytes();
            socket
                .unwrap()
                .send(Box::new(*c), peer);
        }
        // Whatever you want to send
    }
}
