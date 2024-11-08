use std::time::Instant;
use rand::Rng;

pub fn generate_random_array(size: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let array: Vec<i32> = (0..size).map(|_| rng.gen_range(0..=100)).collect();

    // Костыль от варнинга о мутирующей переменной array
    #[cfg(debug_assertions)]
    {
        let mut array = array;
        let random_index = rng.gen_range(0..size as usize);
        array[random_index] = 69;
        array
    }

    #[cfg(not(debug_assertions))]
    {
        array
    }
}



pub fn generate_target() -> i32{
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=100)
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