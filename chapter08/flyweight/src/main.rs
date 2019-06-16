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
    fn new(car_type: CarType) -> Rc<Car> {
        let mut new_pool: HashMap<CarType, Rc<Car>> = HashMap::new();
        let new_car = Rc::new(Car { pool: Rc::new(RefCell::new(new_pool)) });
        new_car.pool.borrow_mut().insert(
            car_type,
            Rc::clone(&new_car)
        );
        Rc::clone(&new_car)
    }
}

impl fmt::Pointer for Car {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use `as` to convert to a `*const T`, which implements Pointer, which we can use

        write!(f, "{:p}", self as *const Car)
    }
}

fn main() {
    let car_a = Car::new(CarType::Suv);
    let car_b = Car::new(CarType::Suv);
    let car_c = Car::new(CarType::Compact);

    println!("{:p}", car_a);
    println!("{:p}", car_b);
    println!("{:p}", car_c);
}
