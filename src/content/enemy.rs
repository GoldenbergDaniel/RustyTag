extern crate raylib;

use raylib::prelude::*;
use crate::content::object::Object;

pub struct Enemy {
    pub obj: Object,
}

impl Enemy {
    pub fn update(&mut self) {
        self.obj.pos.x += self.obj.speed * 1.60;
        self.obj.pos.y += self.obj.speed * 0.90;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.obj.draw(d);
    }

}