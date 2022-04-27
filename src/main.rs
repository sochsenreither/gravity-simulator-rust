mod particle;

use image;
use rand::Rng;
use raylib::prelude::MouseButton::*;
use raylib::prelude::*;
use std::path::Path;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 900;
const OFFSET: i32 = 1500;
const STARTMASS: f32 = 5000.0;

fn main() {
    if !Path::new("bg.png").exists() {
        draw();
    }

    // raylib setup
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("gravity simulator")
        .build();
    rl.set_target_fps(60);
    let t = rl
        .load_texture(&thread, "bg.png")
        .expect("could not load texture from image");

    let mut start = Vector2::new(0.0, 0.0);
    let mut objects: Vec<particle::Object> = Vec::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&t, 0, 0, Color::WHITE);

        let mouse_pos = d.get_mouse_position();

        // Place new planet on left button click. Drag to add velocity
        if d.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
            start = mouse_pos;
        }
        if d.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
            let end = mouse_pos;
            let v_x = (end.x - start.x) / ((WIDTH >>2 ) as f32);
            let v_y = (end.y - start.y) / ((HEIGHT >>2) as f32);
            let f = format!("vX: {:.3}\nvY: {:.3}", v_x, v_y);
            let s = f.as_str();
            d.draw_text(s, mouse_pos.x as i32, mouse_pos.y as i32, 17, Color::WHITE)
        }
        if d.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
            let end = mouse_pos;
            let v = Vector2::new(
                (end.x - start.x) / ((WIDTH >> 2) as f32),
                (end.y - start.y) / ((HEIGHT >> 2) as f32),
            );
            objects.push(particle::Object::new(
                start,
                v,
                5.0,
                1.0,
                Color::WHITE,
                false,
            ));
        }

        // Place new star on right button click. Drag to add velocity
        if d.is_mouse_button_pressed(MOUSE_RIGHT_BUTTON) {
            start = mouse_pos;
        }
        if d.is_mouse_button_down(MOUSE_RIGHT_BUTTON) {
            let end = mouse_pos;
            let dist_x = end.x - start.x;
            let dist_y = end.y - start.y;
            let dist = f32::sqrt(dist_x * dist_x + dist_y * dist_y);
            let m = (STARTMASS * dist) as i32 >> 9;
            let f = format!("Mass: {:.0}", m);
            let s = f.as_str();
            d.draw_text(s, mouse_pos.x as i32, mouse_pos.y as i32, 17, Color::WHITE)
        }
        if d.is_mouse_button_released(MOUSE_RIGHT_BUTTON) {
            let end = mouse_pos;
            let dist_x = end.x - start.x;
            let dist_y = end.y - start.y;
            let dist = f32::sqrt(dist_x * dist_x + dist_y * dist_y);
            let m = (STARTMASS * dist) as i32 >> 9;
            let v = Vector2::new(0.0, 0.0);
            objects.push(particle::Object::new(
                start,
                v,
                50.0,
                m as f32,
                Color::YELLOW,
                true,
            ));
        }

        // Remove objects with middle mouse button click.
        if d.is_mouse_button_released(MOUSE_MIDDLE_BUTTON) {
            for i in 0..objects.len() {
                if raylib::check_collision_circles(
                    mouse_pos,
                    15.0,
                    objects[i].pos,
                    objects[i].radius,
                ) {
                    objects[i].alive = false;
                }
            }
        }

        // Update objects
        for i in 0..objects.len() {
            if !objects[i].alive {
                // TODO: delete element
                continue;
            }
            if objects[i].is_offscreen() {
                objects[i].alive = false
            }
            if !objects[i].fixed {
                for j in 0..objects.len() {
                    if i != j && objects[j].alive {
                        let other_pos = Vector2::new(objects[j].pos.x, objects[j].pos.y);
                        let other_m = objects[j].mass;
                        let other_v = Vector2::new(objects[j].vel.x, objects[j].vel.y);
                        objects[i].update_vel(other_pos, other_m);
                        if raylib::check_collision_circles(
                            objects[i].pos,
                            objects[i].radius,
                            objects[j].pos,
                            objects[j].radius,
                        ) {
                            if objects[i].mass > objects[j].mass {
                                objects[i].mass += objects[j].mass;
                                objects[j].alive = false;
                                objects[i].inelastic_collision(other_v, other_m);
                            } else {
                                objects[j].mass += objects[i].mass;
                                objects[i].alive = false;
                                let this_v = Vector2::new(objects[i].vel.x, objects[i].vel.y);
                                let this_m = objects[i].mass;
                                objects[j].inelastic_collision(this_v, this_m);
                            }
                        }
                    }
                    objects[i].update_pos();
                }
            }
            objects[i].render(&mut d);
        }
    }
}

fn draw() {
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        let num = rand::thread_rng().gen_range(0..65);
        match num {
            0 => *pixel = image::Rgb([80 as u8, 80 as u8, 80 as u8]),
            1 => *pixel = image::Rgb([130 as u8, 130 as u8, 130 as u8]),
            _ => *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]),
        }
    }
    imgbuf.save("bg.png").unwrap();
}
