use scan_rules::scanner::Word;

pub(crate) fn first_try() {
    let input_char = readln! {
        (let input_char: Word<String>) => input_char
    };
    println!("the input value is {input_char}");
    // let cf = "codeforces";
    // match cf.find(input_char) {

    // }
}