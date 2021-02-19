use std::env;
use clap::{App, Arg};

fn main(){
    let matches = App::new("My RPN progr")
            .version("1.0.0")
            .author("Your name")
            .about("Super awesome sample RPN calculator")
            .arg(
                Arg::with_name("verbose")
                .about("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false)
            ).arg(
                Arg::with_name("formula_file")
                .about("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false) 
            ).get_matches();

        match maches.value_of("formula_file"){
            Some(file) => println!("File specified: {}", file),
            None => println!("No file specified."),
        }
        let verbose = matches.in_present("verbose");
        println!("Is verbosity specified?: {}", verbose)
}


/*
fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}
*/
