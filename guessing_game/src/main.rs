use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("=== Guessing game ===");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("What is your guess?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing: reuses the same variable name
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // crash the program in a non proper way
        // let guess: u32 = match guess.trim().parse().expect("You must type a number");

        println!("Your guess is: {guess}");

        // match: looks like a switch in other languages
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("It's more"), // comma, not a semicolon
            Ordering::Greater => println!("It's less"),
            Ordering::Equal => {
                println!("Well played!");
                break; // break the loop
            }
        }
    }
}
