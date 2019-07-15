use std::path::Path;
use std::io::{Error, ErrorKind};
use std::fs;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, App};
use flexi_logger::{Duplicate,Logger};
use log::info;


trait Command {
    fn execute(&self) -> Result<(), Error>;
    fn can_be_undo(&self) -> bool;
}

trait Undo {
    fn undo(&self) -> Result<(), Error>;
}

struct CreateFile<'a> {
    undo: bool,
    // https://stackoverflow.com/questions/35296336/e0277-sized-is-not-implemented-for-the-type-u8-but-my-type-does-not-have-a
    path: &'a Path,
    text: String,
}

impl<'a> CreateFile<'a> {
    fn new(path: &Path, text: String) -> CreateFile {
        CreateFile {
            undo: true,
            path: &path,
            text: text,
        }
    }
}

impl<'a> Command for CreateFile<'a> {
    fn can_be_undo(&self) -> bool {
        self.undo
    }

    /// Todo: Add a feature checking whether the file already exists or not.
    fn execute(&self) -> Result<(), Error> {
        info!("creating file {}", self.path.to_str().unwrap());
        fs::write(self.path, &self.text)?;
        Ok(())
    }
}

impl<'a> Undo for CreateFile<'a> {
    fn undo(&self) -> Result<(), Error> {
        info!("removing file {}", self.path.to_str().unwrap());
        fs::remove_file(self.path)?;
        Ok(())
    }
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

    test_create_file(true);
}


fn test_create_file(undo: bool) {
    println!("Testing CreateFile...");

    let path = Path::new("test_create_file.txt");
    let create_file = CreateFile::new(&path, "aaa".to_string());

    create_file.execute();
    if undo == true {
        create_file.undo();
    }
}
