use std::{env::args, fmt, process::{self, ExitCode}};

struct NewVec<T>(Vec<T>);

impl<T: fmt::Display> fmt::Display for NewVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
        write!(f, "{s}")
    }
}

fn main() -> ExitCode {
    let num = if let Some(first_arg) = args().nth(1) {
        match u64::from_str_radix(&first_arg.trim(), 10) {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Invalid argument. It has to be a number.");
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE
            }
        }
    } else {
        eprintln!("Please provide the argument <number>.");
        return ExitCode::FAILURE
    };
    
    // let num = 64u64;
    let mut factors: NewVec<u64> = NewVec(Vec::new());
    
    factor(&mut factors, num);

    println!("{}", factors);
    ExitCode::SUCCESS
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