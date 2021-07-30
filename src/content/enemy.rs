extern crate raylib;
extern crate rand;

use raylib::prelude::*;
use self::rand::Rng;
use crate::content::object::Object;

pub struct Enemy
{
    pub obj: Object,
    pub dir: Vector2
}

impl Enemy 
{
    pub fn new(pos: Vector2, speed: f32, radius: f32, color: Color) -> Enemy 
    {
        let mut rng = rand::thread_rng();

        let mut rand_dir_x: f32 = rng.gen::<f32>() * 2.0 - 1.0;
        let mut rand_dir_y: f32 = rng.gen::<f32>() * 2.0 - 1.0;

        if rand_dir_x < 0.0 
        {
            rand_dir_x = -1.0;
        }
        else 
        {
            rand_dir_x = 1.0;
        }

        if rand_dir_y < 0.0 
        {
            rand_dir_y = -1.0;
        }
        else 
        {
            rand_dir_y = 1.0;
        }

        let p = Enemy 
        {
            obj: Object 
            {
                pos: pos,
                vel: Vector2 {x: 0.0, y: 0.0},
                speed: speed,
                radius: radius,
                color: color
            },
            dir: Vector2 {x: rand_dir_x, y: rand_dir_y},
        };

        return p;
    }

    pub fn update(&mut self) 
    {
        self.obj.pos.x += self.obj.speed * self.dir.x;
        self.obj.pos.y += self.obj.speed * self.dir.y;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) 
    {
        self.obj.draw(d);
    }
}
