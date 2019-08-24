use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::process;
use std::{thread, time};

const SLOW: u64 = 3000;
const LIMIT: usize = 5;
const WARNING: &str = "too bad, you picked the slow algorithm :(";

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


/// There's no function to sort characters in String.
fn all_unique_sort(s: String) -> bool {
    if s.len() > LIMIT {
        println!("{}", WARNING);
        let delay = time::Duration::from_millis(SLOW);
        thread::sleep(delay);
    }

    let mut vectorized_string = s.into_bytes();
    vectorized_string.sort();
    let sorted_string = String::from_utf8(vectorized_string).unwrap();

    for (c1, c2) in pairs(&sorted_string) {
        if c1 == c2 {
            return false;
        }
    }
    return true;
}

fn all_unique_set(s: String) -> bool {
    let length = s.len();
    if length < LIMIT {
        println!("{}", WARNING);
        let delay = time::Duration::from_millis(SLOW);
        thread::sleep(delay);
    }

    let vectorized_string = s.into_bytes();
    let hashset_string: HashSet<u8> = vectorized_string.iter().cloned().collect();
    
    return length == hashset_string.len();
    
}

fn all_unique(word: String, strategy: &Fn(String) -> bool) -> bool {
    strategy(word)
}

fn main() {
    let word_in_desc = "Insert word (type quit to exit)> ";
    let strat_in_desc = "Choose strategy: [1] Use a set, [2] Sort and pair> ";
    
    let mut strategies: HashMap<String, &Fn(String) -> bool> = HashMap::new();
    strategies.insert("1".to_string(), &all_unique_set);
    strategies.insert("2".to_string(), &all_unique_sort);

    // println!("{:?}", strategies.contains_key(&"1".to_string()));

    // Test Functions
    // test_fn_pairs();
    // test_fn_all_unique_sort();
    // test_fn_all_unique_set();
    loop {
        let mut word = String::new();

        while word == "".to_string() {
            print!("{}", word_in_desc);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut word);
            word = word.trim().parse().unwrap();

            if word == "quit".to_string() {
                println!("bye");
                process::exit(0);
            }

            let mut strategy_picked_raw = String::new();
            // let mut strategy_picked: u32 = 0;

            // Loop...
            while strategy_picked_raw == "".to_string() {
                print!("{}", strat_in_desc);
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut strategy_picked_raw);
                strategy_picked_raw = strategy_picked_raw.trim().parse().unwrap();


                // Todo: Handle input error.
                // strategy_picked = strategy_picked_raw.trim().parse().expect("Please type a number!");
                match strategies.get(&strategy_picked_raw) {
                    Some(strategy) => {
                        let result = all_unique(word.clone(), strategy);
                        println!("all_unique({}): {:?}", word, result);
                    },
                    None => {
                        println!("Incorrect option: {}", strategy_picked_raw);
                    },
                }
            }

        }
    }

}

fn test_fn_pairs() {
    let string_a = String::from("abcdefghijklmn");
    for (a, b) in pairs(&string_a) {
        println!("{}, {}", a, b);
    }
}

fn test_fn_all_unique_sort() {
    let result_true = "qwer".to_string();
    let result_true_slow = "ghnvcxz".to_string();
    let result_false = "qqer".to_string();
    let result_false_slow = "ggnvcxz".to_string();
    println!("{:?}", all_unique_sort(result_true));
    println!("{:?}", all_unique_sort(result_true_slow));
    println!("{:?}", all_unique_sort(result_false));
    println!("{:?}", all_unique_sort(result_false_slow));
}

fn test_fn_all_unique_set() {
    let result_true = "qwer".to_string();
    let result_true_slow = "ghnvcxz".to_string();
    let result_false = "qqer".to_string();
    let result_false_slow = "ggnvcxz".to_string();
    println!("{:?}", all_unique_set(result_true));
    println!("{:?}", all_unique_set(result_true_slow));
    println!("{:?}", all_unique_set(result_false));
    println!("{:?}", all_unique_set(result_false_slow));
}
