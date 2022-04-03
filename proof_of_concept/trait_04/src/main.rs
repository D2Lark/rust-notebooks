pub struct Dog {
    name: String,
    favorite_food: String,
}
pub struct Cat {
    name: String,
    favorite_food: String,
}

pub trait Animal {
    fn new() -> Self;
    fn name(&self) -> &str;
    fn favorite_food(&self) -> &str;
}

pub trait EatExtend: Animal {
    fn eat(&self);
}

impl<T: Animal> EatExtend for T {
    fn eat(&self) {
        println!("{} eats {}", self.name(), self.favorite_food());
    }
}

impl Animal for Dog {
    fn new() -> Self {
        Dog {
            name: String::from("Dog"),
            favorite_food: String::from("Dog Food"),
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn favorite_food(&self) -> &str {
        &self.favorite_food
    }
}

impl Animal for Cat {
    fn new() -> Self {
        Cat {
            name: String::from("Cat"),
            favorite_food: String::from("Cat Food"),
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn favorite_food(&self) -> &str {
        &self.favorite_food
    }
}

fn main() {
    let d = Dog::new();
    let c = Cat::new();
    println!("{}", d.name());
    println!("{}", c.name());
    d.eat();
    c.eat();
}
