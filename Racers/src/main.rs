use car::Car;
use macroquad::prelude::*;
use track::{test_track1, Track};

pub mod car;
pub mod track;
pub mod utils;

// constants
const WINDOW_WIDTH: i32 = 1200;
const WINDOW_HEIGHT: i32 = 800;

// config
fn window_conf() -> Conf {
    Conf {
        window_title: "Racers".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut car1: Car = Car::new();
    let mut track: Track = Track::new(test_track1, 75.0);
    loop {
        clear_background(GREEN);
        car1.update();

        track.draw();
        car1.draw();

        next_frame().await
    }
}
