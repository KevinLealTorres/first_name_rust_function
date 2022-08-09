fn first_word(word: String) -> String {

    let mut result: String = String::new();

    for character in word.chars() {
        if character == ' ' {
            break;
        }
        result.push(character);
    }
    return result;
}

//Driver function
fn main() {
    let word: String = String::from(first_word("hello world".to_string())); //Result: hello
    println!("{word}");
}
