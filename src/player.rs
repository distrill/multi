use nanoid::nanoid;

use serde::{Serialize, Deserialize};

use tetra::{Context};
use tetra::graphics::{Texture, DrawParams};
use tetra::math::{Vec2};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub pos: Pos,
}

impl Player {
    pub fn new() -> Player {
        Player {
            id: nanoid!(),
            pos: Pos { x: 0, y: 0 },
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let texture = Texture::new(ctx, "./resources/player.png")?;

        let position = Vec2::new(self.pos.x as f32, self.pos.y as f32);
        texture.draw(ctx, DrawParams::new().position(position));

        Ok(())
    }
}
