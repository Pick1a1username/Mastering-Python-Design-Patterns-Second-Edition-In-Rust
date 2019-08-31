use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

use rand::Rng;
use regex::{Regex, Captures};
use serde_json::Value;


#[derive(Debug)]
struct NoTextBoxFound;

impl Error for NoTextBoxFound {}

impl fmt::Display for NoTextBoxFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct TextTooLong;

impl Error for TextTooLong {}

impl fmt::Display for TextTooLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct LengthShorterThanString;

impl Error for LengthShorterThanString {}

impl fmt::Display for LengthShorterThanString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct UnexpectedResult;

impl Error for UnexpectedResult {}

impl fmt::Display for UnexpectedResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Cow {
    name: String,
    max_text_length: usize,
    image: Vec<String>,
}

enum Position {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
struct NoCowFound;

impl Error for NoCowFound {}

impl fmt::Display for NoCowFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Note: Some characters in image may make the function fails.
pub fn load_cows_from_files(path: &Path) -> Result<Vec<Cow>, Box<dyn Error>> {
    let mut cows: Vec<Cow> = Vec::new();
    for data in path.read_dir()? {
        if let Ok(cow_path) = data {
            let raw_data = fs::read_to_string(cow_path.path())?;
            let json_data: Value = serde_json::from_str(&raw_data)?;
            let image_raw = value_vec_to_string_vec(json_data["image"].as_array().unwrap().clone());
            let mut image = Vec::new();

            // Correct some escape characters and quotes.
            // Todo: Make an indenpendent function for this process.
            // https://users.rust-lang.org/t/escape-speech-marks-in-regex/4266/5
            let starting_double_quote_re = Regex::new(r#"^""#).unwrap();
            let ending_double_quote_re = Regex::new(r#""$"#).unwrap();
            let escaped_back_slash_re = Regex::new(r#"\\\\"#).unwrap();

            for line in image_raw {

                let mut temp_line = String::new();
                temp_line = starting_double_quote_re.replace(&line,
                    |_caps: &Captures| { "".to_string() }
                ).to_string();

                temp_line = ending_double_quote_re.replace(&temp_line,
                    |_caps: &Captures| { "".to_string() }
                ).to_string();

                temp_line = escaped_back_slash_re.replace_all(&temp_line,
                    |_caps: &Captures| { "\\".to_string() }
                ).to_string();

                image.push(temp_line);
            }

            cows.push( Cow {
                name: json_data["name"].to_string(),
                max_text_length: get_max_text_len(image.clone()).unwrap(),
                image: image.clone(),
            });
        }
    }
    match cows.is_empty() {
        false => Ok(cows),
        true => Err(Box::new(NoCowFound)),
    }
}

pub fn generate_cow(cow: Cow, text: String) -> Result<String, Box<dyn Error>> {

    if cow.max_text_length < text.len() {
        return Err(Box::new(TextTooLong));
    }

    let text_box_re = Regex::new(r"BEGIN\s+END").unwrap();
    let mut image_vec: Vec<String> = Vec::new();

    for line in cow.image.iter() {
        if let Some(_) = text_box_re.captures(&line) {
            // https://qiita.com/scivola/items/60141f262caa53983c3a
            let replaced_line = text_box_re.replace(&line,
                |_caps: &Captures| {
                    align_string_with_ws(text.clone(), cow.max_text_length, Position::Center).unwrap()
                }
            );
            image_vec.push(replaced_line.to_string());
        } else {
            image_vec.push(line.clone());
        }
    }
    
    for line in image_vec.iter() {
        println!("{}", line);
    }

    return Ok("image_string".to_string());
}

/// Note that the string will be trimmed before being positioned.
fn align_string_with_ws(string: String, length: usize, position: Position) -> Result<String, Box<dyn Error>> {
    let mut trimmed_string = String::new();
    trimmed_string = string.trim().to_string();
    if trimmed_string.len() > length {
        return Err(Box::new(LengthShorterThanString));
    }
    
    match position {
        Position::Left => {
            let mut chars = Vec::new();
            for char in trimmed_string.chars() {
                chars.push(char);
            }
            for _ in 0..(length - trimmed_string.len()) {
                chars.push(' ');
            }
            return Ok(chars.into_iter().collect());
        },
        Position::Center => {
            let mut chars = Vec::new();
            let left_right_ws = length - trimmed_string.len();
            let mut left_ws: usize = 0;
            let mut right_ws: usize = 0;
            
            match left_right_ws % 2 {
                0 => {
                    left_ws = left_right_ws / 2;
                    right_ws = left_right_ws / 2;
                },
                1 => {
                    left_ws = (left_right_ws - 1) / 2;
                    right_ws = ( (left_right_ws - 1) / 2 ) + 1;
                },
                _ => {},
            }

            for _ in 0..(left_ws) {
                chars.push(' ');
            }
            for char in trimmed_string.chars() {
                chars.push(char);
            }
            for _ in 0..(right_ws) {
                chars.push(' ');
            }
            return Ok(chars.into_iter().collect());
        },
        Position::Right => {
            let mut chars = Vec::new();
            for _ in 0..(length - trimmed_string.len()) {
                chars.push(' ');
            }
            for char in trimmed_string.chars() {
                chars.push(char);
            }
            return Ok(chars.into_iter().collect());
        },
    }
}

pub fn milk_random_cow(cows: Vec<Cow>, string: &String) -> String {
    let mut rng = rand::thread_rng();
    let cow_num: usize = cows.len();

    generate_cow(cows[rng.gen_range(0, cow_num)].clone(), string.clone()).unwrap()
}

/// Get the length of the text box in the image.
fn get_max_text_len(image: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let text_box_re = Regex::new(r"^.*(BEGIN\s+END).*$").unwrap();

    for line in image.into_iter() {
        if let Some(text_box) = text_box_re.captures(&line) {
            let length = text_box[1].len().clone();
            return Ok(length);
        }
    }
    return Err(Box::new(NoTextBoxFound));
}

/// Convert a vector of serde_json::Value to a vector of String.
fn value_vec_to_string_vec(value_vec: Vec<Value>) -> Vec<String> {
    let mut string_vec = Vec::new();

    for value in value_vec.iter() {
        string_vec.push(value.to_string());
    }

    return string_vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Todo: Test properly.
    #[test]
    fn test_fn_load_cows_from_files() {
        let path = Path::new("cows/");
        match load_cows_from_files(&path) {
            Ok(_) => { println!("Ok") },
            Err(_) => { println!("Oops!") },
        }
        assert_eq!(1, 1);
    }

    /// Todo: Test properly.
    #[test]
    fn test_fn_get_max_text_len() {
        let image = vec![
            "asdfasdfasdf".to_string(),
            "111BEGIN   END222".to_string(),
            "asdfBEGIN   ENDzxcv".to_string(),
        ];

        get_max_text_len(image);
        assert_eq!(1, 1);
    }

    /// Todo: Test properly.
    #[test]
    fn test_fn_generate_cow() {
        let path = Path::new("cows/");
        let cows = load_cows_from_files(&path).unwrap();
        generate_cow(cows[1].clone(), "hello".to_string());
        assert_eq!(1, 1);
    }

    #[test]
    fn test_fn_align_string_with_ws() {
        let string_1 = "  hello  world  ".to_string();
        let string_2 = "  hello world   ".to_string();

        assert_eq!(align_string_with_ws(string_1.clone(), 20, Position::Left).unwrap(), "hello  world        ".to_string());
        assert_eq!(align_string_with_ws(string_1.clone(), 20, Position::Right).unwrap(), "        hello  world".to_string());
        assert_eq!(align_string_with_ws(string_2.clone(), 20, Position::Center).unwrap(), "    hello world     ".to_string());
        assert_eq!(align_string_with_ws(string_1.clone(), 10, Position::Right).is_err(), true);
    }

    #[test]
    fn test_fn_milk_random_cow() {
        let path = Path::new("cows/");
        let cows = load_cows_from_files(&path).unwrap();

        for i in 1..21 {
            println!("Random cows!");
            milk_random_cow(cows.clone(), &"Hello World!".to_string());
        }
    }
}

