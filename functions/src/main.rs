fn main() {
    println!("Hello, world!");

    another_function(45, 'l'); // using "" instead of '' gives an error, expected char and found &str

    let x = five(); // x = 5

    let y = { // blocks are an expression and here the scope block created returns the value x + 1 which is assigned to y
        let x = 3;
        x + 1 // expressions do not include ending semicolons, otherwise they'll turn into statements and not return any value
    };

    println!("The value of x and y are: {x}, {y}"); // prints 5 and 4

    let number = 3;

    if number < 5 {  // blocks of code associated with conditions in if expressions are sometimes called arms just like those in match expressions
                     // the condition must be a bool, for example writing only number will give an error
        println!("condition evaluated to true");  
    } else {
        println!("condition evaluated to false");
    }
    
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("number is divisible by 2.");
    } else {
        println!("number is not divisible by 4, 3 or 2.");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // the values of each arm should be of the same type

    println!("the value of number is: {}", number);

}

fn another_function(value: i64, label: char){  // the declaration of type of each parameter is a must
    println!("The value and label are: {value}, {label}");
}

fn five() -> i32 {
    5 // functions return the last expression implicitly, though we can return early using the return keyword
}