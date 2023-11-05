use rand::Rng;
use std::io;

fn main() {
    println!("--- Guess the number! ---");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess: ");

    /*
    let mut name creates a variable called name, which is mutable
    by default variables are immutable
    String::new() creates a new instance of a string making name be an empty String
    */
    let mut number = String::new();

    /*
    We pass &mut name. Passes name as a reference so that read_line can modify it
    it has to be mutable
    */
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    println!("Your guess is : {number}"); // can also be written as println!("Hello, {}", number);
}

// cargo doc --open <--- great command to get the different posible usages of your code
