// Implementing a function that checks whether a given string is a palindrome or not

fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    let test_cases = vec![
        "A man, a plan, a canal, Panama!",
        "racecar",
        "hello",
        "Able was I, ere I saw Elba",
    ];

    for &test_case in &test_cases {
        println!("Is '{}' a palindrome? {}", test_case, is_palindrome(test_case));
    }
}
