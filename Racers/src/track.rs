use crate::utils::draw_thick_line;
use macroquad::prelude::*;

pub const tarmac_colour: Color = color_u8!(171, 170, 167, 255);
pub const checkpoint_colour: Color = color_u8!(36, 255, 251, 150);

pub const test_track1: [Vec2; 20] = [
    vec2(507.0, 142.0),
    vec2(654.0, 140.0),
    vec2(782.0, 139.0),
    vec2(851.0, 165.0),
    vec2(923.0, 209.0),
    vec2(958.0, 292.0),
    vec2(965.0, 394.0),
    vec2(948.0, 493.0),
    vec2(879.0, 566.0),
    vec2(774.0, 585.0),
    vec2(682.0, 597.0),
    vec2(565.0, 621.0),
    vec2(479.0, 530.0),
    vec2(405.0, 438.0),
    vec2(314.0, 427.0),
    vec2(205.0, 425.0),
    vec2(139.0, 338.0),
    vec2(170.0, 212.0),
    vec2(272.0, 165.0),
    vec2(391.0, 145.0),
];

pub struct Track {
    points_set: [Vec2; 20],
    track_width: f32,
}

impl Track {
    pub fn new(points_set: [Vec2; 20], track_width: f32) -> Self {
        return Self {
            points_set,
            track_width,
        };
    }

    pub fn draw(&self) {
        for i in 0..self.points_set.len() {
            let p1 = self.points_set[i];
            let p2 = self.points_set[(i + 1) % self.points_set.len()];

            let mp = (p1 + p2) / 2.0;

            draw_thick_line(p1.x, p1.y, p2.x, p2.y, self.track_width, tarmac_colour);
            if i == 0 {
                draw_circle(mp.x, mp.y, 8.0, WHITE);
            }
        }
        // draw the checkpoints
        self.draw_checkpoints();
    }

    pub fn get_points(&self) -> &[Vec2; 20] {
        return &self.points_set;
    }

    pub fn get_width(&self) -> f32 {
        return self.track_width;
    }

    pub fn draw_checkpoints(&self) {
        for i in 0..self.points_set.len() {
            let p1 = self.points_set[i];
            let p2 = self.points_set[(i + 1) % self.points_set.len()]; // this is the one we drawing on
            let p3 = self.points_set[(i + 2) % self.points_set.len()];

            let joining_vec1 = p2 - p1;
            let normal1 = vec2(-joining_vec1.y, joining_vec1.x);

            let joining_vec2 = p3 - p2;
            let normal2 = vec2(-joining_vec2.y, joining_vec2.x);

            let avg_normal = (normal1 + normal2) / 2.0;

            let checkpoint_vec = avg_normal.normalize() * self.track_width;
            let start_pos = p2;

            draw_line(
                start_pos.x - checkpoint_vec.x * 0.5,
                start_pos.y - checkpoint_vec.y * 0.5,
                start_pos.x + checkpoint_vec.x * 0.5,
                start_pos.y + checkpoint_vec.y * 0.5,
                4.0,
                checkpoint_colour,
            );
        }
    }

    pub fn get_start_pos(&self) -> Vec2 {
        let pos = self.points_set[0];
        let pos1 = self.points_set[1];

        return (pos + pos1) / 2.0;
    }
}
