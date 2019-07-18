use std::io;
use std::io::Write;
use std::path::Path;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, App};
use flexi_logger::{Duplicate,Logger};
use log::info;

// Import Traits, Structs and Functions from lib.rs
use command_2::{Command, CreateFile, ReadFile, RenameFile};

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

