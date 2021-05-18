use tetra::graphics::Color;

pub const CHUNK_SIZE: f32 = 64.0;
pub const GLOBAL_BACKGROUND_COLOR: Color = Color::rgb(0.4, 0.45, 0.5);
pub const DEBUG_FONT_COLOR: Color = Color::rgb(0.5, 0.55, 0.6);
pub const MAP_CHUNK_WIDTH: usize = 4;
pub const MAP_CHUNK_HEIGHT: usize = 3;
pub const MAP_WIDTH: i32 = CHUNK_SIZE as i32 * MAP_CHUNK_WIDTH as i32;
pub const MAP_HEIGHT: i32 = CHUNK_SIZE as i32 * MAP_CHUNK_HEIGHT as i32;
