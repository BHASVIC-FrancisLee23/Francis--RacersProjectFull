use crate::population::Population;
use crate::{
    ui::{Button, Slider},
    WINDOW_HEIGHT, WINDOW_WIDTH,
};
use macroquad::prelude::*;

#[derive(PartialEq)]
enum ProgramStatus {
    MainMenu,
    Simulation,
}

pub struct App {
    status: ProgramStatus,
    population: Option<Population>,

    // ui elements
    buttons: Vec<Button>,
    sliders: Vec<Slider>,
    end_button: Option<Button>, // new line
}

impl App {
    pub fn new() -> Self {
        Self {
            status: ProgramStatus::MainMenu,
            population: None,

            // ui design
            buttons: vec![Button::new(
                400.0,
                550.0,
                400.0,
                150.0,
                "Run".to_string(),
                LIGHTGRAY,
            )],

            end_button: None, // set to none by default

            sliders: vec![
                Slider::new(600.0, 350.0, 10, 300, 220),
                Slider::new(600.0, 450.0, 500, 3000, 1250),
            ],
        }
    }

    pub fn update(&mut self) {
        if self.status == ProgramStatus::MainMenu {
            // updates for main menu instance
            for b in &mut self.buttons {
                b.check_pressed();
            }
            for s in &mut self.sliders {
                s.update();
            }

            // if the first button is pressed, (aka the 'Run' button)
            if self.buttons[0].pressed {
                self.status = ProgramStatus::Simulation;
                // create population
                let pop_size = self.sliders[0].value;
                let gen_length = self.sliders[1].value;

                self.population = Some(Population::new(pop_size as usize, gen_length as u32));

                // create the end simulation button
                // button width is 150px height is 75px
                self.end_button = Some(Button::new(
                    WINDOW_WIDTH as f32 - 150.0,
                    WINDOW_HEIGHT as f32 - 150.0, // shift up 150px from bottom rather than 75px
                    150.0,
                    75.0,
                    "End".to_string(),
                    RED,
                ));

                // reset the UI components
                for b in self.buttons.iter_mut() {
                    b.reset();
                }
                for s in self.sliders.iter_mut() {
                    s.reset();
                }
            }
        } else {
            // run the simulation

            if let Some(pop) = &mut self.population {
                pop.update();
            }

            // update the end button
            if let Some(end_button) = &mut self.end_button {
                end_button.check_pressed();

                if end_button.pressed {
                    self.population = None;
                    self.status = ProgramStatus::MainMenu;

                    // reset the ui components
                    self.end_button.as_mut().unwrap().reset();

                    self.end_button = None;
                }
            }
        }
    }

    pub fn draw(&self) {
        if self.status == ProgramStatus::MainMenu {
            // in main menu stage

            // draw the ui of the main screen
            for b in &self.buttons {
                b.draw();
            }
            for s in &self.sliders {
                s.draw();
            }

            // draw the labels for the ui elements
            draw_text("Population Size:", 100.0, 350.0, 30.0, BLACK);
            draw_text("Generation Time Limit:", 100.0, 450.0, 30.0, BLACK);
            draw_text("(Ticks)", 140.0, 475.0, 18.0, BLACK);
            draw_text("Create Simulation", 300.0, 150.0, 75.0, BLACK);
        } else {
            // inside a simulation so draw it!

            if let Some(pop) = &self.population {
                pop.draw();
            }

            // draw the end button
            if let Some(end_button) = &self.end_button {
                end_button.draw();
            }
        }
    }
}
