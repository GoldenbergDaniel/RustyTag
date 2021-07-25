extern crate raylib;

mod content;

use raylib::prelude::*;
use raylib::consts::ConfigFlag;
use content::object::*;
use content::player::*;
use content::enemy::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 450;
const TITLE: &str = "Game";

fn main() {
    unsafe {
        raylib::ffi::SetConfigFlags(ConfigFlag::FLAG_WINDOW_HIGHDPI as u32);
    }

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title(TITLE)
        .vsync()
        .build();

    let mut player = Player {
        obj: Object {
            pos: Vector2 {x: 0.0, y: 0.0},
            speed: 5.0,
            radius: 30.0,
            color: Color::WHITE,
        },
        lives: 3,
    };

    let mut enemy = Enemy {
        obj: Object {
            pos: Vector2 {x: WIDTH as f32 / 2.0, y: HEIGHT as f32 / 2.0},
            speed: 2.5,
            radius: 20.0,
            color: Color::RED,
        },
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        player.update();
        player.draw(&mut d);

        enemy.update();
        enemy.draw(&mut d);
    }
}
