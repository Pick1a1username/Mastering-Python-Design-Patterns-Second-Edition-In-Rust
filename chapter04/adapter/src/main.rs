use adapter::{Musician,Dancer};

enum EventElement {
    Club(Club),
    Musician(Musician),
    Dancer(Dancer),
}

struct Club {
    name: String,
}

impl Club {
    fn new(name: String) -> Club {
        Club { name: name }
    }

    fn organize_event(&self) -> String {
        String::from("does a dance performance")
    }

    fn is_called(&self) -> String {
        String::from(format!("the club {}", &self.name))
    }
}

struct Adaptor {
    obj: EventElement,
}

impl Adaptor {
    fn new(obj: EventElement) -> Adaptor {
        Adaptor { obj: obj }
    }

    fn organize_event(&self) -> String {
        match &self.obj {
            EventElement::Club(club) => club.organize_event(),
            EventElement::Musician(musician) => musician.play(),
            EventElement::Dancer(dancer) => dancer.dance(),
        }
    }

    fn is_called(&self) -> String {
        match &self.obj {
            EventElement::Club(club) => club.is_called(),
            EventElement::Musician(musician) => musician.is_called(),
            EventElement::Dancer(dancer) => dancer.is_called(),
        }
    }

}

fn main() {
    // println!("Hello World!");
    let objects = vec![
        EventElement::Club(Club::new(String::from("Jazz Cafe"))),
        EventElement::Musician(Musician::new(String::from("Roy Ayers"))),
        EventElement::Dancer(Dancer::new(String::from("Shane Sparks"))),
    ];

    for obj in objects {
        let adapter = Adaptor::new(obj);

        println!("{} {}", adapter.is_called(), adapter.organize_event());
    }
}
