pub struct Musician {
    name: String,
}

impl Musician {
    pub fn new(name: String) -> Musician {
        Musician { name: name }
    }

    pub fn play(&self) -> String {
        String::from("plays music")
    }

    pub fn is_called(&self) -> String {
        String::from(format!("the musician {}", &self.name))
    }
}

pub struct Dancer {
    name: String,
}

impl Dancer {
    pub fn new(name: String) -> Dancer {
        Dancer { name: name }
    }

    pub fn dance(&self) -> String {
        String::from("does a dance performance")
    }

    pub fn is_called(&self) -> String {
        String::from(format!("the dancer {}", &self.name))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
