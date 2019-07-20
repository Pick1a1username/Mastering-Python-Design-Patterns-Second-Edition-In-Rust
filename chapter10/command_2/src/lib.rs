//! # Command Pattern (2)
//! 
//! This module is an example of command pattern based on the code written in Python.
//! The original Python code is [this](https://github.com/PacktPublishing/Mastering-Python-Design-Patterns-Second-Edition/blob/master/chapter10/command.py).
//!
//! This example has the following commands:
//!
//! * CreateFile: Create a file.
//! * ReadFile: Read a file's content.
//! * RenameFile: Rename a file.
//! 
//! Also, there is a function that doesn't need to be Struct:
//! 
//! * delete_file: Delete a file.
//! 
//! Most of codes are same as Python's one. But some parts are different because of:
//! 
//! * ease of test automation.
//! * difference of Rust and Python.
//! * improvements which can be easily implemented.
//! * using crates which I have learned from other books.
//! 

use std::error::Error; // For creating a custom error type.
use std::fmt; // For creating a custom error type.
use std::fs; // For handling files.
use std::path::Path; // For handling file's path easily.

use log::info; // For logging.

/// Command's result when it is executed successfully.
pub enum Answer {
    /// For a result when there is nothing to return.
    SUCCEED,
    /// For a result when there is something to return as String.
    Content(String), 
}

/// Error when a command cannot be undone.
#[derive(Debug)]
struct NotUndoable;

impl Error for NotUndoable {}

impl fmt::Display for NotUndoable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Execute a command and (optionally) undo a command.
pub trait Command {
    /// Execute a command.
    fn execute(&self) -> Result<Answer, Box<dyn Error>>;
    /// Return whether a command can be undone or not.
    fn can_be_undo(&self) -> bool;
    /// Undo a command.
    fn undo(&self) -> Result<Answer, Box<dyn Error>>;
}

/// Create a file.
/// 
/// This is NotUndoable.
#[derive(Debug)]
pub struct CreateFile<'a> {
    /// Whether the command can be undone or not.
    undo: bool,
    /// File's name and path.
    path: &'a Path,
    /// Text that will be written to the file.
    text: String,
}

impl<'a> CreateFile<'a> {
    /// Initialize a CreateFile command.
    /// 
    /// As opposed to the original code, text parameter should be specified.
    pub fn new(path: &Path, text: String) -> CreateFile {
        CreateFile {
            undo: true,
            path: path,
            text: text,
        }
    }
}

impl<'a> Command for CreateFile<'a> {
    /// Return whether the command can be undone or not.
    /// 
    /// * `true`: doable.
    /// * `false`: NotUndoable.
    fn can_be_undo(&self) -> bool {
        self.undo
    }

    /// Execute the command.
    /// 
    /// Todo: Add a feature checking whether the file already exists or not.
    fn execute(&self) -> Result<Answer, Box<dyn Error>> {
        info!("creating file '{}'", self.path.to_str().unwrap());
        fs::write(self.path, &self.text)?;
        Ok(Answer::SUCCEED)
    }

    /// Undo the command.
    fn undo(&self) -> Result<Answer, Box<dyn Error>> {
        info!("removing file '{}'", self.path.to_str().unwrap());
        match self.can_be_undo() {
            true => {
                delete_file(self.path)?;
                Ok(Answer::SUCCEED)
            },
            false => {
                Err(Box::new(NotUndoable))
            },
        }
    }
}

/// Rename a file.
/// 
/// This is NotUndoable.
#[derive(Debug)]
pub struct RenameFile<'a> {
    /// Whether the command can be undone or not.
    undo: bool,
    /// File's name and path that will be renamed.
    src: &'a Path,
    /// File's new name.
    dest: &'a Path,
}

impl<'a> RenameFile<'a> {
    /// Initialize a CreateFile command.
    ///
    /// I have no idea why lifetime parameters are needed in this function as opposed to new() in CreateFile.
    pub fn new(src: &'a Path, dest: &'a Path) -> RenameFile<'a> {
        RenameFile {
            undo: true,
            src: src,
            dest: dest,
        }
    }
}

impl<'a> Command for RenameFile<'a> {
    /// Return whether the command can be undone or not.
    /// 
    /// * `true`: doable.
    /// * `false`: NotUndoable.
    fn can_be_undo(&self) -> bool {
        self.undo
    }

    /// Execute the command.
    fn execute(&self) -> Result<Answer, Box<dyn Error>> {
        info!("renaming '{}' to '{}'", self.src.to_str().unwrap(), self.dest.to_str().unwrap());
        fs::rename(self.src, self.dest)?;
        Ok(Answer::SUCCEED)
    }

    /// Undo the command.
    fn undo(&self) -> Result<Answer, Box<dyn Error>> {
        info!("renaming '{}' to '{}'", self.dest.to_str().unwrap(), self.src.to_str().unwrap());
        match self.can_be_undo() {
            true => {
                fs::rename(self.dest, self.src)?;
                Ok(Answer::SUCCEED)
            },
            false => {
                Err(Box::new(NotUndoable))
            }
        }
    }
}

/// Read a file.
/// 
/// This is not NotUndoable.
#[derive(Debug)]
pub struct ReadFile<'a> {
    /// Whether the command can be undone or not.
    undo: bool,
    /// File's name and path.
    path: &'a Path,
}


impl<'a> ReadFile<'a> {
    /// Initialize a CreateFile command.
    pub fn new(path: &Path) -> ReadFile {
        ReadFile {
            undo: false,
            path: path,
        }
    }
}

impl<'a> Command for ReadFile<'a> {
    /// Return whether the command can be undone or not.
    /// 
    /// * `true`: doable.
    /// * `false`: NotUndoable.
    fn can_be_undo(&self) -> bool {
        self.undo
    }

    /// Execute the command.
    fn execute(&self) -> Result<Answer, Box<dyn Error>> {
        info!("reading file '{}'", self.path.to_str().unwrap());
        let contents = fs::read_to_string(self.path)?;
        println!("{}", contents);
        // The content is returned as well for test automation.
        Ok(Answer::Content(contents))
    }

    /// Undo the command.
    fn undo(&self) -> Result<Answer, Box<dyn Error>> {
        Err(Box::new(NotUndoable))
    }
}

/// Delete a file.
/// 
/// This is not a struct and not NotUndoable.
fn delete_file(path: &Path) -> Result<Answer, Box<dyn Error>> {
    fs::remove_file(path)?;
    Ok(Answer::SUCCEED)
}



#[cfg(test)]
mod tests {

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
            // Anythign else is unexpected, and the test is considered as fail.
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
            // Anythign else is unexpected, and the test is considered as fail.
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

