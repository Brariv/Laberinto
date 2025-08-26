

use raylib::prelude::*;
use raylib::consts::GamepadAxis;
use std::f32::consts::PI;

use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32, 
}

pub fn process_events(player: &mut Player, rl: &mut RaylibHandle, maze: &mut Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 25.0;
    let gamepad = 0; // Gamepad 1 is usually index 0 in raylib


    let mut is_walkable = |x: f32, y: f32| {
        let col = (x / block_size as f32) as usize;
        let row = (y / block_size as f32) as usize;
        if maze[row][col] == ' ' {
            return true;
        } else if maze[row][col] == 'c' {
            maze[row][col] = ' ';
            return true; 
        } else {
            return false;
        }
    };

    let mouse_delta = rl.get_mouse_delta();
    rl.set_mouse_position((650.0, 450.0));
    rl.hide_cursor();
    player.a += -mouse_delta.x * 0.003;

    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a -= ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();
        if is_walkable(new_x, new_y) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();
        if is_walkable(new_x, new_y) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }

    if rl.is_key_down(KeyboardKey::KEY_D) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.sin();
        let new_y = player.pos.y + MOVE_SPEED * player.a.cos();
        if is_walkable(new_x, new_y) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }

    if rl.is_key_down(KeyboardKey::KEY_A) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.sin();
        let new_y = player.pos.y - MOVE_SPEED * player.a.cos();
        if is_walkable(new_x, new_y) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }

    let gamepad_index = 0;
    if rl.is_gamepad_available(gamepad_index) {
        let deadzone = 0.2;

        let left_x = rl.get_gamepad_axis_movement(gamepad_index, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
        let left_y = rl.get_gamepad_axis_movement(gamepad_index, GamepadAxis::GAMEPAD_AXIS_LEFT_Y);
        let right_x = rl.get_gamepad_axis_movement(gamepad_index, GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
        
    
        let move_x = if left_x.abs() > deadzone { left_x } else { 0.0 };
        let move_y = if left_y.abs() > deadzone { -left_y } else { 0.0 }; 

        let new_x = player.pos.x + MOVE_SPEED * move_y * player.a.cos() - MOVE_SPEED * move_x * player.a.sin();
        let new_y = player.pos.y + MOVE_SPEED * move_y * player.a.sin() + MOVE_SPEED * move_x * player.a.cos();
        if is_walkable(new_x, new_y) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }

        let rotate_x = if right_x.abs() > deadzone { right_x } else { 0.0 };
        player.a += -rotate_x * ROTATION_SPEED;  }
    if rl.is_gamepad_available(gamepad_index) {
        println!(
            "Gamepad {} detected: {}",
            gamepad_index,
            rl.get_gamepad_name(gamepad_index).unwrap_or("Unknown".to_string())
        );
    }



}

