

fn main() {
    let mut s = String::from("hello"); // for situations where we want to take input of a custom string value, rust has the String type

    s.push_str(", world!"); // push_str appends a literal to a String

    println!("{}", s); // This will print hello, world!

    let s1 = String::from("hello");
    let s2 = s1; // the String data of s1 is copied into s2, rather than the data on the heap which the pointer is pointing to

    println!("s2 is: {s2}"); // prints hello, println!("{s1}") will give an error as s1 is no longer valid

    let x = 5;
    let y = x; // the variables x and y are stored on the stack as size of u32 is known at compile time
                    // the value stored in x is simply copied into y
    println!("values in x and y are: {x}, {y}"); // prints 5, 5
    
    let s1 = String::from("hello");
    let s2 = s1.clone(); // clone makes a deep copy of s1

    println!("s1 is: {s1}, and s2 is: {}", s2); // prints hello, hello

    // Ownership and Functions

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                                    // ... and is no longer valid here

    // println!("s is: {s}"); would throw an error

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                                // but i32 is Copy, so it's okay to still use x afterwards
    
    println!("x is: {x}"); // prints 5;

    let s =  String::from("lmao ded");

    takes_ownership(s.clone()); // s's value is cloned instead of moved and hence s is still valid

    println!("s is still valid and its value is: {s}"); // prints lmao ded


    // Return Values and Scope

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which moves its return value into s3

    println!("s1 and s3 are: {s1}, {s3}");


}   // here, x goes out of scope, then s. 
    //But because s's value was moved, nothing special happens


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}   // here, some_string goes out of scope and 'drop' is called. The backing memory is freed.


fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}   // here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}