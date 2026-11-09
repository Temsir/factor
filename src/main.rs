use std::{env::args, vec};

fn main() {
    let input = args().nth(1).unwrap();
    let num = u64::from_str_radix(&input.trim(), 10).unwrap();
    let mut factors: Vec<u64> = Vec::new();
    
    let mut i = 1u64;
    while i * i <= num {
        if num % i == 0 {
            factors.append(&mut vec![i, num / i]);
        }
        i = i + 1;
    }
    
    println!("{:?}", factors)
}

fn factor(num: u64) {
    
}