trait Pet {
    fn name(&self) -> String;
}

struct Cat {
    name: String,
}

impl Pet for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    // This doesn't work because dyn Pet is a trait object, and trait objects must be behind a pointer.
    // let pet: dyn Pet;

    // This works because Box is a pointer.
    let pet: Box<dyn Pet>;
    pet = Box::new(Cat {
        name: "Misty".to_string(),
    });

    println!("You can call me {}", pet.name());
}