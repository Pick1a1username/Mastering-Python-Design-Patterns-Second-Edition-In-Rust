# Flyweight in Rust

This material shows how instances are generated in Rust.


## 1

This is before the code is executed.

```mermaid
graph LR
  Car["Car (Struct)"]
```

## 2

```rust
let car_generator = Car::init();
```

```mermaid
graph LR
  Car["Car (Struct)"]-. Instantiate .->CarGenerator["car_generator"]

  subgraph car_generator
    CarGenerator---CarGeneratorCarType["car_type </br> == Nothing"]
    CarGenerator---CarGeneratorPool["pool</br> { Nothing : car_generator } "]
    CarGeneratorPool-->CarGenerator
  end
```


## 3

```rust
for _ in 0..10 {
    let c1 = car_generator.new(CarType::SubCompact);
    c1.render(
        color_generator(),
        rng.gen_range(min_point, max_point),
        rng.gen_range(min_point, max_point)
    );
    car_counter += 1;
}
```

```mermaid
graph LR
  Car["Car (Struct)"]-. Instantiate .->CarGenerator["car_generator"]

  subgraph car_generator
    CarGenerator---CarGeneratorCarType["car_type </br> == Nothing"]
    CarGenerator---CarGeneratorPool["pool</br> { Nothing : car_generator,</br> SubCompact: c1 } "]
    CarGeneratorPool-->CarGenerator
  end

  CarGeneratorPool-->C1["c1"]
  subgraph c1
    C1---C1Pool["pool"]
    C1---C1CarType["car_type </br> == SubCompact"]
    C1Pool-->CarGeneratorPool
  end
```


## 4

```rust
for _ in 0..3 {
    let c2 = car_generator.new(CarType::Compact);
    c2.render(
        color_generator(),
        rng.gen_range(min_point, max_point),
        rng.gen_range(min_point, max_point)
    );
    car_counter += 1;
}
```

```mermaid
graph LR
  Car["Car (Struct)"]-. Instantiate .->CarGenerator["car_generator"]

  subgraph car_generator
    CarGenerator---CarGeneratorCarType["car_type </br> == Nothing"]
    CarGenerator---CarGeneratorPool["pool</br> { Nothing : car_generator, </br> SubCompact: c1 </br> Compact: c2 } "]
    CarGeneratorPool-->CarGenerator
  end

  CarGeneratorPool-->C1["c1"]
  subgraph c1
    C1---C1Pool["pool"]
    C1---C1CarType["car_type </br> == SubCompact"]
    C1Pool-->CarGeneratorPool
  end

  CarGeneratorPool-->C2["c2"]
  subgraph c2
    C2---C2Pool["pool"]
    C2---C2CarType["car_type </br> == Compact"]
    C2Pool-->CarGeneratorPool
  end
```


## 5

```rust
for _ in 0..5 {
    let c3 = car_generator.new(CarType::Suv);
    c3.render(
        color_generator(),
        rng.gen_range(min_point, max_point),
        rng.gen_range(min_point, max_point)
    );
    car_counter += 1;
}
```

```mermaid
graph LR
  Car["Car (Struct)"]-. Instantiate .->CarGenerator["car_generator"]

  subgraph car_generator
    CarGenerator---CarGeneratorCarType["car_type </br> == Nothing"]
    CarGenerator---CarGeneratorPool["car_type</br> { Nothing : car_generator, </br> SubCompact: c1, </br> Compact: c2, </br> Suv: c3 } "]
    CarGeneratorPool-->CarGenerator
  end

  CarGeneratorPool-->C1["c1"]
  subgraph c1
    C1---C1Pool["pool"]
    C1---C1CarType["car_type </br> == SubCompact"]
    C1Pool-->CarGeneratorPool
  end

  CarGeneratorPool-->C2["c2"]
  subgraph c2
    C2---C2Pool["pool"]
    C2---C2CarType["car_type </br> == Compact"]
    C2Pool-->CarGeneratorPool
  end

  CarGeneratorPool-->C3["c3"]
  subgraph c3
    C3---C3Pool["pool"]
    C3---C3CarType["car_type </br> == Suv"]
    C3Pool-->CarGeneratorPool
  end
```


## 6

```rust
let c4 = car_generator.new(CarType::SubCompact);
```

```mermaid
graph LR
  Car["Car (Struct)"]-. Instantiate .->CarGenerator["car_generator"]

  subgraph car_generator
    CarGenerator---CarGeneratorCarType["car_type </br> == Nothing"]
    CarGenerator---CarGeneratorPool["car_type</br> { Nothing : car_generator, </br> SubCompact: c1, </br> Compact: c2, </br> Suv: c3 } "]
    CarGeneratorPool-->CarGenerator
  end

  CarGeneratorPool-->C1["c1"]
  subgraph c1
    C1---C1Pool["pool"]
    C1---C1CarType["car_type </br> == SubCompact"]
    C1Pool-->CarGeneratorPool
  end

  CarGeneratorPool-->C2["c2"]
  subgraph c2
    C2---C2Pool["pool"]
    C2---C2CarType["car_type </br> == Compact"]
    C2Pool-->CarGeneratorPool
  end

  CarGeneratorPool-->C3["c3"]
  subgraph c3
    C3---C3Pool["pool"]
    C3---C3CarType["car_type </br> == Suv"]
    C3Pool-->CarGeneratorPool
  end

  C4["c4"]-->C1
```


## 7

```rust
let c5 = car_generator.new(CarType::SubCompact);
```

```mermaid
graph LR
  Car["Car (Struct)"]-. Instantiate .->CarGenerator["car_generator"]

  subgraph car_generator
    CarGenerator---CarGeneratorCarType["car_type </br> == Nothing"]
    CarGenerator---CarGeneratorPool["car_type</br> { Nothing : car_generator, </br>
    SubCompact: c1, </br>
    Compact: c2, </br>
    Suv: c3 } "]
    CarGeneratorPool-->CarGenerator
  end

  CarGeneratorPool-->C1["c1"]
  subgraph c1
    C1---C1Pool["pool"]
    C1---C1CarType["car_type </br> == SubCompact"]
    C1Pool-->CarGeneratorPool
  end

  CarGeneratorPool-->C2["c2"]
  subgraph c2
    C2---C2Pool["pool"]
    C2---C2CarType["car_type </br> == Compact"]
    C2Pool-->CarGeneratorPool
  end

  CarGeneratorPool-->C3["c3"]
  subgraph c3
    C3---C3Pool["pool"]
    C3---C3CarType["car_type </br> == Suv"]
    C3Pool-->CarGeneratorPool
  end

  C4["c4"]-->C1
  C5["c5"]-->C1
```


## 8

```rust
let c6 = car_generator.new(CarType::Suv);
```

```mermaid
graph LR
  Car["Car (Struct)"]-. Instantiate .->CarGenerator["car_generator"]

  subgraph car_generator
    CarGenerator---CarGeneratorCarType["car_type </br> == Nothing"]
    CarGenerator---CarGeneratorPool["car_type</br> { Nothing : car_generator, </br> SubCompact: c1, </br> Compact: c2, </br> Suv: c3 } "]
    CarGeneratorPool-->CarGenerator
  end

  CarGeneratorPool-->C1["c1"]
  subgraph c1
    C1---C1Pool["pool"]
    C1---C1CarType["car_type </br> == SubCompact"]
    C1Pool-->CarGeneratorPool
  end

  CarGeneratorPool-->C2["c2"]
  subgraph c2
    C2---C2Pool["pool"]
    C2---C2CarType["car_type </br> == Compact"]
    C2Pool-->CarGeneratorPool
  end

  CarGeneratorPool-->C3["c3"]
  subgraph c3
    C3---C3Pool["pool"]
    C3---C3CarType["car_type </br> == Suv"]
    C3Pool-->CarGeneratorPool
  end

  C4["c4"]-->C1
  C5["c5"]-->C1
  C6["c6"]-->C3
```