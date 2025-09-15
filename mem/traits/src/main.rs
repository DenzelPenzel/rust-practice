use std::{any::Any, fmt};

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

trait Animal: std::fmt::Debug {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

fn make_animal_speak(animal: &impl Animal) {
    animal.speak();
    animal.speak();
    println!("Animal: {animal:?}");
}

fn make_animal() -> impl Animal {
    Cat
}

trait DowncastableAnimal {
    fn speak(&self) {
        println!("No idea");
    }

    fn as_any(&self) -> &dyn Any;
}

struct Tortoise;

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let cat = Cat;
    cat.speak();

    let dog = Dog;
    dog.speak();

    make_animal_speak(&cat);
    make_animal_speak(&dog);

    let animal = make_animal();

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];

    animals.iter().for_each(|f| {
        f.speak();
    });

    let more_animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
    for animal in more_animals {
        if let Some(tortoise) = animal.as_any().downcast_ref::<Tortoise>() {
            println!("I am a tortoise");
        }
    }
}
