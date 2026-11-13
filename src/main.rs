use num_bigint::BigUint;
use num_prime::{Primality, nt_funcs::factorize};
use std::{env::args, fmt, process::ExitCode, str::FromStr};

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
    let first_arg = match args().nth(1) {
        Some(first_arg) => first_arg,
        None => {
            eprintln!("Please provide the argument <number>.");
            return ExitCode::FAILURE;
        }
    };

    let num = match BigUint::from_str(&first_arg.trim()) {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Invalid argument. It has to be a number.");
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // pre test with Miller-Rabin Test
    match num_prime::nt_funcs::is_prime(&num, None) {
        Primality::Yes => {
            println!("{num}");
            return ExitCode::SUCCESS;
        },
        Primality::Probable(_) => {
            println!("{num}");
            return ExitCode::SUCCESS;
        },
        Primality::No => {}
    }

    let mut human_readable_vec = NewVec(Vec::new());
    
    // transform to a more human-readable form
    let factors_tree = factorize(num);
    for key in factors_tree.keys() {
        for _ in 0..*factors_tree.get(key).unwrap() {
            human_readable_vec.0.push(key);
        }
    }

    println!("{}", human_readable_vec);
    ExitCode::SUCCESS
}
