
use std::io;

// Define the Counter structure
struct Counter {
    count: u64,
}

// Implementation block for Counter
impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    // A method to increment the count
    fn increment(&mut self) {
        self.count += 1;
        println!("Counter incremented!");
    }

    // A method to get the current count
    fn get_count(&self) -> u64 {
        self.count
    }

    // A method to reset the count
    fn reset(&mut self) {
        self.count = 0;
        println!("Counter reset!");
    }
}

fn main() {
    let mut counter = Counter::new();
    println!("Initial count: {}", counter.get_count());

    loop {
        println!("\n--- Rust Counter Menu ---");
        println!("1. Increment Counter");
        println!("2. View Count");
        println!("3. Reset Counter");
        println!("4. Exit");
        print!("Please enter your choice: ");

        
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                counter.increment();
                println!("Current count: {}", counter.get_count());
            }
            Ok(2) => {
                println!("Current count: {}", counter.get_count());
            }
            Ok(3) => {
                counter.reset();
            }
            Ok(4) => {
                println!("Exiting. Final count: {}", counter.get_count());
                break;
            }
            _ => {
                println!("Invalid input. Please enter a number from 1 to 4.");
            }
        }
    }
}