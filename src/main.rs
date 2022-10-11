#![allow(non_snake_case)]
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    let mut derived:u64 = 0;
    for i in 1..args.len() {
        derived |= 1 << u64::from_str_radix(&args[i].to_string(), 10).unwrap();
    }
    if derived & 0xFFFFFFFF00000000 == 0 {
        println!("{:#010x}",derived);
    }
    else {
        println!("{:#018x}",derived);
    }
}
