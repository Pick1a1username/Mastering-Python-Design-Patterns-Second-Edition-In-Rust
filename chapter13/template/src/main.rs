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

/// Note that this may not work if msg is not English.
fn admire_style(msg: &String) -> String {
    let msg_uppered = msg.to_uppercase();
    let msg_chars = msg_uppered.chars();
    // Todo: Add an exclamation mark between each character.
    let mut msg_vec = Vec::new();

    // https://www.quora.com/How-do-you-iterate-over-the-characters-of-a-string-in-RUST
    for character in msg_chars {
        msg_vec.push(character);
        // msg_vec.push(33);
        msg_vec.push('!');
    }

    // https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string
    let new_msg = msg_vec.into_iter().collect();

    return new_msg;
}

/// Capitalize characters in String variable.
fn capitalize(string: &String) -> String {
    unimplemented!()
}

fn cow_style(msg: &String) -> String {
    unimplemented!()
}

/// Since there is no crate like cowpy in Python,
/// make a simple ascii character generator.
fn milk_random_cow(msg: &String) -> String {
    unimplemented!()
}

fn main() {
    // Test Functions
    // test_fn_dots_style();
    test_fn_admire_style();
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
