use observer::{DefaultFormatter, Observer, HexFormatterObs, BinaryFormatterObs};

fn main() {
    let mut df = DefaultFormatter::new("test1".to_string());
    println!("{}", &df.get_info());

    println!("");
    let hf = Box::new(HexFormatterObs::new());
    df.add(hf);
    df.set_data(3);
    println!("{}", df.get_info());

    println!("");
    let bf = Box::new(BinaryFormatterObs::new());
    df.add(bf);
    df.set_data(21);
    println!("{}", df.get_info());

    println!("");
    df.remove("HexFormatterObs".to_string());
    df.set_data(40);
    println!("{}", df.get_info());

    println!("");
    match df.remove("HexFormatterObs".to_string()) {
        Err(NotExist) => {
            println!("The observer doesn't exist");
        },
        _ => {
        },
    }
    let bf_2 = Box::new(BinaryFormatterObs::new());
    match df.add(bf_2) {
        Err(AlreadyExist) => {
            println!("The observer already exists");
        },
        _ => {
        },
    }

    // This makes compilation to fail since Rust is statically-typed.
    // df.set_data("Hello".to_string());
    // println!("{}", df.get_info());

    // This makes compilation to fail since Rust is statically-typed.
    // println!("");
    // df.set_data(15.8);
    // println!("{}", df.get_info());
}
