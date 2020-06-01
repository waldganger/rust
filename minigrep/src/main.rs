use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|erreur| {
        println!("Erreur : {}", erreur);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Erreur dans l'application : {}", e);
        process::exit(1);
    }

}