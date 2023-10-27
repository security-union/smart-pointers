use std::rc::Rc;

struct Home {
    address: String
}

struct Resident {
    home: Rc<Home>
}

fn main() {
    let home = Rc::new(Home { address: String::from("123 Elm street") });
    let resident_1 = Resident { home: Rc::clone(&home) };
    let resident_2 = Resident { home: Rc::clone(&home) };

    println!("resident_1 address: {}", resident_1.home.address);
    println!("resident_2 address: {}", resident_2.home.address);

    // Friend moves :(
    std::mem::drop(resident_2);

    println!("resident_1 address after drop: {}", resident_1.home.address);
}
