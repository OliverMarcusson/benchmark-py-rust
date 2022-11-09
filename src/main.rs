use std::io::{stdin, stdout, Write};
use std::{time::Instant};

fn input_string(prompt: &str) -> String {
    print!("{prompt}");
    stdout().flush().unwrap();
    
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Failed to read line.");
    
    let mut input = input.as_str();
    input = input.trim();
    let input = input.to_string();
    
    return input;
}

fn main() {
    let iterations = input_string("Iterations: ");
    let iterations: u64 = iterations.parse().unwrap();
    let instant = Instant::now();
    let _: f64 = 0.0;
    for i in 1..=iterations {
        let j: f64 = i as f64;
        _ = 1.0 / j;
    }
    println!("Done, took {:?}.", instant.elapsed())
}
