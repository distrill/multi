use tetra::Context;

use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::math::Vec2;

use serde::{Serialize, Deserialize};

use crate::constants::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chunk {
    x: i32,
    y: i32,
}

impl Chunk {
    pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let position = 
            Vec2::new(self.x as f32 * CHUNK_SIZE, self.y as f32 * CHUNK_SIZE);

        Mesh::rectangle(
            ctx,
            ShapeStyle::Stroke(1.0),
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: CHUNK_SIZE,
                height: CHUNK_SIZE,
            },
        )?.draw(
            ctx,
            DrawParams::new()
                .color(DEBUG_FONT_COLOR)
                .position(position),
        );

        Text::new(
            format!("({}, {})", self.x, self.y),
            Font::vector(ctx, "./resources/DejaVuSansMono.ttf", 12.0)?,
        ).draw(
            ctx,
            DrawParams::new()
                .color(DEBUG_FONT_COLOR)
                .position(position),
        );

        Ok(())
    }

    pub fn new(x: i32, y: i32) -> Chunk {
        Chunk { x, y }
    }
}
