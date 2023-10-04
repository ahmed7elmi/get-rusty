use std::io;

fn main() {
    println!("Hello fellow rustian! What's your name?");

    let mut name = String::new();
    
    io::stdin().read_line(&mut name).expect("Failed to read your name.");

    let name = name.trim();

    println!("Horraay {}!! You just wrote your first Rust program :D", name);
}
