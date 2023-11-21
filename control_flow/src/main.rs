use std::io;

fn main() {
    let x = 3;
    let condition: bool = true;

    if x < 3 {
        println!("x is less than 3");
    } else if x == 3 {
        println!("x is equal to 3");
    } else {
        println!("x is greater than 3");
    }

    // Because if is an expression, we can use it on the right side of a let statement
    let vip = if condition { 1 } else { 0 };

    println!("vip: {}", vip);
    println!("----------- LOOP -----------");
    let loop_num = returning_loop();
    println!("loop_num {} is equal to total (iterations * 2)", loop_num);
    loop_labels();

    println!("----------- WHILE -----------");
    while_example();
    while_iteration();

    println!("----------- FOR -----------");
    for_iteration();
    for_range();

    println!("----------- EXERCISES PROPOSED -----------");
    println!("FAHRENHEIT TO CELSIUS");
    fahrenheit();
    println!("FIBONACCI");
    println!("What nth of the Fibonacci sequence: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,                            // if match is Ok, return num
        Err(_) => panic!("Please enter a number"), // if match is Err, continue ignores the rest of the code and starts the loop again
    };
    println!("The nth Fibonacci term is:{}", fibonacci(n));

}

fn returning_loop() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("loop iteration: {}", counter);

        if counter == 10 {
            // The break value is returned from the loop
            break counter * 2;
        }
    };
    result
}

fn loop_labels() {
    let mut counter = 0;
    // when you have nested loop you can break to a label.
    // 'label: loop

    'outer_loop: loop {
        counter += 1;
        println!("outer loop iteration: {}", counter);

        let mut inner_counter = 0;
        loop {
            inner_counter += 1;
            println!("inner loop iteration: {}", inner_counter);

            if inner_counter == 5 {
                println!("breaking inner loop");
                break;
            }
        }

        if counter == 3 {
            println!("breaking outer loop");
            break 'outer_loop;
        }
    }
}

fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn while_iteration() {
    let a = [10, 20, 30, 40, 50];
    let total = a.len();
    let mut index = 0;

    while index < total {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_iteration() {
    // <-- method preferred over while because of safety
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

fn for_range() {
    // The range 1..4 is an iterator that produces 1, 2, and 3.
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // The range 1..=4 is an iterator that produces 1, 2, 3, and 4.
    for number in 1..=10 {
        println!("time in air :{}s", number);
    }

    // you can use .rev() to reverse the range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LANDING!!!");
}

fn fahrenheit() {
    println!("Temperature in F: ");
    let mut f = String::new();
    io::stdin().read_line(&mut f).expect("Failed to read line");

    let f: f32 = match f.trim().parse() {
        Ok(num) => num,                            // if match is Ok, return num
        Err(_) => panic!("Please enter a number"), // if match is Err, continue ignores the rest of the code and starts the loop again
    };

    let c = (f - 32.0) * 5.0 / 9.0;
    println!("Temperature in C: {}", c);
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
