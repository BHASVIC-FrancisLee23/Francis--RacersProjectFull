use macroquad::prelude::*;
use std::f32::consts::PI;

pub fn to_rad(deg: f32) -> f32 {
    return (deg / 180.0) * PI;
}

pub fn lerp(val1: f32, val2: f32, weight: f32) -> f32 {
    // weight between 0-1
    return val1 + (val2 - val1) * clamp(weight, 0.0, 1.0);
}

pub fn draw_thick_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, colour: Color) {
    // draw a set of circles along the line
    // the number of circles determines how smooth the line is so i chose 20
    let steps = 20;
    let vec_x = x2 - x1;
    let vec_y = y2 - y1;
    for step in 0..steps {
        let fstep = step as f32;
        draw_circle(
            x1 + fstep * (vec_x / (steps as f32)),
            y1 + fstep * (vec_y / (steps as f32)),
            thickness / 2.0,
            colour,
        );
    }
}

pub fn find_line_eq(x1: f32, y1: f32, x2: f32, y2: f32) -> Vec2 {
    // trying to complete form:
    // ax + by + c = 0
    let m = (y2 - y1) / (x2 - x1);
    let d = y1 - m * x1;
    let a = -m;
    let c = -d;

    // the Vec2 will return (a, c) in this form
    return vec2(a, c);
}
