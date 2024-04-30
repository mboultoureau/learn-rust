// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.


fn magnitude(v: &[f64;3]) -> f64 {
    let mut sum = 0.0;
    for coord in v {
        sum += coord * coord;
    }
    sum.sqrt()

    // Other solution
    // let sum = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    // sum.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


fn normalize(v: &mut [f64;3]) {
    let mag = magnitude(v);
    for item in v {
        *item /= mag;
    }

    // Other solution
    // let magnitude = magnitude(v);
    // v[0] = v[0] / magnitude;
    // v[1] = v[1] / magnitude;
    // v[2] = v[2] / magnitude;
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}