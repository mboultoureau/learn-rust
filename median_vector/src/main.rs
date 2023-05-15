use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut values: Vec<i32> = (0..100).map(|_| rand::thread_rng().gen_range(-100..=100)).collect();
    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Values: {:?}", values);

    values.sort();

    // println!("Sorted values: {:?}", values);

    // Median
    let median = if values.len() % 2 == 0 {
        let mid = values.len() / 2;
        (values[mid] + values[mid - 1]) / 2
    } else {
        values[values.len() / 2]
    };

    println!("Median: {}", median);

    // Mode (most frequent value)
    let mut frequencies =  HashMap::new();

    for value in values.iter() {
        let count = frequencies.entry(value).or_insert(0);
        *count += 1;
    }

    // println!("Frequencies: {:?}", frequencies);

    let mut mode = 0;
    let mut max_count = 0;

    for (value, count) in frequencies.iter() {
        if *count > max_count {
            mode = **value;
            max_count = *count;
        }
    }

    println!("Mode: {}", mode);
}
