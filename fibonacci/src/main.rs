use std::io;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    println!("=== Fibonacci calculator ===");

    loop {
        println!("Enter n (or 0 to quit):");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        // shadowing
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n == 0 {
            break;
        }

        println!("Fibonacci({n}) = {}\n", fibonacci(n));
    }
}
