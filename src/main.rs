extern crate clap;
use clap::{Arg, App};

fn main() {
    let app = App::new("Hello Rust")
                .arg(Arg::with_name("input")
                    .long("input")
                    .takes_value(true)
                    .required(true));

    let matches = app.get_matches();
    let input = matches.value_of("input").unwrap();

    println!("{}", input);
}
