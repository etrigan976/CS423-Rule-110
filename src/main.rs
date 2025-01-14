// std::env receives user flags for specified command line values
use std::env;
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
fn print_row(row: [bool; 8]) {
    let row_string: String = row.iter().map(|&bit| if bit {'*'} else {'.'}).collect();
    println!("{}", row_string);
}
