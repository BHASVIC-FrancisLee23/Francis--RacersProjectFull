use crate::utils::repeat_space;

pub struct Timer {
    times: [LapTime; 10],
}

// (car num, generation, time)
pub type LapTime = (usize, usize, f32);

impl Timer {
    pub fn new() -> Self {
        Self {
            times: [(0, 0, 0.0); 10],
        }
    }

    pub fn get_slowest_time(&self) -> LapTime {
        return self.times[9];
    }

    pub fn get_fastest_time(&self) -> LapTime {
        return self.times[0];
    }

    pub fn get_times(&self) -> [LapTime; 10] {
        return self.times;
    }

    pub fn enter_time(&mut self, time: LapTime) -> i32 {
        // loop through all times and place in order
        for i in (0..self.times.len()) {
            let t = &self.times[i];
            if (t.0 == 0 || (time.2 < t.2)) {
                // shift all times down 1 after this
                self.times[i..].rotate_right(1);

                // place time in this spot
                self.times[i] = time;

                return i as i32;
            }
        }

        return -1;
    }

    pub fn print_times(&self) {
        // each column will be 10 spaces width
        println!("__________________________________");
        println!("|Car no.   |Generation|Lap Time  |");
        println!("__________________________________");

        for i in 0..self.times.len() {
            let t = self.times[i];
            print!("|{}", t.0);
            repeat_space(10 - format!("{}", t.0).len());
            print!("|{}", t.1);
            repeat_space(10 - format!("{}", t.1).len());
            print!("|{}", t.2);
            repeat_space(10 - format!("{}", t.2).len());
            println!("|");
        }

        println!("__________________________________");
    }
}
