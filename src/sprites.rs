use raylib::math::Vector2;
use raylib::core::color::Color;

use crate::framebuffers::FrameBuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::texture::TextureManager;
use crate::caster::{cast_ray, Intersect};



pub struct Sprite {
    pub pos: Vector2,
    pub kind: char, 
}

