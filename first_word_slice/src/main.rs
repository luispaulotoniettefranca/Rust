fn main() {
    let mut buf = String::new();
    println!("Please, input some words to be wrapped");
    let first_word = match std::io::stdin().read_line(&mut buf) {
        Ok(_) => first_word(&buf),
        Err(_) => "Error reading user input!"
    };
    println!("{}", first_word);
}

fn first_word(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
