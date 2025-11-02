use std::io;

fn longest_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut longest = &s[..0];

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let word = &s[start..i];
            if word.len() > longest.len() {
                longest = word;
            }
            start = i + 1;
        }
    }
    
    let word = &s[start..];
    if word.len() > longest.len() {
        longest = word;
    }

    longest
}

fn shortest_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut shortest = &s[..];
    let mut found_first = false;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let word = &s[start..i];
            if !found_first || word.len() < shortest.len() {
                shortest = word;
                found_first = true;
            }
            start = i + 1;
        }
    }

    let word = &s[start..];
    if !found_first || word.len() < shortest.len() {
        shortest = word;
    }

    shortest
}

fn main() {
    let mut my_string = String::new();
    io::stdin().read_line(&mut my_string).unwrap();
    let my_string = my_string.trim().to_string();

    println!("Shortest word: {}", shortest_word(&my_string));
    println!("Longest word: {}", longest_word(&my_string));
}
