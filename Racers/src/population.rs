use crate::car::*;
use crate::timer::*;
use crate::track::*;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::fs::File;
use std::io::prelude::*;
use std::time;

pub struct Population {
    generation: usize,
    cars: Vec<Car>,
    track: Track,
    ticks: u32,
    timer: Timer,
    data_file: File,
    time_limit: u32,
}

impl Population {
    pub fn new(size: usize, time_limit: u32) -> Self {
        let track: Track = Track::new(test_track1, 100.0);
        let mut cars = vec![];
        for i in 0..size {
            // set car numbers as i+1, e.g. first car will get number 1
            cars.push(Car::new(track.get_start_pos(), i + 1));
        }

        Self {
            generation: 0,
            cars,
            track,
            ticks: 0,
            time_limit,
            timer: Timer::new(),
            data_file: File::create("fitness_values_test1.csv").unwrap(),
        }
    }

    pub fn draw(&self) {
        self.track.draw();

        // find best performer
        let mut best_fitness = -1000000; // close enough to - infinity
        let mut best_car_number: usize = 0;
        for i in 0..self.cars.len() {
            let car = &self.cars[i];
            if car.fitness > best_fitness {
                best_fitness = car.fitness;
                best_car_number = car.number;
            }
        }

        // draw cars
        for i in 0..self.cars.len() {
            let car = &self.cars[i];
            if car.number == best_car_number {
                car.draw(true);
            } else {
                car.draw(false);
            }
        }

        // draw the generation number
        let text = format!("Generation: {}", self.generation);
        let colour = color_u8!(20, 20, 20, 100);
        draw_text(&text, 400.0, 350.0, 75.0, colour);

        // draw the timer bar
        self.draw_timer_bar();
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        if self.ticks >= self.time_limit || self.all_cars_crashed() {
            self.new_population();
        }

        for car in self.cars.iter_mut() {
            car.update(&self.track, dt);
            if !car.is_on_track(&self.track) {
                car.crashed();
            }

            if car.just_lapped {
                self.timer
                    .enter_time((car.number, self.generation, car.lap_time as f32));
                print!("{esc}c", esc = 27 as char);
                self.timer.print_times();
            }
        }

        self.ticks += 1;
    }

    fn new_population(&mut self) {
        // reset all the cars to start position
        let size = self.cars.len();
        let mut cars: Vec<Car> = vec![];

        // 100% of children made by top 2 performers
        self.cars.sort_by(|a, b| {
            b.get_final_fitness(self.ticks + 1)
                .cmp(&a.get_final_fitness(self.ticks + 1))
        });

        for i in 0..(size) {
            cars.push(self.reproduce(&self.cars[0], &self.cars[1], i));
        }

        // add data to csv file
        let best_fitness = self.cars[0].get_final_fitness(self.ticks + 1);
        writeln!(self.data_file, "{},{}", self.generation, best_fitness).unwrap();

        println!(
            "GEN [{}] - Best Fitness = {}",
            self.generation, best_fitness
        );

        self.cars = cars;

        self.ticks = 0;

        self.generation += 1;
    }

    fn all_cars_crashed(&self) -> bool {
        for car in self.cars.iter() {
            if !car.crashed {
                return false;
            }
        }
        return true;
    }

    fn reproduce(&self, car1: &Car, car2: &Car, i: usize) -> Car {
        let mut child_car = Car::new(self.track.get_start_pos(), i + 1);
        let mut child_net = car1.brain.clone();
        let network2 = &car2.brain;

        // apply cross over
        for i in 0..child_net.layers.len() {
            let layer2 = &network2.layers[i];
            let biases2 = &layer2.bias;
            let weights2 = &layer2.weights;

            let weights_size = weights2.len() * weights2[0].len();
            let biases_size = biases2.len();

            let weights_crossover = gen_range(0, weights_size - 1);
            let biases_crossover = gen_range(0, biases_size - 1);

            let child_layer = &mut child_net.layers[i];

            // cross over the weights
            for j in 0..=weights_crossover {
                let new_weight = weights2[j / weights2[0].len()][j % weights2[0].len()];
                child_layer.weights[j / weights2[0].len()][j % weights2[0].len()] = new_weight;
            }

            // cross over the biases
            for k in 0..=biases_crossover {
                let new_bias = biases2[k];
                child_layer.bias[k] = new_bias;
            }

            // apply mutations
            for row in child_layer.weights.iter_mut() {
                for weight in row.iter_mut() {
                    if gen_range(0.0, 1.0) <= 0.02 {
                        *weight = gen_range(-1.0, 1.0);
                    }
                    if gen_range(0.0, 1.0) <= 0.03 {
                        *weight += gen_range(-0.5, 0.5);
                    }
                }
            }

            for bias in child_layer.bias.iter_mut() {
                if (gen_range(0.0, 1.0)) <= 0.02 {
                    *bias = gen_range(-0.5, 0.5);
                }
                if (gen_range(0.0, 1.0) <= 0.03) {
                    *bias += gen_range(-0.5, 0.5);
                }
            }
        }

        child_car.brain = child_net;

        return child_car;
    }

    pub fn draw_timer_bar(&self) {
        // set the height of the bar
        let height = 30.0;
        let draw_pos = Vec2::new(0.0, WINDOW_HEIGHT as f32 - height);

        let width = (WINDOW_WIDTH as f32) * (self.ticks as f32) / (self.time_limit as f32);

        // draw the rectangle with corresponding width and height
        draw_rectangle(
            draw_pos.x,
            draw_pos.y,
            WINDOW_WIDTH as f32 - width,
            height,
            YELLOW,
        );

        // draw the label above the bar
        draw_text(
            "Generation Time Left:",
            10.0,
            WINDOW_HEIGHT as f32 - height - 30.0,
            35.0,
            BLACK,
        );
    }
}
