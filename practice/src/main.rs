
pub mod custom_input;

#[macro_use]
extern crate scan_rules;


fn main() {
    let result = try_readln! { // readln! also similar to try_readln!, but it doesn't a result and panicks if error
        (let n: u32, let m: u32) => (n, m)
    };

    match result {
        Ok((n, m)) => println!("I read n={}, m={}", n, m),
        Err(e) => println!("Failed to parse input: {}", e),
    }

    for _ in 1..5 {
        custom_input::first_try();
    }
}
