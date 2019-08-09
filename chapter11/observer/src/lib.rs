use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;


#[derive(Debug)]
pub struct AlreadyExist;

impl Error for AlreadyExist {}

impl fmt::Display for AlreadyExist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct NotExist;

impl Error for NotExist {}

impl fmt::Display for NotExist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub trait Publisher {
    fn add(&mut self, observer: Box<dyn Observer>) -> Result<(), Box<dyn Error>>;
    fn remove(&mut self, observer: String) -> Result<(), Box<dyn Error>>;
    fn notify(&self);
    fn get_info(&self) -> String;
    fn get_data(&self) -> i32;
    fn set_data(&mut self, new_data: i32);
}

pub struct DefaultFormatter {
    observers: Vec<Box<dyn Observer>>,
    name: String,
    data: i32,
}

impl DefaultFormatter {
    pub fn new(name: String) -> Box<dyn Publisher> {
        Box::new(DefaultFormatter {
            observers: Vec::new(),
            name: name,
            data: 0,
        })
    }
}

impl Publisher for DefaultFormatter {
    fn add(&mut self, observer: Box<dyn Observer>) -> Result<(), Box<dyn Error>> {
        // If there's a observer whose name is same as new observer, return error.
        for obs in self.observers.iter() {
            if obs.get_name() == observer.get_name() {
                return Err(Box::new(AlreadyExist));
            }
        }
        self.observers.push(observer);
        return Ok(());
    }

    fn remove(&mut self, observer_name: String) -> Result<(), Box<dyn Error>> {
        let mut index_to_remove: Option<usize> = None;

        for (index, obs) in self.observers.iter().enumerate() {
            if obs.get_name() == observer_name {
                index_to_remove = Some(index);
                break;
            }
        }

        match index_to_remove {
            Some(index) => {
                self.observers.remove(index);
                return Ok(());
            },
            _ => {
                return Err(Box::new(NotExist));
            },
        }
    }

    fn notify(&self) {
        for obs in self.observers.iter() {
            obs.notify(&self.name, &self.data);
        }
    }

    fn get_info(&self) -> String {
        return String::from(format!("DefaultFormatter: '{}' has data = {}", &self.name, &self.data));
    }

    fn get_data(&self) -> i32 {
        return self.data;
    }

    fn set_data(&mut self, new_data: i32) {
        self.data = new_data;
        self.notify();
    }
}

pub trait Observer {
    fn get_name(&self) -> String;
    fn notify(&self, publisher_name: &String, data: &i32);
}

pub struct HexFormatterObs {
    name: String,
}

impl HexFormatterObs {
    pub fn new() -> HexFormatterObs {
        HexFormatterObs { name: "HexFormatterObs".to_string() }
    }
}

impl Observer for HexFormatterObs {
    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn notify(&self, publisher_name: &String, data: &i32) {
        println!("{}: '{}' has now hex data = {:#x}",
            self.name,
            publisher_name,
            data
        );
    }
}

pub struct BinaryFormatterObs {
    name: String,
}

impl BinaryFormatterObs {
    pub fn new() -> BinaryFormatterObs {
        BinaryFormatterObs { name: "BinaryFormatterObs".to_string() }
    }
}

impl Observer for BinaryFormatterObs {
    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn notify(&self, publisher_name: &String, data: &i32) {
        println!("{}: '{}' has now bin data = {:#b}",
            self.name,
            publisher_name,
            data
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
