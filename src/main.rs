struct User {
    name: String,
    last_name: String,
    age: u64,
    is_active: bool,
}

enum Coin {
    Penny,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let mut user = build_user(String::from("Mario"), String::from("Rossi"), 25);
    user.is_active = false;
    println!("User name {} {} {} is_active {}", user.name, user.last_name, user.age,user.is_active);

    let user2 = User {
        name: String::from("Paolo"),
        ..user
    };

    let color = Color(1, 1, 1);

    println!("Color index 0 {}", color.0);

    println!("User name {} {} {} is_active {}", user2.name, user2.last_name, user2.age,user2.is_active);

    let value = values_in_cents(Coin::Penny);
    println!("Value {}", value);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => println!("Is not three"),
    }

    if let Some(3) = some_u8_value {
        println!("Three")
    }
}

fn build_user(name: String, last_name: String, age: u64) -> User {
    User {
        name,
        last_name,
        age,
        is_active: true,
    }
}

fn values_in_cents (coin: Coin) -> u8 {
    match coin  {
        Coin::Penny => 1,
    }
}