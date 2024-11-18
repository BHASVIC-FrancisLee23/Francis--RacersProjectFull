use crate::car::*;
use crate::network::*;
use crate::track::*;
use macroquad::prelude::*;

pub struct Population {
    generation: usize,
    cars: Vec<Car>,
    track: Track,
}

impl Population {
    pub fn new(size: usize) -> Self {
        let track: Track = Track::new(test_track1, 75.0);
        let mut cars = vec![];
        for i in 0..size {
            cars.push(Car::new(track.get_start_pos()));
        }

        Self {
            generation: 0,
            cars,
            track,
        }
    }

    pub fn draw(&self) {
        self.track.draw();
        // cast rays
        for car in self.cars.iter() {
            car.draw();
        }
    }

    pub fn update(&mut self) {
        for car in self.cars.iter_mut() {
            car.update(&self.track);
        }
    }
}
