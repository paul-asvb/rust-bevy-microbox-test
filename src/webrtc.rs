use bevy::{prelude::*, tasks::IoTaskPool};
use matchbox_socket::WebRtcSocket;
use serde::{Deserialize, Serialize};

use crate::GameState;

#[derive(Serialize, Deserialize, Debug)]
pub struct MyEvent {
    pub value: usize,
}

pub struct WebRtcPlugin;

impl Plugin for WebRtcPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MyEvent>()
            //     .add_system_set(SystemSet::on_enter(GameState::Initalizing).with_system(create_socket))
            //     .add_system(end_state);
            .add_startup_system_set_to_stage(
                StartupStage::PostStartup,
                SystemSet::new().with_system(create_socket),
            );
    }
}

//

//fn create_socket(mut ev_writer: EventWriter<MyEvent>, mut ev_reader: EventReader<MyEvent>) {
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

            //events.send(MyEvent { value: 12 });

            //socket.wait_for_peers(1);

            for event in reader.iter(&events) {
                dbg!(event);
                // for peer in &peers {
                //     let packet = serde_json::to_vec(&event).unwrap().into_boxed_slice();
                //     socket.send(packet, peer);
                // }
            }

            if peers.len() > 0 {
                for (_peer, packet) in socket.receive() {
                    let packet = packet;
                    let event: MyEvent = serde_json::from_slice(&packet).unwrap();
                    events.send(event);
                }
            }

            //
        }
    };

    IoTaskPool::get().spawn(background_task).detach();
}

fn end_state(mut app_state: ResMut<State<GameState>>) {
    //app_state.replace(GameState::Lobby)
    dbg!(app_state.current());
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
