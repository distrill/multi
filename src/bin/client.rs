use std::sync::{Arc, Mutex};
use std::{io, panic, process};

use tetra::graphics::{self, Color};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, State};

use message_io::network::{Transport, NetEvent, Endpoint};
use message_io::node::{self, NodeEvent, NodeTask, NodeHandler};

use multi::world::WorldState;
use multi::messages::{Cmd, FromClientMessage, FromServerMessage};
use multi::constants::*;

use rustop::opts;

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

        let username = sign_in();

        let server_id = match handler.network().connect(transport, address) {
            Ok((server_id, local_addr)) => {
                println!("Connected to server by {} at {}", transport, server_id.addr());
                println!("Client identified by local port: {}", local_addr.port());
                
                // start signin verification process
                let data = FromClientMessage::Init { username: username.clone() };
                let data = bincode::serialize(&data).unwrap();
                handler.clone().network().send(server_id, &data);

                server_id
            }
            Err(_) => {
                panic!("Can not connect to the server by {} to {}", transport, address);
            }
        };

        let world = Arc::new(Mutex::new(WorldState::new()));


        let task = {
            let t_handler = handler.clone();
            let t_world = world.clone();
            listener.for_each_async(move |event| match event {
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Message(_, input_data) => {
                        let message : FromServerMessage = bincode::deserialize(&input_data).unwrap();
                        match message {
                            FromServerMessage::Tick { world: server_world } => {
                                let mut t_world = t_world.lock().unwrap();
                                t_world.players = server_world.players;
                                t_world.map = server_world.map;
                            }
                            FromServerMessage::ConnectionSuccess => {
                                println!(
                                    "Succesfully connected as {}",
                                    username.clone(),
                                );
                            }
                            FromServerMessage::ConnectionError(msg) => {
                                panic!(
                                    "
                                    Error connecting to server: {}
                                    Please try again
                                    ",
                                    msg,
                                );
                            }
                        }
                    }
                    NetEvent::Connected(_, _) => unreachable!(),
                    NetEvent::Disconnected(_) => {
                        println!("Server is disconnected");
                        t_handler.stop();
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
            let data = FromClientMessage::Update { cmds };
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

fn sign_in() -> String {
    let (args, _) = opts! {
        synopsis "Start the multi client and connect to a server";
        param username :Option<String>, desc:"username to sign in with";
    }.parse_or_exit();

    if let Some(un) = args.username {
        un
    } else {
        println!("Enter a username");
        let mut un = String::new();
        io::stdin()
            .read_line(&mut un)
            .expect("Failed to read line");
        un
    }.trim().to_string()
}

fn main() -> tetra::Result {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    ContextBuilder::new("multi-client", MAP_WIDTH, MAP_HEIGHT)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
