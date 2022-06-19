use clap::{self, Parser};

#[derive(Parser, Debug)]
struct CLi{
    #[clap(short = 'n', default_value("false"))]
    new_line: bool,

    #[clap(short = 'e', default_value("false"))]
    enable_backslash: bool,

    #[clap(short = 'E', default_value("false"))]
    disable_backslash: bool,

    #[clap(value_parser, multiple_values(true))]
    string: Vec<String>    
}

fn main() {
    let cli = CLi::parse();
    
    let output_string = cli.string.into_iter()
        .reduce(|mut accum, item|{
            // let mut new_string = String::new();
            accum.push_str(item.as_str());
            accum
        } );

    println!("{:?}", output_string.ok_or::<Result<String, &str>>(Err("")));
}