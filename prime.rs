// Implement a function that checks whether a given number is prime or not. in rust

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }

    // We only need to check up to the square root of the number
    let max_divisor = (num as f64).sqrt() as u64;

    for i in 2..=max_divisor {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let test_cases = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 30];

    for &num in &test_cases {
        println!("Is {} prime? {}", num, is_prime(num));
    }
}
