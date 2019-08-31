use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

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

#[derive(Clone)]
struct Cow {
    name: String,
    max_text_length: usize,
    image: Vec<String>,
}

#[derive(Debug)]
struct NoCowFound;

impl Error for NoCowFound {}

impl fmt::Display for NoCowFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn load_cows_from_files(path: &Path) -> Result<Vec<Cow>, Box<dyn Error>> {
    let mut cows: Vec<Cow> = Vec::new();
    for data in path.read_dir()? {
        if let Ok(cow_path) = data {
            let raw_data = fs::read_to_string(cow_path.path())?;
            let json_data: Value = serde_json::from_str(&raw_data)?;
            let image = value_vec_to_string_vec(json_data["image"].as_array().unwrap().clone());
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

fn generate_cow(cow: Cow, text: String) -> Result<String, Box<dyn Error>> {

    if cow.max_text_length < text.len() {
        return Err(Box::new(TextTooLong));
    }

    let text_box_re = Regex::new(r"BEGIN\s+END").unwrap();
    let mut image_vec: Vec<String> = Vec::new();

    for line in cow.image.iter() {
        if let Some(_) = text_box_re.captures(&line) {
            // https://qiita.com/scivola/items/60141f262caa53983c3a
            let replaced_line = text_box_re.replace(&line,
                |caps: &Captures| {
                    text.clone()
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

fn milk_random_cow(string: &String) -> String {
    unimplemented!();
}

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

    #[test]
    fn test_fn_load_cows_from_files() {
        let path = Path::new("cows/");
        match load_cows_from_files(&path) {
            Ok(_) => { println!("Ok") },
            Err(_) => { println!("Oops!") },
        }
        assert_eq!(1, 1);
    }

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

    #[test]
    fn test_fn_generate_cow() {
        let path = Path::new("cows/");
        let cows = load_cows_from_files(&path).unwrap();
        generate_cow(cows[0].clone(), "hello".to_string());
        assert_eq!(1, 1);
    }
}

