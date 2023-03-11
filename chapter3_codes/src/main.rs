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
}
