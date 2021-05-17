use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

use message_io::network::{Transport, NetEvent};
use message_io::node::{self, NodeEvent, NodeTask, NodeHandler};

use multi::world::WorldState;

struct GameState {
    task: NodeTask,
    handler: NodeHandler<()>,
}

impl GameState {
    fn new(_: &mut Context) -> tetra::Result<GameState> {
        let (handler, listener) = node::split::<()>();

        let address = "127.0.0.1:12345";
        let transport = Transport::Ws;

        let _server_id = match handler.network().connect(transport, address) {
            Ok((server_id, local_addr)) => {
                println!("Connected to server by {} at {}", transport, server_id.addr());
                println!("Client identified by local port: {}", local_addr.port());
                server_id
            }
            Err(_) => {
                panic!("Can not connect to the server by {} to {}", transport, address);
            }
        };

        let task = {
            let handler = handler.clone();
            listener.for_each_async(move |event| match event {
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Message(_, input_data) => {
                        let world : WorldState = bincode::deserialize(&input_data).unwrap();
                        let np = world.players.len();
                        println!("players connected: {}", np);
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

        Ok(GameState { task, handler })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.45, 0.50, 0.55));
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
    ContextBuilder::new("multi-client", 640, 480)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
