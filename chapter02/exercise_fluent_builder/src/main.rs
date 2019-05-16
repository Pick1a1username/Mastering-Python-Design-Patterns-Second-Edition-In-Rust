struct Pizza {
    garlic: bool,
    extra_cheese: bool,
}

impl Pizza {
    fn new(builder: PizzaBuilder) -> Pizza {
        Pizza {
            garlic: builder.garlic,
            extra_cheese: builder.extra_cheese,
        }
    }

    fn get_info(&self) -> String {
        let garlic = {
            if self.garlic == true {
                "yes"
            } else {
                "no"
            }
        };

        let cheese = {
            if self.extra_cheese == true {
                "yes"
            } else {
                "no"
            }
        };

        let info = format!("Garlic: {}\nExtra cheese: {}",
            garlic,
            cheese
        );
        info
    }
}

struct PizzaBuilder {
    garlic: bool,
    extra_cheese: bool,
}

impl PizzaBuilder {
    fn new() -> PizzaBuilder {
        PizzaBuilder {
            garlic: false,
            extra_cheese: false,
        }
    }

    fn add_garlic(mut self) -> Self{
        self.garlic = true;
        self
    }

    fn add_extra_cheese(mut self) -> Self{
        self.extra_cheese = true;
        self
    }

    fn build(self) -> Pizza {
        Pizza::new(self)
    }
}

fn main() {
    let pizza = PizzaBuilder::new().add_garlic().add_extra_cheese().build();
    println!("{}", pizza.get_info());
}
