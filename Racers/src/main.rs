use macroquad::prelude::*;
use car::Car;

pub mod car;
pub mod utils;
pub mod track;

// constants
const WINDOW_WIDTH: i32 = 1200;
const WINDOW_HEIGHT: i32= 800;

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
    loop {
        clear_background(GREEN);
        car1.update();
        car1.draw();

        next_frame().await
    }
}