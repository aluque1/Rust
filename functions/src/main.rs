use std::io;

fn main() {
    let x: i32 = 5;
    let string: String = something(x);
    println!("{}", string);

    let value = 4.12;
    let unit_label = 's';
    print_labeled_measurement(value, unit_label);

    println!("The value of x is: {}", plus_one(x));

    testing();
}

// function that returns a string
fn something(x: i32) -> String {
    let mut number = String::new();
    println!("Please input your number");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return String::from("Please input a number");
        }
    };

    let string = String::from(format!(
        "your number [{}] plus {} is = {}",
        number,
        x,
        number + x
    ));

    return string;
}

fn testing() {
    let y = {
        let x = 3;
        x + 1 // <-- no semicolon because it's an expression
    };
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // <-- no semicolon because it's an expression
}

fn print_labeled_measurement(value: f32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
