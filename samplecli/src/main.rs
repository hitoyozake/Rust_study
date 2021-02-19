use std::env;
use clap::{App, Arg};

fn main(){
    //builders pattern
    let matches = App::new("My RPN progr")
            .version("1.0.0")
            .author("Your name")
            .about("Super awesome sample RPN calculator")
            .arg(
                Arg::new("verbose")
                .about("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false)
            ).arg(
                Arg::new("formula_file")
                .about("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false) 
            ).get_matches();

        match matches.value_of("formula_file"){
            Some(file) => println!("File specified: {}", file),
            None => println!("No file specified."),
        }
        let verbose = matches.is_present("verbose");
        println!("Is verbosity specified?: {}", verbose)
}


/*
fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}
*/
