use std::thread;
use std::collections::HashMap;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::cmp;

use anyhow::Result;

use message_io::node;
use message_io::network::{NetEvent, Transport};

use multi::messages::{FromClientMessage, FromServerMessage, Cmd};
use multi::world::WorldState;
use multi::db::Db;
use multi::player::Player;
use multi::constants::*;

fn main() -> Result<()> {
    let (handler, listener) = node::split::<()>();

    let address = "127.0.0.1:12345";
    let transport = Transport::Ws;

    let world = Arc::new(Mutex::new(WorldState::new()));
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let db = Db::new()?;

    match handler.network().listen(transport, address) {
        Ok((_resource_id, real_addr)) => {
            println!("Server running at {} by {}", real_addr, transport);
        }
        Err(_) => {
            panic!("Can not conenct to {} by {}", address, transport);
            
        }
    }

    let _task = {
        let t_world = world.clone();
        let t_clients = clients.clone();
        let t_handler = handler.clone();
        listener.for_each_async(move |event| match event.network() {
            NetEvent::Message(endpoint, input_data) => {
                let message: FromClientMessage =
                    bincode::deserialize(&input_data).unwrap();
                // let t_clients = t_clients.lock().unwrap();
                // let player_id = t_clients.get(&endpoint).unwrap();

                match message {
                    FromClientMessage::Init { username } => {
                        let data = if t_world.lock().unwrap().players.contains_key(&username) {
                            // player is already connected, send a connection error
                            let data = FromServerMessage::ConnectionError(
                                format!(
                                    "User {} is already signed in",
                                    username,
                                ),
                            );
                            println!("User {} is already signed in", username);
                            bincode::serialize(&data).unwrap()
                        } else {
                            // new player, update clients lookup and add them to world players
                            let data = match db.fetch_player(username.clone()) {
                                Ok(None) => {
                                    let player = Player::default(username.clone());
                                    {
                                        t_clients.lock().unwrap().insert(endpoint, username.clone());
                                    }
                                    match db.insert_player(&player) {
                                        Ok(_) => {
                                            t_world.lock().unwrap().players.insert(username, player);
                                            FromServerMessage::ConnectionSuccess
                                        }
                                        Err(err) => FromServerMessage::ConnectionError(
                                            err.to_string(),
                                        )
                                    }
                                }
                                Ok(Some(player)) => {
                                    t_clients.lock().unwrap().insert(endpoint, username.clone());
                                    t_world.lock().unwrap().players.insert(username, player);
                                    FromServerMessage::ConnectionSuccess
                                }
                                Err(err) => {
                                    FromServerMessage::ConnectionError(
                                        err.to_string(),
                                    )
                                }
                            };
                            bincode::serialize(&data).unwrap()
                        };
                        t_handler.clone().network().send(endpoint, &data);
                    }
                    FromClientMessage::Update { cmds } => {
                        let t_clients = t_clients.lock().unwrap();
                        let player_id = t_clients.get(&endpoint).unwrap();
                        let mut t_world = t_world.lock().unwrap();
                        let mut player = t_world.players
                            .get_mut(player_id)
                            .unwrap();
                        for cmd in cmds.iter() {
                            match cmd {
                                Cmd::Up => {
                                    player.pos.y = cmp::max(
                                        0,
                                        player.pos.y - 4,
                                    );
                                }
                                Cmd::Down => {
                                    player.pos.y = cmp::min(
                                        player.pos.y + 4,
                                        MAP_HEIGHT - 16,
                                    );
                                }
                                Cmd::Left => {
                                    player.pos.x = cmp::max(
                                        0,
                                        player.pos.x - 4,
                                    );
                                }
                                Cmd::Right => {
                                    player.pos.x = cmp::min(
                                        player.pos.x + 4,
                                        MAP_WIDTH - 16,
                                    );
                                }
                            }
                        }
                    }
                }     
            }
            NetEvent::Connected(endpoint, _) => {
                let mut t_clients = t_clients.lock().unwrap();
                t_clients.insert(endpoint, "".to_string());
                println!(
                    "Client ({}) connected (total clients: {})",
                    endpoint.addr(),
                    t_clients.len(),
                );
            }
            NetEvent::Disconnected(endpoint) => {
                let mut t_clients = t_clients.lock().unwrap();
                if let Some(player_id) = t_clients.get(&endpoint) {
                    let mut t_world = t_world.lock().unwrap();
                    t_world.players.remove(player_id);
                }
                t_clients.remove(&endpoint);
                println!(
                    "Client ({}) disconnected (total clients: {})",
                    endpoint.addr(),
                    t_clients.len(),
                );
            }
        })
    };

    loop {
        // ~ 60 times per second
        thread::sleep(Duration::from_millis(15));

        let world = world.lock().unwrap().clone();
        let clients = clients.lock().unwrap();
        let message = FromServerMessage::Tick {
            world,
        };
        let data = bincode::serialize(&message).unwrap();
        for (endpoint, _) in clients.iter() {
            handler.network().send(*endpoint, &data);
        }
    }
}
