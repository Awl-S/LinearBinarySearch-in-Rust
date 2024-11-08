use std::io::Write;
use std::fs::File;
use std::time::Instant;
use num_format::{Locale, ToFormattedString};
use rand::Rng;
use crate::{perform_binary_search, perform_linear_search};

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

pub fn generate_target() -> i32 {
    let mut rng = rand::thread_rng();
    let mut target = rng.gen_range(0..=100);

    if cfg!(debug_assertions) {
        target = 69;
    }
    target
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


#[cfg(debug_assertions)]
pub fn write_csv_header(csv_file: &mut File) {
    writeln!(csv_file, "Version,Size,Linear Time (ms),Binary Time (ms)")
        .expect("Unable to write to CSV file");
}

#[cfg(debug_assertions)]
pub fn write_to_csv(
    csv_file: &mut File,
    size: u32,
    linear_time: f64,
    binary_time: f64,
    version: u32,
) {
    writeln!(
        csv_file,
        "{},{},{:.6},{:.6}",
        version, size, linear_time, binary_time
    )
        .expect("Unable to write to CSV file");
}


pub fn run_search(size: u32, target: i32, csv_file: &mut File, version: u32) {
    let mut array = generate_random_array(size);

    let (linear_time, linear_result) = perform_linear_search(&array, target);
    println!(
        "Linear search time for size {}: {:.6} ms; Result: {:?}",
        size.to_formatted_string(&Locale::fr), linear_time, linear_result
    );

    array.sort();

    let (binary_time, binary_result) = perform_binary_search(&array, target);
    println!(
        "Binary search time for size {}: {:.6} ms; Result: {:?}",
        size.to_formatted_string(&Locale::fr), binary_time, binary_result
    );

    let memory_usage = (size as usize) * size_of::<i32>();
    println!("Allocated memory for array: {} bytes\n", memory_usage);

    #[cfg(debug_assertions)]
    write_to_csv(csv_file, size, linear_time, binary_time, version);
}
