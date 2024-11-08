
# Project: Performance Analysis of Linear and Binary Search Algorithms in Rust

## Project Description

This project implements two search algorithms — **linear search** and **binary search** — and provides a comparative analysis of them. We use unit tests to verify the correctness of the implementations and measure the execution time of both algorithms across different data volumes. The project serves as an educational example for understanding the differences between algorithms with linear and logarithmic complexity, as well as for studying testing and performance measurement in Rust.

## Project Structure

```plaintext
project-root/
├── src/
│   ├── main.rs                 # Main entry point for the program
│   ├── binary_search.rs         # Binary search algorithm implementation
│   ├── linear_search.rs         # Linear search algorithm implementation
│   └── utils.rs                 # Helper functions (data generation, timing)
├── Cargo.toml                   # Cargo configuration file
└── README.md                    # Project documentation
```

## Installation

1. Clone the repository to your local computer.
   ```bash
   git clone https://github.com/Awl-S/LinearBinarySearch-in-Rust.git
   cd LinearBinarySearch-in-Rust
   ```

2. Ensure Rust and Cargo are installed, then build the project.
   ```bash
   cargo build
   ```

## Running the Program

To run the program, use the command:

```bash
cargo run
```

## Testing

The project includes both unit and integration tests to verify the functionality of the search algorithms. To run all tests, execute the command:

```bash
cargo test
```

## Using the Algorithms

### Linear Search

Linear search performs a sequential check of all array elements until the target value is found. It has a time complexity of **O(n)** and is suitable for small and unsorted arrays.

### Binary Search

Binary search requires a pre-sorted array and uses a divide-and-conquer approach, reducing the search space by half with each step. Its time complexity is **O(log n)**, making it efficient for large, sorted arrays.

## Example Usage

Example usage of the search functions and their timing in `main.rs`:

```rust
fn main() {
    let array = utils::generate_random_array(1000); // Generate a random array
    let target = 50;

    // Linear search
    let linear_result = linear_search::linear_search(&array, target);
    println!("Linear Search: result = {:?}", linear_result);

    // Binary search (first sort the array)
    let mut sorted_array = array.clone();
    sorted_array.sort();
    let binary_result = binary_search::binary_search(&sorted_array, target);
    println!("Binary Search: result = {:?}", binary_result);
}
```

## License

This project is copyright-protected and intended solely for personal and educational use. Commercial use requires prior written permission from the author. If you plan to use this project in academic publications, such as conferences, reports, articles, or scientific papers, please notify me.

Please note that the license described in this README takes precedence over any text automatically added to each source code file and remains the primary license for the project. All rights reserved, and the code is provided "as is" without any guarantees, including guarantees of fitness for a particular purpose or commercial value.

---

The project is created for demonstration of algorithms and Rust practice.
