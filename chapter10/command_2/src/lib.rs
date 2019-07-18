use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

use log::info;


pub enum Answer {
    SUCCEED,
    Content(String),
}

#[derive(Debug)]
struct Undoable;

impl Error for Undoable {}

impl fmt::Display for Undoable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Command {
    fn execute(&self) -> Result<Answer, Box<dyn Error>>;
    fn can_be_undo(&self) -> bool;
    fn undo(&self) -> Result<Answer, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct CreateFile<'a> {
    undo: bool,
    // https://stackoverflow.com/questions/35296336/e0277-sized-is-not-implemented-for-the-type-u8-but-my-type-does-not-have-a
    path: &'a Path,
    text: String,
}

impl<'a> CreateFile<'a> {
    // Note that, as opposed to the original code, text parameter should be specified.
    pub fn new(path: &Path, text: String) -> CreateFile {
        CreateFile {
            undo: true,
            path: path,
            text: text,
        }
    }
}

impl<'a> Command for CreateFile<'a> {
    fn can_be_undo(&self) -> bool {
        self.undo
    }

    /// Todo: Add a feature checking whether the file already exists or not.
    fn execute(&self) -> Result<Answer, Box<dyn Error>> {
        info!("creating file '{}'", self.path.to_str().unwrap());
        fs::write(self.path, &self.text)?;
        Ok(Answer::SUCCEED)
    }

    fn undo(&self) -> Result<Answer, Box<dyn Error>> {
        info!("removing file '{}'", self.path.to_str().unwrap());
        match self.can_be_undo() {
            true => {
                delete_file(self.path)?;
                Ok(Answer::SUCCEED)
            },
            false => {
                Err(Box::new(Undoable))
            },
        }
    }
}


#[derive(Debug)]
pub struct RenameFile<'a> {
    undo: bool,
    src: &'a Path,
    dest: &'a Path,
}

impl<'a> RenameFile<'a> {
    // I have no idea why lifetime parameters are needed in this function as opposed to new() in CreateFile.
    pub fn new(src: &'a Path, dest: &'a Path) -> RenameFile<'a> {
        RenameFile {
            undo: true,
            src: src,
            dest: dest,
        }
    }
}

impl<'a> Command for RenameFile<'a> {
    fn can_be_undo(&self) -> bool {
        self.undo
    }

    fn execute(&self) -> Result<Answer, Box<dyn Error>> {
        info!("renaming '{}' to '{}'", self.src.to_str().unwrap(), self.dest.to_str().unwrap());
        fs::rename(self.src, self.dest)?;
        Ok(Answer::SUCCEED)
    }

    fn undo(&self) -> Result<Answer, Box<dyn Error>> {
        info!("renaming '{}' to '{}'", self.dest.to_str().unwrap(), self.src.to_str().unwrap());
        match self.can_be_undo() {
            true => {
                fs::rename(self.dest, self.src)?;
                Ok(Answer::SUCCEED)
            },
            false => {
                Err(Box::new(Undoable))
            }
        }
    }
}


#[derive(Debug)]
pub struct ReadFile<'a> {
    undo: bool,
    path: &'a Path,
}


impl<'a> ReadFile<'a> {
    pub fn new(path: &Path) -> ReadFile {
        ReadFile {
            undo: false,
            path: path,
        }
    }
}

impl<'a> Command for ReadFile<'a> {
    fn can_be_undo(&self) -> bool {
        self.undo
    }

    fn execute(&self) -> Result<Answer, Box<dyn Error>> {
        info!("reading file '{}'", self.path.to_str().unwrap());
        let contents = fs::read_to_string(self.path)?;
        println!("{}", contents);
        // The content is returned as well for test automation.
        Ok(Answer::Content(contents))
    }

    fn undo(&self) -> Result<Answer, Box<dyn Error>> {
        Err(Box::new(Undoable))
    }
}

fn delete_file(path: &Path) -> Result<Answer, Box<dyn Error>> {
    fs::remove_file(path)?;
    Ok(Answer::SUCCEED)
}


// Work in Progress
#[cfg(test)]
mod tests {

    // use std::path::Path;
    use super::*;

    #[test]
    fn struct_create_file() {
        let contents_expected = String::from("test content");
        let path = Path::new("test_create_file.txt");
        let create_file = CreateFile::new(&path, contents_expected.clone());

        // Create the file
        create_file.execute().unwrap();
            
        // Verify the file created.
        assert_eq!(fs::metadata(&path).unwrap().is_file(), true);
        
        // Verify the content of the file.
        match ReadFile::new(path).execute().unwrap() {
            Answer::Content(contents_result) => {
                assert_eq!(contents_result, contents_expected);
            },
            _ => {
                assert!(false);
            },
        }

        // Undo creating the file.
        create_file.undo().unwrap();

        // Verify the file deleted.
        assert_eq!(fs::metadata(&path).is_err(), true);

    }

    #[test]
    fn struct_read_file() {
        let contents_expected = String::from("test content");
        let path = Path::new("test_read_file.txt");
        let create_file = CreateFile::new(&path, contents_expected.clone());

        // Create the file
        create_file.execute().unwrap();
            
        // Verify the content of the file.
        match ReadFile::new(path).execute().unwrap() {
            Answer::Content(contents_result) => {
                assert_eq!(contents_result, contents_expected);
            },
            _ => {
                assert!(false);
            },
        }

        // Undo creating the file.
        create_file.undo().unwrap();
    }

    #[test]
    fn struct_rename_file() {

        let src_path = Path::new("test_before.txt");
        let dest_path = Path::new("test_after.txt");

        // Before the test, make sure that the file whose name is same as dest_path doesn't exist.
        assert_eq!(fs::metadata(&dest_path).is_err(), true);

        // Create the file
        let create_file = CreateFile::new(&src_path, "aaa".to_string());
        create_file.execute().unwrap();

        // Rename the file
        let rename_file = RenameFile::new(src_path, dest_path);
        rename_file.execute().unwrap();

        // Verify the file's name is changed.
        assert_eq!(fs::metadata(&dest_path).unwrap().is_file(), true);

        // Undo renaming the file.
        rename_file.undo().unwrap();

        // Verify renaming is undone.
        assert_eq!(fs::metadata(&src_path).unwrap().is_file(), true);

        // Delete the file
        create_file.undo().unwrap();
    }

    #[test]
    fn fn_delete_file() {

        let path = Path::new("test_delete_file.txt");
        
        // Create the file
        let create_file = CreateFile::new(&path, "aaa".to_string());
        create_file.execute().unwrap();

        // Delete the file
        delete_file(path).unwrap();

        // Verify the file is deleted.
        assert_eq!(fs::metadata(&path).is_err(), true);
    }
}

