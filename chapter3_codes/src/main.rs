pub mod functions; // includes the functions module

use std::io;

fn main() {
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    let spaces = "   ";
    println!("currently spaces is: ({spaces})");
    let spaces = spaces.len();
    println!("now spaces is: ({spaces})"); //spaces now is 3

    let x = 2.0; 
    println!("floating point no. x is: {x}"); // prints 2
    let y: f32 = 3.0;
    println!("floating point no. of f32 type y is: {y}");
    
    let sum = 60 + 9;
    println!("sum of 60 and 9 is: {sum}");
    let diff = 95.5 - 4.30;
    println!("difference between 95.5 and 4.3 is: {diff}"); // prints 91.2
    let div = 56.7/32.2;
    println!("56.7 divided by 32.2 gives: {div}"); // prints 1.7608695652173911, that is precision upto 16 digits
    let div = 5/2;
    println!(" 5 divided by 2 is: {div}"); //prints 2
    let remainder = 43 % 5;
    println!("remainder when 43 is divided by 5 is: {remainder}");

    let t: bool = true; let f = false;  // assigning 1 and 0 to bools is not possible, gives compilation error
    println!("value stored in t is: {t} and f is: {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c is {c}, z is {z} and heart eyed cat is {heart_eyed_cat}");

    print!("checking difference between println and print..."); // doesn't add newline at the end
    print!("\n");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("the tuple is: {:?}", tup); // prints the tuple (500, 6.4, 1)
    let tup = (500, 6.4, 1);
    println!("tuple without manually defining the datatypes: {:?}", tup); // prints the same thing

    let five_hundred = tup.0;
    let sixpointfour = tup.1;
    let one = tup.2;
    println!("elements of tuple accessed using '.' are: {five_hundred}, {sixpointfour}, {one}");

    let a = [1, 3, 2, 4, 5];
    println!("the array is: {:?}", a); // prints [1, 3, 2, 4, 5]
    let a = [3; 5];
    println!("the array is: {:?}", a); // prints [3, 3, 3, 3, 3]
    let first = a[0]; let second = a[1];
    println!("elements of array accessed using arr[index] are: {first}, {second}");

    let a = [1, 2, 3, 4, 5];
    println!("please enter the array index: ");
    let mut index  = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index    
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("the value of the element at index {index} is: {element}");

    functions::lmao(); // calls lmao function from the functions module

}
