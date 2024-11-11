use macroquad::prelude::*;
use std::f32::consts::PI;

pub fn to_rad(deg: f32) -> f32 {
    return (deg / 180.0) * PI;
}

pub fn lerp(val1: f32, val2: f32, weight: f32) -> f32 {
    // weight between 0-1 
    return val1 + (val2 - val1) * clamp(weight, 0.0, 1.0);
}