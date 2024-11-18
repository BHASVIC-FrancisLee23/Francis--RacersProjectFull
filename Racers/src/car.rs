use crate::network::*;
use crate::utils::{find_line_eq, lerp, line_intersection, to_rad};
use core::f32;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use std::f32::consts::PI;
use std::sync::Arc;

use crate::track::Track;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

// consts
const FRIC_COEF_ROAD: f32 = 0.88;
const FRIC_COEF_GRASS: f32 = 0.08;

pub struct Car {
    // Physics variables
    // -- Vectors
    velocity: Vec2,
    pub direction: Vec2,
    position: Vec2,
    acceleration: Vec2,
    // -- Scalar
    angle: f32,
    steer: f32,

    // Graphics
    texture: Texture2D,
    rect: Rect,

    // network
    brain: Network,

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
    pub const MAX_SPEED: f32 = 350.0;
    pub const MAX_TURNING_ANGLE: f32 = (40.0 / 180.0) * PI;
    pub const MAX_ACC: f32 = 400.0;
    pub const STEER_WEIGHT: f32 = PI / 6.0;
    pub const MASS: f32 = 40.0;
    pub const BRAKING_FACTOR: f32 = 0.9;
    const RAYS: usize = 12;

    pub fn new(start_pos: Vec2) -> Self {
        // default car setup
        let mut brain = Network::new_empty();
        brain = brain
            .add_layer(Layer::new_random(6 + Car::RAYS, 8, None))
            .add_layer(Layer::new_random(8, 5, None))
            .add_layer(Layer::new_random(5, 3, Some(sigmoid)));

        let mut car: Self = Self {
            texture: Texture2D::from_file_with_format(include_bytes!("../assets/car.png"), None),

            // Defining Vector
            position: vec2(
                start_pos.x - Self::HITBOX_WIDTH / 2.0,
                start_pos.y - Self::HITBOX_HEIGHT / 2.0,
            ),
            velocity: Vec2::ZERO,
            direction: Vec2::ZERO,
            acceleration: Vec2::ZERO,

            // Scalar
            angle: 0.0,
            steer: 0.0,

            // other
            rect: Rect::new(0.0, 0.0, Car::HITBOX_WIDTH, Car::HITBOX_HEIGHT),

            // brain
            brain,

            // inputs
            accelerator_input: Input::new_default(),
            brakes_input: Input::new_default(),
            steering_input: Input {
                min: -1.0,
                max: 1.0,
                weight: 0.0,
                default: 0.0,
            },
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
        let params: DrawTextureParams = DrawTextureParams {
            dest_size: Some(Vec2::new(w, h)),
            source: None,
            flip_x: false,
            flip_y: false,
            rotation: self.angle + PI / 2.0,
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

    pub fn update(&mut self, track: &Track) {
        let dt = get_frame_time();

        // run the neural network with inputs
        let rays = self.cast_rays(Car::RAYS, 200.0, track);
        let velx_norm = self.velocity.x / Car::MAX_SPEED;
        let vely_norm = self.velocity.y / Car::MAX_SPEED;
        let accx_norm = self.acceleration.x / Car::MAX_ACC;
        let accy_norm = self.acceleration.y / Car::MAX_ACC;
        let steer_norm = self.steer / Car::STEER_WEIGHT;
        let angle_norm = (self.angle).sin();

        let mut inputs: Vec<f64> = vec![];
        for ray in rays.iter() {
            inputs.push((*ray) as f64);
        }
        inputs.push(velx_norm as f64);
        inputs.push(vely_norm as f64);
        inputs.push(accx_norm as f64);
        inputs.push(accy_norm as f64);
        inputs.push(steer_norm as f64);
        inputs.push(angle_norm as f64);

        // run the network
        let outputs = self.brain.run(inputs);
        self.accelerator_input.weight = outputs[0] as f32;
        self.steering_input.weight = ((outputs[1] - 0.5) * 2.0) as f32; // convert to value between -1.0 and 1.0
        self.brakes_input.weight = outputs[2] as f32;

        self.steer = self.steering_input.weight * Car::STEER_WEIGHT;
        let new_angle = self.angle + self.steer;

        self.angle = lerp(self.angle, new_angle, dt * 6.0);
        self.direction = Vec2::from_angle(self.angle);

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

    pub fn get_sector(&self, track: &Track) -> i32 {
        let mut closest_sector = 0;
        let mut shortest_distance: f32 = f32::MAX;

        for i in 0..track.get_points().len() {
            // calculate the midpoint
            let p1 = track.get_points()[i];
            let p2 = track.get_points()[(i + 1) % track.get_points().len()];
            let mp = (p1 + p2) / 2.0;

            let center = self.rect.center();
            let distance = mp.distance(center);
            if distance < shortest_distance {
                shortest_distance = distance;
                closest_sector = i as i32;
            }
        }
        return closest_sector;
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

    pub fn is_on_track(&self, track: &Track) -> bool {
        // find the current sector
        let sector: usize = self.get_sector(track) as usize;

        // find the sector line equation
        let points = track.get_points();
        let p1 = points[sector];
        let p2 = points[(sector + 1) % points.len()];
        let coef_vec = find_line_eq(p1.x, p1.y, p2.x, p2.y);
        let a = coef_vec.x;
        let b = 1.0;
        let c = coef_vec.y;

        // use the formula for distance
        let center = self.rect.center();
        let distance = (a * center.x + b * center.y + c).abs() / (a * a + b * b).sqrt();

        if distance > (track.get_width() / 2.0) {
            // off the track
            return false;
        }

        return true;
    }

    pub fn cast_ray(&self, track: &Track, ray_direction: Vec2) -> f32 {
        // returns distance to line sector

        let track_width = track.get_width();
        let current_sector: usize = self.get_sector(track) as usize;
        let points = track.get_points();

        let s1 = self.rect.center();
        let s2 = s1 + ray_direction * WINDOW_WIDTH as f32 * 5.0;

        let mut shortest_interection_point: Vec2 = Vec2::MAX;
        let mut shortest_distance: f32 = WINDOW_WIDTH as f32;

        for i in 0..points.len() {
            // finding points A, B, C, D
            let sector_index = (current_sector + i) % points.len();
            let sector = points[sector_index];
            let next_sector = points[(sector_index + 1) % points.len()];
            let prev_sector = points[(sector_index + (points.len() - 1)) % points.len()];
            let next_next_sector = points[(sector_index + 2) % points.len()];

            // finding normal for A and C
            let direction1_left = sector - prev_sector;
            let direction2_left = next_sector - sector;
            let normal1_left = vec2(-direction1_left.y, direction1_left.x);
            let normal2_left = vec2(-direction2_left.y, direction2_left.x);
            let avg_normal_left = ((normal1_left + normal2_left) / 2.0).normalize();

            // finding normal for B and D
            let direction1_right = next_sector - sector;
            let direction2_right = next_next_sector - next_sector;
            let normal1_right = vec2(-direction1_right.y, direction1_right.x);
            let normal2_right = vec2(-direction2_right.y, direction2_right.x);
            let avg_normal_right = ((normal1_right + normal2_right) / 2.0).normalize();

            // calculating points
            let A = sector + avg_normal_left * (track_width / 2.0);
            let B = next_sector + avg_normal_right * (track_width / 2.0);
            let C = sector - avg_normal_left * (track_width / 2.0);
            let D = next_sector - avg_normal_right * (track_width / 2.0);

            if let Some(point1) = line_intersection(s1, s2, A, B) {
                let distance = (point1 - s1).length();
                if distance < shortest_distance {
                    shortest_interection_point = point1;
                    shortest_distance = distance;
                }
            } else if let Some(point2) = line_intersection(s1, s2, C, D) {
                let distance = (point2 - s1).length();
                if distance < shortest_distance {
                    shortest_interection_point = point2;
                    shortest_distance = distance;
                }
            }
        }

        draw_line(
            s1.x,
            s1.y,
            shortest_interection_point.x,
            shortest_interection_point.y,
            3.0,
            RED,
        );
        draw_circle(
            shortest_interection_point.x,
            shortest_interection_point.y,
            4.0,
            RED,
        );

        shortest_distance
    }

    pub fn cast_rays(&self, rays: usize, fov: f32, track: &Track) -> Vec<f32> {
        // fov in degrees

        let mut ray_list: Vec<f32> = vec![];

        let start_angle = self.angle.to_degrees() - fov / 2.0;
        let step = fov / rays as f32;

        for ray in 0..rays {
            let angle = start_angle + step * ray as f32;
            let dir = Vec2::from_angle(angle.to_radians());
            let distance = self.cast_ray(track, dir);
            // normalize the distance against the window width
            let normalized = distance / (WINDOW_WIDTH as f32);
            ray_list.push(normalized);
        }

        return ray_list;
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
