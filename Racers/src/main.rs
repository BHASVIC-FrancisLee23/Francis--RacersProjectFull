use app::App;
use macroquad::prelude::*;

pub mod app;
pub mod car;
pub mod network;
pub mod population;
pub mod timer;
pub mod track;
pub mod ui;
pub mod utils;

// constants
pub const WINDOW_WIDTH: i32 = 1200;
pub const WINDOW_HEIGHT: i32 = 800;

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

    let mut app = App::new();

    loop {
        clear_background(GREEN);

        app.update();
        app.draw();

        next_frame().await;
    }
}
