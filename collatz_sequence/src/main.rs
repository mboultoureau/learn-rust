/// Determine the length of the collatz sequence beginning at `n`.
// fn collatz_length(mut n: i32) -> u32 {
//     let mut length: u32 = 0;

//     loop {
//         length += 1;

//         if n == 1 {
//             break
//         }

//         if n % 2 == 0 {
//             n = n / 2
//         } else {
//             n = 3 * n + 1
//         }
//     }

//     length
// }

fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;

    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        length += 1;
    }

    length
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    let n: i32 = 3;
    println!("{n} will become 1 in {} steps", collatz_length(n));
}
