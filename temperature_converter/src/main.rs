use std::{io, println};

fn main() {
    println!("=== Temperature converter ===");
    
    loop {
        println!("Enter your choice:");
        println!("0. Quit");
        println!("1. °C ---> °F");
        println!("2. °F ---> °C");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // shadowing
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            0 => break,
            1 => println!("Enter temperature in °C:"),
            2 => println!("Enter temperature in °F:"),
            _ => continue
        };

        loop {
            let mut temperature = String::new();

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");
    
            let temperature: f32 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
            match choice {
                1 => {
                    println!("{temperature}°C ===> {}°F\n", temperature * 1.8 + 32.0);
                    break;
                },
                2 => {
                    println!("{temperature}°F ===> {}°C\n", (temperature - 32.0) * 5.0/9.0);
                    break;
                },
                _ => continue
            }
        }
    }
}
