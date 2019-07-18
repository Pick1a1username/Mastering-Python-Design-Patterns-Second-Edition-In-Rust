use std::fs;
use std::io;
use std::error::Error;
use std::fmt;
use std::io::{Write};
use std::path::Path;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, App};
use flexi_logger::{Duplicate,Logger};
use log::info;


#[derive(Debug)]
struct Undoable;

impl Error for Undoable {}

impl fmt::Display for Undoable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait Command {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
    fn can_be_undo(&self) -> bool;
    fn undo(&self) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
struct CreateFile<'a> {
    undo: bool,
    // https://stackoverflow.com/questions/35296336/e0277-sized-is-not-implemented-for-the-type-u8-but-my-type-does-not-have-a
    path: &'a Path,
    text: String,
}

impl<'a> CreateFile<'a> {
    // Note that, as opposed to the original code, text parameter should be specified.
    fn new(path: &Path, text: String) -> CreateFile {
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
struct RenameFile<'a> {
    undo: bool,
    src: &'a Path,
    dest: &'a Path,
}

impl<'a> RenameFile<'a> {
    // I have no idea why lifetime parameters are needed in this function as opposed to new() in CreateFile.
    fn new(src: &'a Path, dest: &'a Path) -> RenameFile<'a> {
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
struct ReadFile<'a> {
    undo: bool,
    path: &'a Path,
}


impl<'a> ReadFile<'a> {
    fn new(path: &Path) -> ReadFile {
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

fn main() {

    // Set and prase arguments.
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .value_name("VERBOSE")
             .takes_value(false)
             .help("Show logs"))
        .get_matches();

    // Print logs if '--verbose' is passed.
    if matches.occurrences_of("verbose") > 0 {
        Logger::with_str("info")
            .duplicate_to_stderr(Duplicate::Warn)
            .start()
            .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
    }

    // The following functions are for unit test.
    //
    // test_create_file(true);
    // test_read_file();
    // test_rename_file();
    // test_delete_file();

    // Set file names.
    let (orig_name, new_name) = (Path::new("file1"), Path::new("file2"));

    // Define commands
    let mut commands: Vec<Box<dyn Command>> = Vec::new();
    commands.push(Box::new(CreateFile::new(orig_name, "Design Pattern is difficult...".to_string())));
    commands.push(Box::new(ReadFile::new(orig_name)));
    commands.push(Box::new(RenameFile::new(orig_name, new_name)));

    // Execute commands.
    for c in commands.iter() {
        match c.execute() {
            Ok(_) => { continue; },
            Err(e) => { info!("Something went wrong: {:?}", e); },
        }
    }

    // Execute undo reversely if the user wants.
    let mut answer = String::new();
    print!("reverse the executed commands? [y/n] ");   
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut answer)
        .expect("Failed to read line");
    let answer = String::from(answer.trim_end_matches("\n"));

    if answer == "y" {
        commands.reverse();

        for c in commands.iter() {
            match c.undo() {
                Ok(_) => { continue; },
                Err(e) => { info!("Something went wrong: {:?}", e); },
            }
        }
    } else {
        info!("the result is '{}'", new_name.to_str().unwrap());
    }

    info!("finished");
}


fn test_create_file(undo: bool) {
    println!("Testing CreateFile...");

    let path = Path::new("test_create_file.txt");
    let create_file = CreateFile::new(&path, "aaa".to_string());

    create_file.execute().unwrap();
        
    if undo == true {
        create_file.undo().unwrap();
    }
}

fn test_read_file() {
    println!("Testing ReadFile...");

    let path = Path::new("test.txt");
    let read_file = ReadFile::new(path);
    read_file.execute().unwrap();
}

fn test_rename_file() {
    println!("Testing RenameFile...");

    let src_path = Path::new("test.txt");
    let dest_path = Path::new("test_renamed.txt");
    let rename_file = RenameFile::new(src_path, dest_path);
    rename_file.execute().unwrap();
}

// Note: Make sure that the file will be deleted exists.
fn test_delete_file() {
    println!("Testing delete_file()...");

    let path = Path::new("test_will_be_deleted.txt");
    delete_file(path).unwrap();
}
