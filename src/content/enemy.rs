extern crate raylib;

use raylib::prelude::*;
use crate::content::object::Object;

pub struct Enemy {
    pub obj: Object,
}

impl Enemy {
    pub fn new(pos: Vector2, speed: f32, radius: f32, color: Color) -> Enemy {
        let p = Enemy {
            obj: Object {
                pos: pos,
                vel: Vector2 {x: 0.0, y: 0.0},
                speed: speed,
                radius: radius,
                color: color
            },
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
