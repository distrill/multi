use rand::{self, Rng};

use serde::{Serialize, Deserialize};

use tetra::{Context};
use tetra::graphics::{Texture, DrawParams, Color};
use tetra::math::{Vec2};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSettings {
    pub color: (f32, f32, f32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub pos: Pos,
    pub settings: PlayerSettings,
}

impl Player {
    pub fn new(id: String, pos: (i32, i32), color: (f32, f32, f32)) -> Player {
        let settings = PlayerSettings { color };
        Player {
            id,
            pos: Pos { x: pos.0, y: pos.1 },
            settings,
        }
    }

    pub fn default(id: String) -> Player {
        let possible_colors = vec!["red", "green", "blue", "black", "white"];
        let color = possible_colors[
            rand::thread_rng().gen_range(0..possible_colors.len())
        ].to_string();

        let color = match color.as_str() {
            "red" => (1.0, 0.0, 0.0),
            "green" => (0.0, 1.0, 0.0),
            "blue" => (0.0, 0.0, 1.0),
            "black" => (0.0, 0.0, 0.0),
            "white" => (1.0, 1.0, 1.0),
            _ => panic!("Invalid color. Possible values are (red, green, blue, black, white)"),
        };

        let pos = (0, 0);

        Player::new(id, pos, color)
    }

    pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let texture = Texture::new(ctx, "./resources/player.png")?;

        let position = Vec2::new(self.pos.x as f32, self.pos.y as f32);
        texture.draw(
            ctx,
            DrawParams::new()
                .position(position)
                .color(Color::rgb(1.0, 0.0, 0.0))
        );

        Ok(())
    }
}
