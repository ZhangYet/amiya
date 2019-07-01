use std::io;
use std::io::Write;

fn main() {
    println!("Hello, Doctor!");
    loop {
        print!("Doctor: ");
        let _ = io::stdout().flush();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Did not enter a correct string");
        print!("Amiya: {}", s);
    }
}
