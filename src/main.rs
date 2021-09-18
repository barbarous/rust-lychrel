extern crate num_bigint;

use num_bigint::BigUint;
use std::env;
use std::{thread, time};

static mut COUNTER: u32 = 0;
static MAX_DISPLAY_ITR: u32 = 25;
static LAPTOP_SLEEP_SAVER: u64 = 1_000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Application need a number as a parameter.")
    } else {
        match BigUint::parse_bytes(args[1].as_bytes(), 10) {
            None => println!("{} isn't a number", args[1]),
            Some(lychrel) => {
                println!("Result: {}", revert_str(lychrel));
            }
        }
    }
}

fn revert_str(dec: BigUint) -> BigUint {
    let dec_rev = dec
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<BigUint>()
        .unwrap();
    if dec == dec_rev {
        return dec;
    }
    display(&dec);
    return revert_str(dec + dec_rev);
}

fn display(num: &BigUint) {
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
