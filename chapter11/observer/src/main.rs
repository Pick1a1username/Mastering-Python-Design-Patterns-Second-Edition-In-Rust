use std::rc::Rc;

use observer::{DefaultFormatter, Observer, HexFormatterObs};

fn main() {
    let mut df = DefaultFormatter::new("test1".to_string());
    println!("{}", &df.get_info());

    println!("");
    let hf = Box::new(HexFormatterObs::new());
    df.add(hf);
    df.set_data(3);
    println!("{}", df.get_info());
}
