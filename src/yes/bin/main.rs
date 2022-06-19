use std::io::Error;

use clap::Parser;

#[derive(Parser)]
struct CLI {
    #[clap(value_parser, multiple_values(true))]
    string: Vec<String>  
}

fn main () -> Result<(), Error> {
    let cli = CLI::parse();

    if cli.string.is_empty() {
        loop {
            println!("y")
        }
    } else {
        let print_str = cli.string.join(" ");
        loop {
            println!("{}", print_str)
        } 
    }
}