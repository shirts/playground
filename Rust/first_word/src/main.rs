fn main() {
    let words = String::from("LycheeKenny is cool");
    let first = first_word(&words);
    println!("The first word in \"{}\" is \"{}\"", words, first);
}

fn first_word(string: &String) -> &str {
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[..i];
        }
    }

    return &string;
}
