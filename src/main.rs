use std::{env::args, fmt};

struct NewVec<T>(Vec<T>);

impl<T: fmt::Display> fmt::Display for NewVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
        write!(f, "{s}")
    }
}

fn main() {
    let input = args().nth(1).expect("ERROR: need 1 argument <number>");
    let num = u64::from_str_radix(&input.trim(), 10).expect("ERROR: need 1 argument <number>");
    let mut factors: NewVec<u64> = NewVec(Vec::new());
    
    factor(&mut factors, num);

    println!("{}", factors);
}

fn factor(vec: &mut NewVec<u64>, mut num: u64) {
    let mut i: u64 = 2;
    while i <= num / i {
        if num % i == 0 {
            vec.0.push(i);
            num = num / i;
            i = 2;
        } else {
            i = i + 1;
        }
    }
    if num > 0 {
        vec.0.push(num);
    }
}