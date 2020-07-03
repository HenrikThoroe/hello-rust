extern crate clap;
use clap::{Arg, App};

mod math;
use math::pow as pow;
use math::mult as mult;

fn main() {
    let app = App::new("Hello Rust").arg(Arg::with_name("input").long("input").takes_value(true));

    let matches = app.get_matches();

    let input = matches.value_of("input").unwrap_or("Nothing Provided");
    println!("{}", input);

    let power = pow(&2, &3);
    let prod = mult(&2, &3);
    println!("{}, {}", power, prod);
}
