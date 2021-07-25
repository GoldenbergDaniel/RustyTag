extern crate raylib;

use raylib::prelude::*;

pub struct Object {
    pub pos: Vector2,
    pub speed: f32,
    pub radius: f32,
    pub color: Color,
}

impl Object {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.pos, self.radius, self.color);
    }
}
