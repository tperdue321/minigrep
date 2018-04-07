extern crate minigrep;

use std::env;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
    println!("error while parsing command line args: {}", err);
    process::exit(1);
  });
  
  if let Err(e) = minigrep::run(config) {
    println!("application error {}", e);
    process::exit(1);
  }
}

