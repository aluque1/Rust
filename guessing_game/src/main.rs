use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("--- Guess the number! ---");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");
        /*
        let mut name creates a variable called name, which is mutable
        by default variables are immutable
        String::new() creates a new instance of a string making name be an empty String
        */
        let mut guess = String::new();
        let quit = String::from("quit");
        /*
        We pass &mut name. Passes name as a reference so that read_line can modify it
        it has to be mutable
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == quit {
            println!("Thank you for playing!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // if match is Ok, return num
            Err(_) => continue, // if match is Err, continue ignores the rest of the code and starts the loop again
        };

        println!("Your guess is : {guess}"); // can also be written as println!("Hello, {}", number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess higher!"),
            Ordering::Greater => println!("Guess lower!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// cargo doc --open <--- great command to get the different posible usages of your code
