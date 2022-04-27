use raylib::math::Vector2;
use raylib::prelude::*;

use crate::{OFFSET, WIDTH};

pub struct Object {
    pub pos: Vector2,
    pub vel: Vector2,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
    pub alive: bool,
    pub fixed: bool,
}

impl Object {
    pub fn new(
        pos: Vector2,
        vel: Vector2,
        radius: f32,
        mass: f32,
        color: Color,
        fixed: bool,
    ) -> Object {
        Object {
            pos,
            vel,
            radius,
            mass,
            color,
            alive: true,
            fixed,
        }
    }

    /// Returns whether an object is offscreen (with added offset).
    pub fn is_offscreen(&mut self) -> bool {
        let x = self.pos.x > (WIDTH + OFFSET) as f32 || self.pos.x < (-OFFSET) as f32;
        let y = self.pos.y > (WIDTH + OFFSET) as f32 || self.pos.y < (-OFFSET) as f32;
        x || y
    }

    /// Updates the position.
    pub fn update_pos(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }

    /// Handle the case of two colliding objects with inelastic collision.
    pub fn inelastic_collision(&mut self, v: Vector2, m: f32) {
        if self.fixed {
            return;
        }
        let v_x = (self.mass * self.vel.x + m * v.x) / (self.mass + m);
        let v_y = (self.mass * self.vel.y + m * v.y) / (self.mass + m);

        self.vel.x = v_x;
        self.vel.y = v_y;
    }

    /// Updates the velocity according using Newton's law of universal gravitation.
    pub fn update_vel(&mut self, p: Vector2, m: f32) {
        let dist_x = p.x - self.pos.x;
        let dist_y = p.y - self.pos.y;
        let dist = f32::sqrt(dist_x * dist_x + dist_y * dist_y);
        let inverse_dist = (1 as f32) / dist;

        let normalized_x = inverse_dist * dist_x;
        let normalized_y = inverse_dist * dist_y;

        let inverse_square_dropoff = inverse_dist * inverse_dist;

        let acc_x = normalized_x * m * inverse_square_dropoff;
        let acc_y = normalized_y * m * inverse_square_dropoff;

        self.vel.x += acc_x;
        self.vel.y += acc_y;
    }

    /// Draws the object to the screen.
    pub fn render(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.pos, self.radius, self.color);
        let f: String;
        if self.fixed {
            f = format!("M: {:.0}", self.mass);
        } else {
            f = format!("M: {:.3}\nVx: {:.3}\nVy: {}", self.mass, self.vel.x, self.vel.y);
        }
        let s = f.as_str();
        d.draw_text(s, (self.pos.x+self.radius*0.9) as i32, (self.pos.y+self.radius*0.9) as i32, 14, Color::WHITE);
    }
}
