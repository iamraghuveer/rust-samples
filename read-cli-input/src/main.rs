use std::io;

fn main() {
    println!("Enter your input");

    let mut cli_input = String::new();

    io::stdin()
        .read_line(&mut cli_input)
        .expect("Failed to read value");

    println!("You entered: {cli_input}");
}
