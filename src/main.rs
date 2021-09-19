extern crate num_bigint;

use num_bigint::BigUint;
use std::env;
use std::{thread, time};

static mut COUNTER: u32 = 0;
static MAX_DISPLAY_ITR: u32 = 25;
static LAPTOP_SLEEP_SAVER: u64 = 500;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Application need a number as a parameter.")
    } else {
        match BigUint::parse_bytes(args[1].as_bytes(), 10) {
            None => println!("{} isn't a number", args[1]),
            Some(num) => {
                if args.len() == 3 && args[2] == "recursion" {
                    println!("Recursion!");
                    let rvt_num = revert(&num);
                    println!("Result: {}", reverse_and_add_recursively(num, rvt_num));
                } else {
                    println!("Result: {}", reverse_and_add_iteratively(num));
                }
            }
        }
    }
}

fn revert(dec: &BigUint) -> BigUint {
    return dec
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<BigUint>()
        .unwrap();
}

fn reverse_and_add_recursively(num: BigUint, rvt_num: BigUint) -> BigUint {
    let next = num + rvt_num;
    count(&next);
    let rvt_next = revert(&next);
    if next == rvt_next {
        return next;
    }
    return reverse_and_add_recursively(next, rvt_next);
}

fn reverse_and_add_iteratively(num: BigUint) -> BigUint{
    let mut next: BigUint = num;
    let mut rvt_next: BigUint = revert(&next);
    loop {
        next = next + rvt_next;
        count(&next);
        rvt_next = revert(&next);
        if next == rvt_next {
            return next;
        }
    }
}

fn count(num: &BigUint) {
    unsafe {
        COUNTER += 1;
        if COUNTER > MAX_DISPLAY_ITR {
            if COUNTER % 100 == 0 {
                thread::sleep(time::Duration::from_millis(LAPTOP_SLEEP_SAVER));
            }
            print!("\rIterations: {}", COUNTER);
        } else {
            println!("{}", num);
        }
    }
}
