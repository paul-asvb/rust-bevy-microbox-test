use bevy::{prelude::*, tasks::IoTaskPool};
use matchbox_socket::WebRtcSocket;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyEvent {
    value: usize,
}

pub struct WebRtcPlugin;

impl Plugin for WebRtcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label("init")
                .with_system(create_socket)
                .with_system(init_text),
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

fn create_socket() {
    let room_url = "ws://127.0.0.1:3536/something_random";
    info!("connecting to matchbox server: {:?}", room_url);

    let mut events = Events::<MyEvent>::default();
    let mut reader = events.get_reader();

    let background_task = async move {
        let (mut socket, _) = WebRtcSocket::new(room_url);

        let mut peers = Vec::new();

        loop {
            for peer in socket.accept_new_connections() {
                peers.push(peer);
            }

            for event in reader.iter(&events) {
                for peer in &peers {
                    let packet = serde_json::to_vec(&event).unwrap().into_boxed_slice();
                    socket.send(packet, peer);
                }
            }

            for (_peer, packet) in socket.receive() {
                let packet = packet;
                let event: MyEvent = serde_json::from_slice(&packet).unwrap();
                events.send(event);
            }
        }
    };

    IoTaskPool::get().spawn(background_task).detach();
    //commands.insert_resource(Some(socket));
}

// fn accept_new_players(mut socket: ResMut<Option<WebRtcSocket>>) {
//     let socket = socket.as_mut();

//     if socket.is_none() {
//         info!("none");
//         return;
//     }

//     socket.as_mut().unwrap().accept_new_connections();
// }

// fn receive_input(mut socket: ResMut<WebRtcSocket>) {
//     dbg!(socket.connected_peers());
//     // if socket.connected_peers().len() > 0 {
//     //     for (peer_id, payload) in socket.receive() {
//     //         info!("{} {:?}", peer_id, payload);
//     //     }
//     // }
// }

// fn send_input(mut socket: ResMut<WebRtcSocket>, mut char_evr: EventReader<ReceivedCharacter>) {
//     let socket = socket.as_mut();

//     let peers = socket.connected_peers();

//     for ev in char_evr.iter() {
//         info!("try to send to {:#?}", peers);
//         for peer in peers.iter() {
//             socket.send(
//                 ev.char.to_string().as_bytes().to_vec().into_boxed_slice(),
//                 peer,
//             );
//         }
//     }
// }
