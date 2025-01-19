// std::env receives user flags for specified command line values
use std::env;
/// # main()
/// declares default array of bool values for the current array and the next array\
/// the main goal is to overwrite the current row with the next and\
/// utilize the Rule 110 mechanic to create perpuate the process for 10 rows
fn main() {
    let mut _args = env::args().nth(1);
    let mut current: [bool; 8] = [true, false, true, false, false, true, false, false];
    let mut next: [bool; 8] = [false; 8];
    print_row(current);
    for _ in 0..9 {
        for i in 0..8 {
            let pl_l = current[(i + 7) % 8];
            let pl_c = current[i];
            let pl_r = current[(i + 1) % 8];
            next[i] = rule110([pl_l, pl_c, pl_r]);
        }
        print_row(next);
        current = next;
    }
}
/// ## rule110 (bits: [bool; 3]) -> bool
/// matches the given boolean combination to a result value and returns the\
/// new singular value according to the Rule 110 mechanic
fn rule110(bits: [bool; 3]) -> bool {
    match bits {
        [true, true, true] => false,
        [true, true, false] => true,
        [true, false, true] => true,
        [true, false, false] => false,
        [false, true, true] => true,
        [false, true, false] => true,
        [false, false, true] => true,
        [false, false, false] => false,
    }
}
/// ## print_row(row: [bool; 8])
/// takes an array of booleans and matches each value:\
/// if 1 then *\
/// if 0 then .\
/// then combines the values into a string and prints said string
fn print_row(row: [bool; 8]) {
    let row_string: String = row.iter().map(|&bit| if bit { '*' } else { '.' }).collect();
    println!("{}", row_string);
}
