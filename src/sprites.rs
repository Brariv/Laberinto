//const TRANSPARENT_COLOR: Color = Color::new(152, 0, 136, 255);


use raylib::math::Vector2;
use raylib::core::color::Color;

use crate::framebuffers::FrameBuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::texture::TextureManager;
// If 'caster' is a local module in your project, use:
use crate::caster::{cast_ray, Intersect};
// Or, if 'caster' is an external crate, add it to Cargo.toml and use:
// use caster::{cast_ray, Intersect};



pub struct Sprite {
    pub pos: Vector2,
    pub kind: char, // 'e' (enemigo), 'c' (coin), etc.
}

