extern crate raylib;

use raylib::prelude::*;
use raylib::consts::KeyboardKey;
use crate::content::object::*;

pub struct Player 
{
    pub obj: Object,
    pub dir: Vector2,
    pub lives: u8,
}

impl Player 
{
    pub fn new(pos: Vector2, speed: f32, radius: f32, color: Color, lives: u8) -> Player 
    {
        let p = Player 
        {
            obj: Object 
            {
                pos: pos,
                vel: Vector2 {x: 0.0, y: 0.0},
                speed: speed,
                radius: radius,
                color: color
            },
            dir: Vector2 {x: 0.0, y: 0.0},
            lives: lives
        };

        return p;
    }

    pub fn update(&mut self, rl: &RaylibHandle) 
    {
        self.dir = Vector2 {x: 0.0, y:0.0};

        if rl.is_key_down(KeyboardKey::KEY_W) 
        {
            self.dir.y = -1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_S) 
        {
            self.dir.y = 1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_W) && rl.is_key_down(KeyboardKey::KEY_S) 
        {
            self.dir.y = 0.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_A) 
        {
            self.dir.x = -1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_D) 
        {
            self.dir.x = 1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_A) && rl.is_key_down(KeyboardKey::KEY_D) {
            self.dir.x = 0.0;
        }

        self.obj.vel.x = self.obj.speed * self.dir.x;
        self.obj.vel.y = self.obj.speed * self.dir.y;

        self.obj.pos.x += self.obj.vel.x;
        self.obj.pos.y += self.obj.vel.y;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) 
    {
        self.obj.draw(d);
    }
}
