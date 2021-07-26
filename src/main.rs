extern crate raylib;

mod content;

use raylib::prelude::*;
use raylib::consts::ConfigFlag;
use content::player::*;
use content::enemy::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 500;
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

    rl.set_target_fps(60);

    let mut player: Player = Player::new(Vector2 {x: 0.0, y: 0.0}, 5.0, 30.0, Color::WHITE, 3);
    let mut enemy: Enemy = Enemy::new(Vector2 {x: 400.0, y: 250.0}, 2.5, 20.0, Color::RED);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        player.update();
        enemy.update();

        player.draw(&mut d);
        enemy.draw(&mut d);
    }
}
