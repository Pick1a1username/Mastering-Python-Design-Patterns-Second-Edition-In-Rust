pub mod cow_rs {

    use std::path::Path;
    use std::fs;
    use std::error::Error;

    pub struct Cow {
        name: String,
        max_text_length: u8,
        image: String,
    }

    pub fn load_cows_from_files(path: &Path) -> Result<(), Box<dyn Error>> {
        let mut cows: Vec<Cow> = Vec::new();
        for data in path.read_dir()? {
            if let Ok(cow_path) = data {
                let data = fs::read_to_string(cow_path.path())?;
                println!("{}", data);
            }
        }

        Ok(())
    }

    fn generate_cow(cow: Cow, text: String) -> String {
        unimplemented!();
    }

    fn milk_random_cow(string: &String) -> String {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn fn_load_cows_from_files() {
        let path = Path::new("cows/");
        cow_rs::load_cows_from_files(&path);
        assert_eq!(1, 1);
    }
}

