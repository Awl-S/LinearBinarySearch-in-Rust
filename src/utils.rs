use std::time::Instant;
use rand::Rng;
pub fn generate_random_array(size:u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();

    (0..size).map(|_| rng.gen_range(0..=100)).collect()
}

pub struct Timer{
    start_time: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self{start_time: None}
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn get_elapsed(&mut self) -> f64 {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            self.start_time = None;
            elapsed.as_secs_f64() * 1000.0
        } else {
            0.0
        }
    }
}