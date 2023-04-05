use mersenne_prime_number_miller_rabin::{mersenne_number, is_prime};
use clap::{App};
use std::process::exit;

use num_bigint::BigUint;
use std::str::FromStr;

fn main() {
    let matches = App::new("Mersenne Number Generator")
        .version("1.0")
        .author("Tom B. <tom.bedino@gmail.com>")
        .about("Generate and test mersenne number")
        .args_from_usage(
            "-g, --generate 'Generate Mersenne Number'
             -t, --test 'Test number with Miller-Rabin'
             -m, --mersenne 'Generate Mersenne Number & Test number with Miller-Rabin'
                <number> 'Number'",
        )
        .get_matches();

    if let Some(o) = matches.value_of("number") {
        if matches.is_present("generate") {
            let exposant_opt = o.parse::<u32>();
            let exposant = match exposant_opt {
                Ok(v) => v,
                Err(e) => {
                    println!("An error was occured : {}", e);
                    exit(-1);
                }
            };
            println!("Compute Mersenne Number with exposant {}", exposant);
            let one_mersenne_number = mersenne_number(exposant);
            println!("Result : {}", one_mersenne_number);
        } else if matches.is_present("mersenne"){
            let exposant_opt = o.parse::<u32>();
            let exposant = match exposant_opt {
                Ok(v) => v,
                Err(e) => {
                    println!("An error was occured : {}", e);
                    exit(-1);
                }
            };
            println!("Compute Mersenne Number with exposant {}", exposant);
            let one_mersenne_number = mersenne_number(exposant);
            if is_prime(one_mersenne_number){
                println!("Mersenne Number {} is probably prime!", o);
            }else{
                println!("Mersenne Number {} is probably composite!", o);
            }
        }
        else if matches.is_present("test"){
            let n = BigUint::from_str(o).unwrap();
            if is_prime(n){
                println!("Number {} is probably prime!", o);
            }else{
                println!("Number {} is probably composite!", o);
            }
        } else {
            println!("Nothing to do. Just echo number !");
            println!("{}", o);
        }
    }
}
