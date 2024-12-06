use car::Car;
use macroquad::prelude::*;
use population::Population;

pub mod car;
pub mod network;
pub mod population;
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
    macroquad::rand::srand(macroquad::miniquad::date::now() as _);

    let mut population = Population::new(125);

    loop {
        clear_background(GREEN);

        population.update();
        population.draw();

        next_frame().await
    }
}
