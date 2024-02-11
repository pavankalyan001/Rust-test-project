// Given a string of words, implement a function that returns the shortest word in the string. in rust

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    if let Some(shortest) = shortest_word(sentence) {
        println!("The shortest word is: {}", shortest);
    } else {
        println!("The input string is empty");
    }
}
