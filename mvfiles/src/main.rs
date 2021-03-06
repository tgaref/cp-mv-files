extern crate common;
use std::process;

fn main() {
    let config = common::parse_arguments();

    if let Err(e) = common::run(config, false) {
        println!("There was an error: {}", e);

        process::exit(1)
    }
}
