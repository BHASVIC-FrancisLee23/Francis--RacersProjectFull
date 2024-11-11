use std::f32::consts::PI;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use crate::utils::{to_rad, lerp};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

// consts
const FRIC_COEF_ROAD: f32 = 0.88;
const FRIC_COEF_GRASS: f32 = 0.08;

pub struct Car {
    // Physics variables
    // -- Vectors
    velocity: Vec2,
    direction: Vec2,
    position: Vec2,
    acceleration: Vec2,

    // -- Scalar
    angle: f32,
    steer: f32,

    // Graphics
    texture: Texture2D,
    rect: Rect,

    // inputs for controllers
    accelerator_input: Input,
    steering_input: Input, // radians
    brakes_input: Input,
}

#[derive(Default)]
pub struct Input {
    min: f32,
    weight: f32,
    max: f32,
    default: f32,
}


impl Car {
    pub const HITBOX_WIDTH: f32 = 30.0;
    pub const HITBOX_HEIGHT: f32 = 60.0;
    pub const MAX_RPM: f32 = 10000.0;
    pub const MAX_SPEED: f32 = 350.0;
    pub const MAX_TURNING_ANGLE: f32 = (40.0 / 180.0 ) * PI;
    pub const MAX_ACC: f32 = 400.0;
    pub const STEER_WEIGHT: f32 = PI/6.0;
    pub const MASS: f32 = 40.0;
    pub const BRAKING_FACTOR: f32 = 0.9;


    pub fn new() -> Self {
        // default car setup
        let mut car: Self = Self {
            texture: Texture2D::from_file_with_format(
                include_bytes!("../assets/car.png"),
                None,
            ),
            
            // Defining Vector
            position: Vec2::new(crate::WINDOW_WIDTH as f32 / 2.0, crate::WINDOW_HEIGHT as f32 / 2.0),
            velocity: Vec2::ZERO,
            direction: Vec2::ZERO,
            acceleration: Vec2::ZERO,

            // Scalar
            angle: 0.0,
            steer: 0.0,

            // other
            rect: Rect::new(0.0, 0.0, Car::HITBOX_WIDTH, Car::HITBOX_HEIGHT),

            // inputs
            accelerator_input: Input::new_default(),
            brakes_input: Input::new_default(),
            steering_input: Input {min: -1.0, max: 1.0, weight: 0.0, default: 0.0},    
        };
        car.direction = Vec2::from_angle(car.angle);
        return car;
    }

    pub fn draw(&self) {
        // just draws to the screen
        let w: f32 = self.rect.w;
        let h: f32 = self.rect.h;
        let x: f32 = self.rect.x;
        let y: f32 = self.rect.y;
        let params: DrawTextureParams = DrawTextureParams{
            dest_size: Some(Vec2::new(w, h)),
            source:  None,
            flip_x: false,
            flip_y: false,
            rotation: self.angle + PI/2.0,
            pivot: None,
        };
        draw_texture_ex(&self.texture, x, y, WHITE, params);
    }

    pub fn update_pos(&mut self, x: f32, y: f32) { 

        // way to safely change position
        let x = clamp(x, 0.0, WINDOW_WIDTH as f32 - Car::HITBOX_WIDTH); // keep the car on the screen
        let y = clamp(y, 0.0, WINDOW_HEIGHT as f32 - Car::HITBOX_HEIGHT);

        self.position = Vec2::new(x, y);
        self.rect.x = x;
        self.rect.y = y;
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        self.keyboard_control();

        self.steer = self.steering_input.weight * Car::STEER_WEIGHT;
        let new_angle = self.angle + self.steer;

        self.direction = Vec2::from_angle(new_angle);
        self.angle = lerp(self.angle, new_angle, dt * 6.0);

        self.acceleration = self.direction * (self.accelerator_input.weight * Car::MAX_ACC);
        self.velocity += self.acceleration * dt;
       
        let brake_friction = -self.velocity * self.brakes_input.weight * Car::BRAKING_FACTOR;
        self.velocity += brake_friction * dt;

        let normal_fric: Vec2 = -self.velocity * FRIC_COEF_ROAD;

        // apply frictions
        self.velocity += (normal_fric) * dt;
        self.position += self.velocity * dt;
        self.update_pos(self.position.x, self.position.y);

        // reset inputs
        self.brakes_input.weight = 0.0;
        self.accelerator_input.weight = 0.0;
        self.steering_input.weight = 0.0;
    }


    fn keyboard_control(&mut self) {
        // loop through keys 
        for key in get_keys_down() {
            if (key == KeyCode::Up) {
                self.accelerator_input.weight = 1.0;
            }
            if (key == KeyCode::Down) {
                self.brakes_input.weight = 1.0;
            }
            if (key == KeyCode::Left) {
                self.steering_input.weight = -1.0;
            }
            if (key == KeyCode::Right) {
                self.steering_input.weight = 1.0;
            }
        }
        // clamping speeds and steering
        if (self.velocity.length() > Car::MAX_SPEED) {
            self.velocity = ((self.velocity) / self.velocity.length()) * Car::MAX_SPEED;
        }
    }
}



impl Input {
    pub fn new_default() -> Self {
        Self {
            min: 0.0,
            weight: 0.0,
            max: 1.0,
            default: 0.0,
        }
    }
}