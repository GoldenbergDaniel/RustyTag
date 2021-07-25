extern crate raylib;

mod player;

use raylib::prelude::*;
use raylib::consts::ConfigFlag;
use player::Player;

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
        pos: Vector2 {x: 0.0, y: 0.0},
        radius: 30.0,
        color: Color::BLUE,
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        player.update();
        player.draw(d);
    }
}
