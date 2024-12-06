use crate::car::*;
use crate::track::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::fs::File;
use std::io::prelude::*;

pub const GENERATION_TIME: u32 = 1000;

pub struct Population {
    generation: usize,
    cars: Vec<Car>,
    track: Track,
    timer: u32,
    data_file: File,
}

impl Population {
    pub fn new(size: usize) -> Self {
        let track: Track = Track::new(test_track1, 100.0);
        let mut cars = vec![];
        for i in 0..size {
            cars.push(Car::new(track.get_start_pos()));
        }

        Self {
            generation: 0,
            cars,
            track,
            timer: 0,
            data_file: File::create("fitness_values_test1.csv").unwrap(),
        }
    }

    pub fn draw(&self) {
        self.track.draw();

        // find best performer
        let mut best_fitness = -1000000;
        let mut best_index = 0;
        for i in 0..self.cars.len() {
            if self.cars[i].fitness > best_fitness {
                best_fitness = self.cars[i].fitness;
                best_index = i;
            }
        }

        // draw cars
        for i in 0..self.cars.len() {
            let car = &self.cars[i];
            if i == best_index {
                car.draw(false);
            } else {
                car.draw(false);
            }
        }
    }

    pub fn update(&mut self) {
        if self.timer >= GENERATION_TIME || self.all_cars_crashed()
        //|| self.all_cars_completed_lap()
        {
            self.new_population();
        }

        for car in self.cars.iter_mut() {
            car.update(&self.track);
            if !car.is_on_track(&self.track) {
                car.crashed();
            }
        }

        self.timer += 1;
    }

    fn new_population(&mut self) {
        // reset all the cars to start position
        let size = self.cars.len();
        let mut cars: Vec<Car> = vec![];

        // 100% of children made by top 2 performers
        self.cars
            .sort_by(|a, b| b.get_final_fitness().cmp(&a.get_final_fitness()));

        for i in 0..(size) {
            cars.push(self.reproduce(self.cars[0].clone(), self.cars[1].clone()));
        }

        // add data to csv file
        let best_fitness = self.cars[0].get_final_fitness();
        writeln!(self.data_file, "{},{}", self.generation, best_fitness).unwrap();

        println!(
            "GEN [{}] - Best Fitness = {}",
            self.generation, best_fitness
        );

        self.cars = cars;

        self.timer = 0;

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

    fn all_cars_completed_lap(&self) -> bool {
        for car in self.cars.iter() {
            if (!car.crashed) {
                if (car.laps == 0 || car.laps == 1) {
                    // lap 1 counts as the first lap
                    return false;
                }
            }
        }
        return true;
    }

    fn reproduce(&self, car1: Car, car2: Car) -> Car {
        let mut child_car = Car::new(self.track.get_start_pos());
        let mut child_net = car1.brain.clone();
        let network2 = car2.brain.clone();

        // apply cross over
        for i in 0..child_net.layers.len() {
            let layer2 = network2.layers[i].clone();
            let biases2 = layer2.bias;
            let weights2 = layer2.weights;

            let weights_size = weights2.len() * weights2[0].len();
            let biases_size = biases2.len();

            let weights_crossover = gen_range(0, weights_size - 1);
            let biases_crossover = gen_range(0, biases_size - 1);

            let child_layer = &mut child_net.layers[0];

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
            let child_layer = &mut child_net.layers[0];

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
}
