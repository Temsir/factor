use num_bigint::BigUint;
use std::{env::args, fmt, process::ExitCode, str::FromStr};

const ZERO: BigUint = BigUint::ZERO;
const ONE: BigUint = BigUint::ONE;
const TWO: BigUint = BigUint::new_const(2u32);

struct NewVec<T>(Vec<T>);

impl<T: fmt::Display> fmt::Display for NewVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{s}")
    }
}

fn main() -> ExitCode {
    let num = match args().nth(1) {
        Some(first_arg) => match BigUint::from_str(&first_arg.trim()) {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Invalid argument. It has to be a number.");
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        },
        None => {
            eprintln!("Please provide the argument <number>.");
            return ExitCode::FAILURE;
        }
    };

    println!("{}", factor(num));
    ExitCode::SUCCESS
}

fn factor(mut num: BigUint) -> NewVec<BigUint> {
    let mut vec: NewVec<BigUint> = NewVec(Vec::new());
    let mut i = TWO;

    while (&i * &i) <= num {
        if (&num % &i) == ZERO {
            vec.0.push(i.clone());
            num = num / &i;
        } else {
            i = i + ONE;
        }
    }

    vec.0.push(num);

    vec
}
