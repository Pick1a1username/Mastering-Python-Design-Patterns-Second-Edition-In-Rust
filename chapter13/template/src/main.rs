fn generate_banner(msg: String, style: &Fn(String) -> String) {
    println!("-- start of banner --");
    println!("{}", style(msg));
    println!("-- end of banner --nn");
}

fn dots_style(mut msg: String) -> String {
    // Capitalize msg.
    // msg = msg.to_uppercase();
    msg = String::from(format!("..........{}..........", msg));
    return msg;
}

fn admire_style(mut msg: String) -> String {
    msg = msg.to_uppercase();
    // Todo: Add an exclamation mark between each character.

    return msg;
}

/// Capitalize characters in String variable.
fn capitalize(string: String) -> String {
    unimplemented!()
}

fn cow_style(msg: String) -> String {
    unimplemented!()
}

/// Since there is no crate like cowpy in Python,
/// make a simple ascii character generator.
fn milk_random_cow(msg: String) -> String {
    unimplemented!()
}

fn main() {
    println!("Hello, world!");
}
