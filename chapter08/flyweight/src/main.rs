use rand::Rng;
use std::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


/// # Traits
/// 
/// * Debug: To print Cartype easily
/// * PartialEq: To compare CarType
/// * Eq: To compare CarType
/// * Hash: To use CarType as value in HashMap
/// * Clone: To use CarType in the same scope multiple times. Refer to Car.new().
/// * Copy: To use CarType in the same scope multiple times. Refer to Car.new().
///
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum CarType {
    SubCompact,
    Compact,
    Suv,
    Nothing, // Only for Car::init()
}

/// To print car type easily
impl fmt::Display for CarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// # Traits
/// 
/// * PartialEq: To compare Car
/// * Eq: To compare Car
///
#[derive(PartialEq, Eq)]
struct Car {
    /// The type of pool should be Rc because:
    /// 
    /// * all of instances of Car must share same pool.
    ///
    /// The RefCell also should be used with Rc because:
    /// 
    /// * pool should be able to be updated when new CarType is appeared.
    ///  
    /// The value of HashMap of pool, Car, should be Rc because:
    /// 
    /// * If there are multiple instances of Car whose car_type are same, there should be only one really instance. The other is nothing but Rc cloned.
    /// * So when instantiating a Car, if the car type already exists, a cloned Car saved in the pool will be returned.
    /// 
    pool: Rc<RefCell<HashMap<CarType, Rc<Car>>>>,
    car_type: CarType,
}

impl Car {
    /// Add new car type.
    fn new(&self, car_type: CarType) -> Rc<Car> {
        match self.pool.borrow().contains_key(&car_type) {
            true => {
               Rc::clone(self.pool.borrow().get(&car_type).unwrap())
            },
            false => {
                let new_car = Rc::new(
                    Car {
                        pool: Rc::clone(&self.pool),
                        car_type: car_type,
                    }
                );
                self.pool.borrow_mut().insert(
                    car_type,
                    Rc::clone(&new_car)
                );
                
                Rc::clone(&new_car)
            }
        }
    }

    /// Additional function not in the Python code.
    ///
    /// Struct cannot have any variable in Rust as opposed to Class in Python.
    fn init() -> Rc<Car> {
        let new_pool: HashMap<CarType, Rc<Car>> = HashMap::new();
        let new_car = Rc::new(
            Car {
                pool: Rc::new(RefCell::new(new_pool)),
                // When Car is initialized, no car type specified.
                // Since Car.car_type's type is CarType, any Variant should be specified. And this is why 'Nothing' Variant exists.
                car_type: CarType::Nothing,
            }
        );
        Rc::clone(&new_car)
    }

    fn render(&self, color: String, x: i32, y: i32) {
        println!("render a car of type {}, and color {} at ({}, {})",
            self.car_type,
            color,
            x,
            y
        );
    }     
}

/// To get address of a instance
impl fmt::Pointer for Car {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:p}", self as *const Car)
    }
}

/// rand crate has no feature to select an element randomly in array as opposed to random module in Python.
/// So creating a function to select a color is much more clear.
fn color_generator() -> String {
    let colors = ["white","black","silver","gray","red","blue","brown","beige","yellow","green"];
    let (min, max) = (0, colors.len());

    let mut rng = rand::thread_rng();

    colors[rng.gen_range(min, max)].to_string()
}
    
/// This function is almost same as Python code.
fn main() {
    let mut rng = rand::thread_rng();
    let (min_point, max_point) = (0, 100);
    let mut car_counter = 0;

    let car_generator = Car::init();

    for _ in 0..10 {
        let c1 = car_generator.new(CarType::SubCompact);
        c1.render(
            color_generator(),
            rng.gen_range(min_point, max_point),
            rng.gen_range(min_point, max_point)
        );
        car_counter += 1;
    }

    for _ in 0..3 {
        let c2 = car_generator.new(CarType::Compact);
        c2.render(
            color_generator(),
            rng.gen_range(min_point, max_point),
            rng.gen_range(min_point, max_point)
        );
        car_counter += 1;
    }

    for _ in 0..5 {
        let c3 = car_generator.new(CarType::Suv);
        c3.render(
            color_generator(),
            rng.gen_range(min_point, max_point),
            rng.gen_range(min_point, max_point)
        );
        car_counter += 1;
    }

    println!("cars rendered: {}", car_counter);
    println!("cars actually created: {}", car_generator.pool.borrow().len());

    let c4 = car_generator.new(CarType::SubCompact);
    let c5 = car_generator.new(CarType::SubCompact);
    let c6 = car_generator.new(CarType::Suv);

    println!("{:p} == {:p}? {}", c4, c5, &c4 == &c5);
    println!("{:p} == {:p}? {}", c5, c6, &c5 == &c6);
}
