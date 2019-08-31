// use cow_rs;
use std::path::Path;

fn generate_banner(msg: &String, style: &Fn(&String) -> String) {
    println!("-- start of banner --");
    println!("{}", style(msg));
    println!("-- end of banner --nn");
}

fn dots_style(msg: &String) -> String {
    // Capitalize msg.
    // msg = msg.to_uppercase();
    let new_msg = String::from(format!("..........{}..........", msg));
    return new_msg;
}

fn admire_style(msg: &String) -> String {
    let msg_uppered = msg.to_uppercase();
    let msg_chars = msg_uppered.chars();
    let mut msg_vec = Vec::new();

    // https://www.quora.com/How-do-you-iterate-over-the-characters-of-a-string-in-RUST
    for character in msg_chars {
        msg_vec.push(character);
        msg_vec.push('!');
    }

    // https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string
    let new_msg = msg_vec.into_iter().collect();

    return new_msg;
}

/// Capitalize words in String variable.
/// Todo: Simplify code.
fn capitalize(string: String) -> String {
    let string_chars = string.chars();

    let mut is_cap = false;
    let mut is_first_char = true;
    let mut chars = Vec::new();

    for character in string_chars {
        if character.is_whitespace() {
            is_cap = true;
            chars.push(character);
        } else {
            if is_cap == true || is_first_char == true {
                let char_upperred = character.to_uppercase();
                let char_stringified = char_upperred.to_string();
                let char_converted = char_stringified.chars();
                for cap in char_converted {
                    chars.push(cap);
                }
                is_cap = false;
                is_first_char = false;
            } else {
                chars.push(character);
            }
        }
    }

    let new_string = chars.into_iter().collect();

    return new_string;
}

fn cow_style(msg: &String) -> String {
    let cows_path = Path::new("cows/");
    let cows = cow_rs::load_cows_from_files(&cows_path).unwrap();

    match cow_rs::milk_random_cow(cows.clone(), &msg) {
        Ok(cow) => cow,
        Err(err) => match err {
            TextTooLong => "The message is too long.".to_string(),
            _ => "Something went wrong.".to_string(),
        },
    }
}

/// Todo: Pass the directory of cows as an argument.
fn main() {
    // Test Functions
    // test_fn_dots_style();
    // test_fn_admire_style();
    // test_fn_capitalize();

    let msg = "happy coding".to_string();
    let long_msg = "haaaaaaaaaaaaaappyyyyyy cooooooodiiiiing".to_string();

    generate_banner(&msg, &dots_style);
    generate_banner(&msg, &admire_style);
    generate_banner(&msg, &cow_style);
    generate_banner(&long_msg, &cow_style);



}

fn test_fn_dots_style() {
    let test_string = "happy coding".to_string();
    println!("{}", dots_style(&test_string));
}

fn test_fn_admire_style() {
    // let test_string = "happy coding".to_string();
    let test_string = "안녕하세요".to_string();
    println!("{}", admire_style(&test_string));
}

fn test_fn_capitalize() {
    let test_string = "happy coding".to_string();
    println!("{}", capitalize(test_string));
}
