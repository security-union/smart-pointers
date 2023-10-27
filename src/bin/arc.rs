use std::sync::Arc;

struct Home {
    address: String
}

struct Resident {
    home: Arc<Home>
}

fn main() {
    let home = Arc::new(Home { address: String::from("123 Elm street") });
    let resident_1 = Resident { home: Arc::clone(&home) };
    let resident_2 = Resident { home: Arc::clone(&home) };
    let resident_3 = Resident { home: Arc::clone(&home) };

    println!("resident_1 address: {}", resident_1.home.address);
    println!("resident_2 address: {}", resident_2.home.address);
    println!("resident_3 address: {}", resident_3.home.address);

    let handle = std::thread::spawn(move || {
        println!("resident_1 address in thread: {}", resident_1.home.address);
    });

    // Divorce :(
    std::mem::drop(resident_2);

    println!("resident_3 address after drop: {}", resident_3.home.address);
    handle.join().unwrap();
}
