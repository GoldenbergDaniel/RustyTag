extern crate raylib;

use raylib::prelude::*;

pub struct Player {
    pub pos: Vector2,
    pub radius: f32,
    pub color: Color,
}

impl Player {
    pub fn update(&mut self) {
        self.pos.x += 10.0;
        self.pos.y += 10.0;
    }
}

impl Player {
    pub fn draw(&mut self, mut d: RaylibDrawHandle) {
        d.draw_circle_v(self.pos, self.radius, self.color);
    }
}
