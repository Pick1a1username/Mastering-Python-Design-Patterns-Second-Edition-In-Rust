use std::fmt;
use std::io;
use std::io::Write;
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
    // fn new(&self) -> Self;
    fn prepare_dough(&mut self);
    fn add_sauce(&mut self);
    fn add_topping(&mut self);
    fn bake(&mut self);
    fn get_pizza_name(&self) -> &String;
}

struct MargaritaBuilder {
    pizza: Pizza,
    progress: PizzaProgress,
    baking_time: u64,
}

impl MargaritaBuilder {
    fn new() -> Self {
        MargaritaBuilder {
            pizza: Pizza::new(String::from("margarita")),
            progress: PizzaProgress::Queued,
            baking_time: 5,
        }
    }
}

impl Builder for MargaritaBuilder {
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

    fn get_pizza_name(&self) -> &String { self.pizza.get_name() }
}


struct Waiter<'a> {
    // https://users.rust-lang.org/t/box-lifetime-problem/9030
    builder: Option<Box<dyn Builder + 'a>>,
}

// trait Worker {}
// trait Worker: Sized {}


// impl<'a> Waiter<'a> {
//     fn new() -> Box<dyn Worker + 'a> {
//         Box::new(Waiter { builder: None })
//     }
// }
// 

// impl<'a> Worker for Waiter<'a> {
impl<'a> Waiter<'a> {
    // If lifetime annotation is not used in this function, compiler will complain.
    fn construct_pizza<T: Builder + 'a>(&mut self, builder: Box<T>) {
        self.builder = Some(builder);

        if let Some(b) = self.builder.as_mut() { b.prepare_dough() };
        if let Some(b) = self.builder.as_mut() { b.add_sauce() };
        if let Some(b) = self.builder.as_mut() { b.add_topping() };
        if let Some(b) = self.builder.as_mut() { b.bake() };
    }

    fn get_pizza_name(&self) -> Option<&String> {
        match self.builder.as_ref() {
            Some(b) => Some(b.get_pizza_name()),
            None => None,
        }
    }
}

fn validate_style() -> String {
    let mut valid_input =false;
    let mut pizza = String::new();

     while valid_input == false {
         print!("What pizza would you like, [m]argarita or [c]reamy bacon? ");
         io::stdout().flush().unwrap();
         if let Ok(_) = io::stdin().read_line(&mut pizza) {
             valid_input = true;
         } else {
             println!("Failed to read character");
         }
     }
     String::from(pizza.trim_end_matches("\n"))
}


fn deploy_waiter(order: String) -> Option<Box<dyn Builder>> {
    if order == String::from("m") {
        Some(Box::new(MargaritaBuilder::new()))
    } else {
        println!("Sorry, only margarita (key m) and creamy bacon (key c) are available");
        None
    }
}


fn main() {
    // let waiter = Box::new(Waiter { builder: None });
    let mut waiter = Waiter { builder: None };
    let order = validate_style();
    let builder: Box<dyn Builder> = deploy_waiter(order).unwrap();
    waiter.construct_pizza(builder);
    let pizza = waiter.get_pizza_name().unwrap();
    println!("Enjoy your {}!", pizza);
    
}
