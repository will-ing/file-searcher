use rust_grep::Config;
use std::{env, process};
fn main() {
    /*To export output to file: cargo run > output.txt */
    let args: Vec<String> = env::args().skip(1).collect();

    /*
    call new function from Config impl and if there is no error return result,
    if error return error and exit program
    */
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    println!("Looking for {:?}", config.query);
    println!("In file {:?}", config.file);

    if let Err(e) = rust_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
