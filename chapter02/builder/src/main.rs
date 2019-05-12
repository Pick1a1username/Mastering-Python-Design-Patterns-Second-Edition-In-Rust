use std::fmt;
use std::thread;
use std::time::Duration;



const STEP_DELAY: u64 = 3;  // in seconds for the sake of the example

enum PizzaProgress {
    Queued,
    Preparation,
    Baking,
    Ready,
}

enum PizzaDough {
    Thin,
    Thick,
}

enum PizzaSauce {
    Tomato,
    CremeFraiche,
}

enum PizzaTopping {
    Mozzarella,
    DoubleMozzarella,
    Bacon,
    Ham,
    Mushrooms,
    RedOnion,
    Oregano,
}

// https://stackoverflow.com/questions/28024373/is-there-a-way-to-print-enum-values
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html?highlight=Fmt#using-the-newtype-pattern-to-implement-external-traits-on-external-types
impl fmt::Display for PizzaDough {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PizzaDough::Thin => write!(f, "Thin"),
            PizzaDough::Thick => write!(f, "Thick"),
        }
    }
}

impl fmt::Display for PizzaTopping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PizzaTopping::Mozzarella => write!(f, "Mozzarella"),
            PizzaTopping::DoubleMozzarella => write!(f, "Double Mozzarella"),
            PizzaTopping::Bacon => write!(f, "Bacon"),
            PizzaTopping::Ham => write!(f, "Ham"),
            PizzaTopping::Mushrooms => write!(f, "Mushrooms"),
            PizzaTopping::RedOnion => write!(f, "Red Onion"),
            PizzaTopping::Oregano => write!(f, "Oregano"),
        }
    }
}

struct Pizza {
    name: String,
    dough: Option<PizzaDough>,
    sauce: Option<PizzaSauce>,
    topping: Vec<PizzaTopping>,
}

impl Pizza {
    fn new(name: String) -> Pizza {
        Pizza {
            name: name,
            dough: None,
            sauce: None,
            topping: Vec::new(),
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn prepare_dough(&mut self, dough: PizzaDough) {
        self.dough = Some(dough);

        // Option in the Struct cannot be simply unwrapped.
        // https://stackoverflow.com/questions/47570580/is-there-a-shortcut-for-as-ref-unwrap
        let dough_name = self.dough.as_ref().unwrap();
        
        println!("preparing the {} dough of your {}...",
            dough_name,
            self.name,
        );
        thread::sleep(Duration::from_millis(STEP_DELAY));
        println!("done with the {} dough", dough_name);
    }

    fn set_sauce(&mut self, sauce: PizzaSauce) {
        self.sauce = Some(sauce);
    }
}

trait Builder {
    fn new(&self) -> Self;
    fn prepare_dough(&mut self);
    fn add_sauce(&mut self);
    fn add_topping(&mut self);
    fn bake(&mut self);
}

struct MargaritaBuilder {
    pizza: Pizza,
    progress: PizzaProgress,
    baking_time: u64,
}

impl Builder for MargaritaBuilder {
    fn new(&self) -> Self {
        MargaritaBuilder {
            pizza: Pizza::new(String::from("margarita")),
            progress: PizzaProgress::Queued,
            baking_time: 5,
        }
    }

    fn prepare_dough(&mut self) {
        self.progress = PizzaProgress::Preparation;
        self.pizza.prepare_dough(PizzaDough::Thin);
    }

    fn add_sauce(&mut self) {
        println!("adding the tomato sauce to your margarita...");
        self.pizza.set_sauce(PizzaSauce::Tomato);
    }

    fn add_topping(&mut self) {
        let topping_desc = String::from("double mozzarella, oregano");
        let topping_items = vec![PizzaTopping::DoubleMozzarella, PizzaTopping::Oregano];

        println!("adding the topping ({}) to your margarita", topping_desc);

        for t in topping_items.into_iter() {
            self.pizza.topping.push(t);
        }

        thread::sleep(Duration::from_millis(STEP_DELAY));

        println!("done with the topping ({})", topping_desc);
    }

    fn bake(&mut self) {
        self.progress = PizzaProgress::Baking;
        println!("baking your margarita for {} seconds", self.baking_time);
        thread::sleep(Duration::from_millis(self.baking_time));
        println!("your margarita is ready");
    }

}

fn main() {
    println!("Hello, world!");
}
