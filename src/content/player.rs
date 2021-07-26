extern crate raylib;

use raylib::prelude::*;
use crate::content::object::*;

pub struct Player {
    pub obj: Object,
    pub lives: u8,
}

impl Player {
    pub fn new(pos: Vector2, speed: f32, radius: f32, color: Color, lives: u8) -> Player {
        let p = Player {
            obj: Object {
                pos: pos,
                vel: Vector2 {x: 0.0, y: 0.0},
                speed: speed,
                radius: radius,
                color: color
            },
            lives: lives
        };

        return p;
    }

    pub fn update(&mut self) {
        self.obj.pos.x += self.obj.speed * 1.60;
        self.obj.pos.y += self.obj.speed * 1.0;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.obj.draw(d);
    }
}
