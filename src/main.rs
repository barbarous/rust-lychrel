extern crate num_bigint;

use num_bigint::BigUint;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Application need a number as the first parameter.")
    } else {
//        match args[1].parse::<u128>() {
//            Err(_) => println!("{} is not a number", args[1]),
//            Ok(lychrel) => {
//                println!("init value: {}", lychrel);
//                println!("result: {}", revert_str(lychrel));
//            }
//        }
        match BigUint::parse_bytes(args[1].as_bytes(), 10){
            None => println!("{} isn't a number", args[1]),
            Some(lychrel) => {
                println!("result: {}", revert_str(lychrel));
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
    println!("{}",dec);
    return revert_str(dec + dec_rev);
}
