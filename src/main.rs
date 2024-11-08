use std::fs::File;

mod binary_search;
mod linear_search;
mod utils;

fn perform_linear_search(array: &[i32], target: i32) -> (f64, Option<usize>) {
    let mut timer = utils::Timer::new();
    timer.start();
    let result = linear_search::linear_search(array, target);
    let elapsed_time = timer.get_elapsed();
    (elapsed_time, result)
}

fn perform_binary_search(array: &[i32], target: i32) -> (f64, Option<usize>) {
    let mut timer = utils::Timer::new();
    timer.start();
    let result = binary_search::binary_search(array, target);
    let elapsed_time = timer.get_elapsed();
    (elapsed_time, result)
}

fn main() {
    let sizes = [10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000];
    let mut target = utils::generate_target();
    let mut version = 1;
    println!("Target: {:?}\n", target);

    #[cfg(debug_assertions)]
    let mut csv_file = File::create("data.csv").expect("Unable to create CSV file");

    #[cfg(debug_assertions)]
    utils::write_csv_header(&mut csv_file);

    for _ in 0..10 {
        for &size in &sizes {
            if version == 10  {
                target = 101;
            }
            utils::run_search(size, target, &mut csv_file, version);
        }
        version += 1;
    }
}
