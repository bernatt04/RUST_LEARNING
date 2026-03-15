#[derive(Debug, Clone)]
struct Dog {
    name: String,
}

#[derive(Debug)]
struct Cat {
    name: String,
}

#[derive(Debug)]
struct Parrot {
    name: String,
}

trait Animal {
    fn name(&self) -> &str;
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Parrot {
    fn name(&self) -> &str {
        &self.name
    }
}

pub fn hacer_ruido(animal: &impl Animal) {
    println!("Animal detectado: {}", animal.name());
}

fn main() {
    let dog = Dog {
        name: String::from("PEPE"),
    };

    hacer_ruido(&dog);
    hacer_ruido(&dog);
}
