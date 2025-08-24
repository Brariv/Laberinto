//TODO
// Minimap
// Minimap show player position ????
// Background Music
// Dots to eat
// If not enough dots not change level
// Enemies with gifs
// walls with texture
// win screen

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread;
use std::time::{Duration, Instant};
use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffers;
mod maze;
mod player;
mod line;
mod caster;
mod texture;

use line::line;
use maze::{Maze,load_maze};
use caster::{cast_ray, Intersect};
use player::{Player, process_events};
use framebuffers::FrameBuffer;
use texture::TextureManager;

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' => {
            return Color::from_hex("#081875").unwrap_or(Color::BLUE);
        },
        '-' => {
            return Color::from_hex("#112fde").unwrap_or(Color::BLUE);
        },
        '|' => {
            return Color::from_hex("#112fde").unwrap_or(Color::BLUE);
        },
        'g' => {
            return Color::GREEN;
        },
        'p' => {
            return Color::YELLOW;
        }
        _ => {
            return Color::BLACK;
        },
        
    }
}

fn draw_cell(
  framebuffer: &mut FrameBuffer,
  xo: usize,
  yo: usize,
  block_size: usize,
  cell: char,
) {
  if cell == ' ' {
    return;
  }
  let color = cell_to_color(cell);

  for x in xo..xo + block_size {
    for y in yo..yo + block_size {
      framebuffer.set_pixel(x as i32, y as i32, color);
    }
  }
}

pub fn render_maze(
  framebuffer: &mut FrameBuffer,
  maze: &Maze,
  block_size: usize,
  player: &Player,
) {
  for (row_index, row) in maze.iter().enumerate() {
    for (col_index, &cell) in row.iter().enumerate() {
      let xo = col_index * block_size;
      let yo = row_index * block_size;
      draw_cell(framebuffer, xo, yo, block_size, cell);
    }
  }

  draw_cell(
    framebuffer,
    (player.pos.x / 4.0) as usize,
    (player.pos.y / 4.0) as usize,
    10,
    'p'
  );
}

fn render_world(
  framebuffer: &mut FrameBuffer,
  texture_manager: &TextureManager,
  maze: &Maze,
  block_size: usize,
  player: &Player,
) {
  let num_rays = framebuffer.image_width;

  // let hw = framebuffer.width as f32 / 2.0;   // precalculated half width
  let hh = framebuffer.image_height as f32 / 2.0;  // precalculated half height


  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

    // Calculate the height of the stake
    let distance_to_wall = intersect.distance;// how far is this wall from the player
    let distance_to_projection_plane = 100.0; // how far is the "player" from the "camera"
    // this ratio doesn't really matter as long as it is a function of distance
    let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

    // Calculate the position to draw the stake
    let stake_top = (hh - (stake_height / 2.0)) as usize;
    let stake_bottom = (hh + (stake_height / 2.0)) as usize;
    
    let ch = intersect.impact;
    let texture = texture_manager.get_texture(ch).unwrap();

    let texture_ref = texture_manager.get_texture(ch).unwrap();
    let tw_u = texture_ref.width();
    let th_u = texture_ref.height();


    // Calculate correct texture X coordinate based on wall orientation
    let wall_x = if (intersect.impact_x.fract() == 0.0) {
        // Vertical wall: use impact_y coordinate
        (intersect.impact_y % block_size as f32) / block_size as f32
    } else {
        // Horizontal wall: use impact_x coordinate
        (intersect.impact_x % block_size as f32) / block_size as f32
    };
    let tex_x = (wall_x * tw_u as f32).clamp(0.0, (tw_u - 1) as f32) as u32;

    let ys = stake_top.max(0);
    let ye = stake_bottom.min(framebuffer.image_height as usize - 1);
    // Draw the stake directly in the framebuffer
    for y in stake_top..stake_bottom {

        let v = (y as f32 - ys as f32) / ((ye as f32 - ys as f32).max(1.0) as f32);
        let tex_y = (v * th_u as f32).clamp(0.0, th_u as f32) as u32;

        
        let color = texture_manager.get_pixel_color(
            intersect.impact,
            tex_x,
            tex_y,
        );

        
        


        framebuffer.set_pixel(i, y as i32, color);
    }
  }
}

fn main() {
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;
    let win = false;
    let framebuffer_color = Color::GRAY;

    let (mut window, mut raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Pacman Maze")
        .log_level(TraceLogLevel::LOG_ALL)
        .build();

    let mut framebuffer = FrameBuffer::new(
        framebuffer_width,
        framebuffer_height,
        framebuffer_color,
        1 // Assuming a pixel size of 4 for RGBA
    );

    let texture_manager = texture::TextureManager::new(
        &mut window,
        &mut raylib_thread
    );

    let maze = load_maze("maze.txt");

    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    let texture = texture_manager.get_texture('#');



    

    

    

    while !window.window_should_close() {
        framebuffer.clear();

        
        process_events(&mut player, &window, &maze, block_size);

        if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }

        render_world(&mut framebuffer, &texture_manager, &maze, block_size, &player);
        render_maze(&mut framebuffer, &maze, 25, &player);


        



        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }


}

