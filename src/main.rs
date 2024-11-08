mod binary_search;
mod linear_search;
mod utils;

fn main() {
    let sizes = [10, 100, 1000, 10_000]; // Размеры массивов для тестирования
    let target = 50;

    for &size in &sizes {
        let mut array = utils::generate_random_array(size);

        let mut timer = utils::Timer::new();
        timer.start();
        let linear_result = linear_search::linear_search(&array, target);
        let linear_time = timer.get_elapsed();
        println!(
            "Linear search time for size {}: {:.6} ms; Result: {:?}",
            size, linear_time, linear_result
        );

        array.sort();

        timer.start();
        let binary_result = binary_search::binary_search(&array, target);
        let binary_time = timer.get_elapsed();
        println!(
            "Binary search time for size {}: {:6} ms; Result: {:?}",
            size, binary_time, binary_result
        );

        println!("Allocated memory for array: {} bytes\n", (size as usize) * std::mem::size_of::<i32>());
    }
}