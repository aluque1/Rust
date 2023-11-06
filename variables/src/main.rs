fn main() {
    let x = 5;

    let x = x + 1;
    { // This is the start of the scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    } // This is the end of the scope
    println!("The value of x is: {x}");

    let _spaces = "   "; // This spaces_ is a :String
    let _spaces = _spaces.len();
    /*
       Which is shadowed by the second spaces_, which is a :u32
       if you try to use mut here for example you are not allowed
       as you cannot change the type of a variable
    */

    let tup: (u32, u32, u32) = (12, 13, 6); // This is a tuple
                                            // One way to get the separate values is to use indexing :
    let x_axis = tup.0; // This is called indexing
    let y_axis = tup.1;
    let z_axis = tup.2;
    println!("INDEXING :: x: {x_axis}, y: {y_axis}, z: {z_axis}");

    // Another way is to use destructuring :
    let (x, y, z) = tup; // This is called destructuring
    println!("DESTRUCTURING ALL :: x: {x}, y: {y}, z: {z}");

    // You can also use destructuring to get a single value from a tuple
    let (x, _, _) = tup; // This is called destructuring
    println!("DESTRUCTURING SINGLE :: x: {x}");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]; // This is an array
    for month in months.iter() {
        println!("MONTH :: {month}");
    }

    let arr1: [u32; 5] = [1, 2, 3, 4, 5]; // This is an array with a type and a size
    for num in arr1.iter() {
        println!("NUM arr1 :: {num}");
    }
    // can also be written as : let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; // This is an array with a size of 5 and all values are 3
    for num in arr2.iter() {
        println!("NUM arr2 :: {num}");
    }
}
