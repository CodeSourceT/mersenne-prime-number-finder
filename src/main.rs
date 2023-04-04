use mersenne_prime_number_miller_rabin::mersenne_number;
use clap::{App};
use std::process::exit;

fn main() {
    let matches = App::new("Mersenne Number Generator")
        .version("1.0")
        .author("Tom B. <tom.bedino@gmail.com>")
        .about("Generate and test mersenne number")
        .args_from_usage(
            "<exposant> 'Exposant of Mersenne Number'",
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("exposant") {
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
    }
}
