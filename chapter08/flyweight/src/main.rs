use std::fmt;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


extern crate rand;

#[derive(PartialEq, Eq, Hash)]
enum CarType {
    SubCompact,
    Compact,
    Suv,
}


struct Car {
    pool: Rc<RefCell<HashMap<CarType, Rc<Car>>>>,
}

impl Car {
    fn new(&self, car_type: CarType) -> Rc<Car> {
        match self.checker(&car_type) {
            true => {
               Rc::clone(self.pool.borrow().get(&car_type).unwrap())
            },
            false => {
                let new_car = Rc::new(Car { pool: Rc::clone(&self.pool) });
                self.pool.borrow_mut().insert(
                    car_type,
                    Rc::clone(&new_car)
            );
                Rc::clone(&new_car)
            }
        }
    }

    fn init() -> Rc<Car> {
        let mut new_pool: HashMap<CarType, Rc<Car>> = HashMap::new();
        let new_car = Rc::new(Car { pool: Rc::new(RefCell::new(new_pool)) });
        Rc::clone(&new_car)
    }

    fn checker(&self, car_type: &CarType) -> bool {
        self.pool.borrow().contains_key(&car_type)
    }
}

impl fmt::Pointer for Car {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use `as` to convert to a `*const T`, which implements Pointer, which we can use

        write!(f, "{:p}", self as *const Car)
    }
}

fn main() {
    let car_generator = Car::init();

    let car_a = car_generator.new(CarType::Suv);
    let car_b = car_generator.new(CarType::Suv);
    let car_c = car_generator.new(CarType::Compact);

    println!("{:p}", car_a);
    println!("{:p}", car_b);
    println!("{:p}", car_c);
}
