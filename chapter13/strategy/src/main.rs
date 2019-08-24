/// As of 8/24/2019, Rust Stable doesn't have a generator feature.
/// So iterator is used instead.
struct Pairs<'a> {
    seq: &'a String,
    seq_len: usize,
    curr_idx: usize,
}

impl<'a> Iterator for Pairs<'a> {
    type Item = ( &'a str, &'a str );
    fn next(&mut self) -> Option<( &'a str, &'a str )> {
        if self.curr_idx > (&self.seq_len - 1) {
            None
        } else {
            self.curr_idx = self.curr_idx + 1;
            let right_str_idx = (self.curr_idx) % (self.seq_len);
            return Some(
                ( self.seq.get(self.curr_idx-1..self.curr_idx).unwrap(),
                  self.seq.get(right_str_idx..right_str_idx + 1).unwrap()
                )
            )
        }
    }
}

fn pairs<'a>(seq: &'a String) -> Pairs<'a> {
    Pairs { seq: seq, seq_len: seq.len(), curr_idx: 0, }
}
fn main() {
    println!("Hello, world!");

    let string_a = String::from("abcdefghijklmn");
    for (a, b) in pairs(&string_a) {
        println!("{}, {}", a, b);
    }
}
