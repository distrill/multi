use std::thread;
use std::collections::HashMap;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use message_io::node;
use message_io::network::{NetEvent, Transport};

use multi::messages::{FromClientMessage};
use multi::world::WorldState;
use multi::player::Player;

fn main() {
    let (handler, listener) = node::split::<()>();

    let address = "127.0.0.1:12345";
    let transport = Transport::Ws;

    let world = Arc::new(Mutex::new(WorldState::new()));
    let clients = Arc::new(Mutex::new(HashMap::new()));

    match handler.network().listen(transport, address) {
        Ok((_resource_id, real_addr)) => {
            println!("Server running at {} by {}", real_addr, transport);
        }
        Err(_) => {
            panic!("Can not listen at {} by {}", address, transport);
            
        }
    }

    let _task = {
        let t_world = world.clone();
        let t_clients = clients.clone();
        listener.for_each_async(move |event| match event.network() {
            NetEvent::Message(endpoint, input_data) => {
                let message: FromClientMessage = bincode::deserialize(&input_data).unwrap();
                let t_clients = t_clients.lock().unwrap();
                let player_id = t_clients.get(&endpoint).unwrap();
                for cmd in message.cmds.iter() {
                    // process command here - update state of the world
                    println!("cmd received from {}: {:?}", player_id, cmd);
                }
            }
            NetEvent::Connected(endpoint, _) => {
                let player = Player::new();
                let player_id = player.id.clone();
                {
                    t_world.lock().unwrap().players.insert(player.id.clone(), player);
                }
                {
                    let mut t_clients = t_clients.lock().unwrap();
                    t_clients.insert(endpoint, player_id);
                    println!("Client ({}) connected (total clients: {})", endpoint.addr(), t_clients.len());
                }
            }
            NetEvent::Disconnected(endpoint) => {
                let mut t_clients = t_clients.lock().unwrap();
                if let Some(player_id) = t_clients.get(&endpoint) {
                    let mut t_world = t_world.lock().unwrap();
                    t_world.players.remove(player_id);
                }
                t_clients.remove(&endpoint);
                println!("Client ({}) disconnected (total clients: {})", endpoint.addr(), t_clients.len());
            }
        })
    };

    loop {
        // ~ 60 times per second
        thread::sleep(Duration::from_millis(15));

        let world = world.lock().unwrap();
        let clients = clients.lock().unwrap();
        let data = bincode::serialize(&*world).unwrap();
        for (endpoint, _) in clients.iter() {
            handler.network().send(*endpoint, &data);
        }
    }
}
