use std::io;

fn last_word(s: &String) -> &str{
    let bytes = s.as_bytes();


    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}

fn main() {
    let mut my_string: String = String::new();
    io::stdin().read_line(&mut my_string).unwrap();
    let my_string = my_string.trim().to_string();
    let word: &str = last_word(&my_string);

    let my_literal: String = word.to_string();
    let word: &str = last_word(&my_literal);
    println! ("The last word is: {}", word);
}
