use std::sync::{Arc, Mutex};

use tetra::graphics::{self, Color};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

use message_io::network::{Transport, NetEvent, Endpoint};
use message_io::node::{self, NodeEvent, NodeTask, NodeHandler};

use multi::world::WorldState;
use multi::messages::{Cmd, FromClientMessage};
use multi::constants::*;

struct GameState {
    task: NodeTask,
    handler: NodeHandler<()>,
    world: Arc<Mutex<WorldState>>,
    server_id: Endpoint,
}

impl GameState {
    fn new(_: &mut Context) -> tetra::Result<GameState> {
        let (handler, listener) = node::split::<()>();

        let address = "127.0.0.1:12345";
        let transport = Transport::Ws;

        let server_id = match handler.network().connect(transport, address) {
            Ok((server_id, local_addr)) => {
                println!("Connected to server by {} at {}", transport, server_id.addr());
                println!("Client identified by local port: {}", local_addr.port());
                server_id
            }
            Err(_) => {
                panic!("Can not connect to the server by {} to {}", transport, address);
            }
        };

        let world = Arc::new(Mutex::new(WorldState::new()));

        let task = {
            let handler = handler.clone();
            let t_world = world.clone();
            listener.for_each_async(move |event| match event {
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Message(_, input_data) => {
                        let server_world: WorldState = bincode::deserialize(&input_data).unwrap();
                        let mut t_world = t_world.lock().unwrap();
                        t_world.players = server_world.players;
                        t_world.map = server_world.map;
                    }
                    NetEvent::Connected(_, _) => unreachable!(),
                    NetEvent::Disconnected(_) => {
                        println!("Server is disconnected");
                        handler.stop();
                    }
                },
                NodeEvent::Signal(signal) => println!("signal: {:?}", signal),
            })
        };


        Ok(GameState { task, handler, world, server_id })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.45, 0.50, 0.55));

        {
            let mut world = self.world.lock().unwrap();
            world.map.draw(ctx)?;
            for (_, p) in world.players.iter_mut() {
                p.draw(ctx)?;
            }
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let mut cmds = vec![];
        if input::is_key_down(ctx, Key::W) {
            cmds.push(Cmd::Up);
        }

        if input::is_key_down(ctx, Key::S) {
            cmds.push(Cmd::Down);
        }

        if input::is_key_down(ctx, Key::A) {
            cmds.push(Cmd::Left);
        }

        if input::is_key_down(ctx, Key::D) {
            cmds.push(Cmd::Right);
        }

        if cmds.len() > 0 {
            let data = FromClientMessage { cmds };
            let data = bincode::serialize(&data).unwrap();
            self.handler.clone().network().send(
                self.server_id,
                &data,
            );
        }

        Ok(())
    }
}

impl Drop for GameState {
    fn drop(&mut self) {
        self.handler.stop();
        drop(&self.task);
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("multi-client", MAP_WIDTH, MAP_HEIGHT)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
