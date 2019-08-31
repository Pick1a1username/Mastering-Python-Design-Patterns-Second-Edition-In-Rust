use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

use rand::Rng;
use regex::{Regex, Captures};
use serde_json::Value;

/// For that there's no textbox in image.
#[derive(Debug)]
struct NoTextBoxFound;

impl Error for NoTextBoxFound {}

impl fmt::Display for NoTextBoxFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// For that text is too long to be written to textbox.
#[derive(Debug)]
pub struct TextTooLong;

impl Error for TextTooLong {}

impl fmt::Display for TextTooLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// For that length of aligned text is shorter than trimmed text.
/// Refer to align_string_with_ws() for more details.
#[derive(Debug)]
struct LengthShorterThanString;

impl Error for LengthShorterThanString {}

impl fmt::Display for LengthShorterThanString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Used when Error is not expected.
#[derive(Debug)]
struct UnexpectedResult;

impl Error for UnexpectedResult {}

impl fmt::Display for UnexpectedResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Used to store image.
#[derive(Clone)]
pub struct Cow {
    name: String,
    max_text_length: usize,
    image: Vec<String>,
}

/// For that there's no cow data in the specified directory.
#[derive(Debug)]
struct NoCowFound;

impl Error for NoCowFound {}

impl fmt::Display for NoCowFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// For text alignment.
enum Position {
    Left,
    Center,
    Right,
}

/// Load images from files.
///
/// Note: Some characters in image may make the function fails.
pub fn load_cows_from_files(path: &Path) -> Result<Vec<Cow>, Box<dyn Error>> {
    // Initialize a vector for images.
    let mut cows: Vec<Cow> = Vec::new();

    // Read each file from the directory.
    for data in path.read_dir()? {

        // If at least one file exists...
        if let Ok(cow_path) = data {
            // Read data as String.
            let raw_data = fs::read_to_string(cow_path.path())?;
            // Convert raw data(String) to Json.
            let json_data: Value = serde_json::from_str(&raw_data)?;
            // Convert image saved as serde_json::Value to a vector of String.
            // This converted data will be cleaned later.
            let image_raw = value_vec_to_string_vec(json_data["image"].as_array().unwrap().clone());

            // Initialize a vector that will be returned as a result.
            let mut image = Vec::new();

            // Correct some escape characters and quotes.
            // Todo: Make an indenpendent function for this process.
            // https://users.rust-lang.org/t/escape-speech-marks-in-regex/4266/5
            let starting_double_quote_re = Regex::new(r#"^""#).unwrap();
            let ending_double_quote_re = Regex::new(r#""$"#).unwrap();
            let escaped_back_slash_re = Regex::new(r#"\\\\"#).unwrap();

            // Read each line.
            for line in image_raw {
                // Initialize an empty vector that will store cleaned lines.
                let mut temp_line = String::new();
                // Clean lines.
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

            // Push the image data to the vector of Cow.
            cows.push( Cow {
                name: json_data["name"].to_string(),
                max_text_length: get_max_text_len(image.clone()).unwrap(),
                image: image.clone(),
            });
        }
    }

    match cows.is_empty() {
        // If there's at least one image, return data.
        false => Ok(cows),
        // If there's no image, return Error.
        true => Err(Box::new(NoCowFound)),
    }
}

/// Generate an image.
pub fn generate_cow(cow: Cow, text: String) -> Result<String, Box<dyn Error>> {

    // If the text is longer than the textbox of the image, return Error.
    // Todo: Is it better to cut the text to fit it to the textbox?
    if cow.max_text_length < text.len() {
        return Err(Box::new(TextTooLong));
    }

    // Create the Regex instance for replace the textbox with the text.
    let text_box_re = Regex::new(r"BEGIN\s+END").unwrap();
    // Initialize a vector to store the image with the text.
    let mut image_vec: Vec<String> = Vec::new();

    // Read each line of the image.
    for line in cow.image.iter() {
        // If there is the textbox, replace the textbox with the text.
        // Todo: It seems to me that this if statement is useless.
        if let Some(_) = text_box_re.captures(&line) {
            // Replace the textbox with the text.
            // Note that you cannot simply specify new text(String).
            // https://qiita.com/scivola/items/60141f262caa53983c3a
            let replaced_line = text_box_re.replace(&line,
                |_caps: &Captures| {
                    align_string_with_ws(text.clone(), cow.max_text_length, Position::Center).unwrap()
                }
            );
            image_vec.push(replaced_line.to_string());
        } else {
            // If there's no textbox, push the line without editing.
            image_vec.push(line.clone());
        }
        // Push new line code between lines.
        image_vec.push("\n".to_string());
    }
    
    // Return image as one String.
    return Ok(image_vec.into_iter().collect());
}

/// Note that the string will be trimmed before being positioned.
fn align_string_with_ws(string: String, length: usize, position: Position) -> Result<String, Box<dyn Error>> {
    // Initialize a String variable to store trimmed String.
    let mut trimmed_string = String::new();
    // Trim the String.
    trimmed_string = string.trim().to_string();
    // If the trimmed String is longer than the length of the entire String that will be returned,
    // return Error.
    if trimmed_string.len() > length {
        return Err(Box::new(LengthShorterThanString));
    }
    
    // Align the String with whitespaces.
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
            
            // If the number of whitespaces, needed to be inserted, is odd, 
            match left_right_ws % 2 {
                0 => {
                    left_ws = left_right_ws / 2;
                    right_ws = left_right_ws / 2;
                },
                1 => {
                    // insert one more whitespace to the right side.
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

/// Generate a random image.
pub fn milk_random_cow(cows: Vec<Cow>, string: &String) -> Result<String, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let cow_num: usize = cows.len();

    return generate_cow(cows[rng.gen_range(0, cow_num)].clone(), string.clone());
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

