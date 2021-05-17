use tetra::{Context, State, ContextBuilder};
use tetra::graphics;

mod chunk;
mod constants;
mod map;
mod player;
mod temp;

// use chunk::{Chunk};
use map::Map;
use constants::*;


struct GameState {
    map: Map,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let map = Map::new(ctx)?;
        Ok(GameState { map })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, GLOBAL_BACKGROUND_COLOR);
        
        self.map.draw(ctx)?;

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new(
        "bc",
        MAP_CHUNK_WIDTH as i32 * CHUNK_SIZE as i32,
        MAP_CHUNK_HEIGHT as i32 * CHUNK_SIZE as i32,
    )
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}
