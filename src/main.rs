extern crate raylib;

mod entity;

use raylib::prelude::*;
use raylib::consts::ConfigFlag;
use entity::player::Player;
use entity::enemy::Enemy;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 450;
const TITLE: &str = "Game";

fn main() 
{
    unsafe 
    {
        ffi::SetConfigFlags(ConfigFlag::FLAG_WINDOW_HIGHDPI as u32);
    }

    let (mut rl, thread) = init()
        .size(WIDTH, HEIGHT)
        .title(TITLE)
        .vsync()
        .build();

    let mut player: Player = Player::new(Vector2 {x: 0.0, y: 0.0}, 5.0, 20.0, Color::WHITE, 3);
    let mut enemy: Enemy = Enemy::new(Vector2 {x: 400.0, y: 250.0}, 2.5, 20.0, Color::RED);

    while !rl.window_should_close() 
    {
        player.update(&mut rl);
        enemy.update();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
 
        player.draw(&mut d);
        enemy.draw(&mut d);
    }
}
