use std::fs;
use std::error::Error;
use std::fmt;
use std::path::Path;

use log::info;


#[derive(Debug)]
struct Undoable;

impl Error for Undoable {}

impl fmt::Display for Undoable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
    fn can_be_undo(&self) -> bool;
    fn undo(&self) -> Result<(), Box<dyn Error>>;
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
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        info!("creating file '{}'", self.path.to_str().unwrap());
        fs::write(self.path, &self.text)?;
        Ok(())
    }

    fn undo(&self) -> Result<(), Box<dyn Error>> {
        info!("removing file '{}'", self.path.to_str().unwrap());
        match self.can_be_undo() {
            true => {
                delete_file(self.path)?;
                Ok(())
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

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        info!("renaming '{}' to '{}'", self.src.to_str().unwrap(), self.dest.to_str().unwrap());
        fs::rename(self.src, self.dest)?;
        Ok(())
    }

    fn undo(&self) -> Result<(), Box<dyn Error>> {
        info!("renaming '{}' to '{}'", self.dest.to_str().unwrap(), self.src.to_str().unwrap());
        match self.can_be_undo() {
            true => {
                fs::rename(self.dest, self.src)?;
                Ok(())
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

    fn execute(&self) -> Result<(), Box<dyn Error>> {
        info!("reading file '{}'", self.path.to_str().unwrap());
        let contents = fs::read_to_string(self.path)?;
        println!("{}", contents);
        Ok(())
    }

    fn undo(&self) -> Result<(), Box<dyn Error>> {
        Err(Box::new(Undoable))
    }
}

fn delete_file(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::remove_file(path)?;
    Ok(())
}


// Work in Progress
#[cfg(test)]
mod tests {

    // Work in Progress
    #[test]
    fn create_file() {

        let path = Path::new("test_create_file.txt");
        let create_file = CreateFile::new(&path, "aaa".to_string());

        create_file.execute().unwrap();
            
        // Verify the file created.

        if undo == true {
            create_file.undo().unwrap();
        }

        // Verify the file deleted.

    }

    // Work in Progress
    #[test]
    fn read_file() {

        let path = Path::new("test.txt");
        let read_file = ReadFile::new(path);
        read_file.execute().unwrap();

        // Verify the content is same as expected.

    }

    // Work in Progress
    #[test]
    fn rename_file() {


        let src_path = Path::new("test.txt");
        let dest_path = Path::new("test_renamed.txt");
        let rename_file = RenameFile::new(src_path, dest_path);
        rename_file.execute().unwrap();

        
    }

    // Work in Progress
    #[test]
    fn delete_file() {

        let path = Path::new("test_will_be_deleted.txt");
        delete_file(path).unwrap();
    }
}

