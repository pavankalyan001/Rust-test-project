// Check if a number is prime in Rust

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }

    let max_divisor = (num as f64).sqrt() as u64;
    for i in 2..=max_divisor {
        if num % i == 0 {
            return false; 
	}
    }
    true
}

fn main() {
    let test_cases = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    for num in test_cases {
        if is_prime(num) {
            println!("{} is prime", num);
        } else {
            println!("{} is not prime", num);
        }
    }
}
