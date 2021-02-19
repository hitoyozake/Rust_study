use std::env;
use clap::{App, Arg, Clap};
use std::io::{stdin, BufRead, BufReader};
use std::fs::File;
#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"

)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
    
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main(){
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file{
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool){
    for line in reader.lines(){
        let line = line.unwrap();
        println!("{}", line);
    }
}
fn main2(){
    /*
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
            ).
            get_matches();
            
        match matches.value_of("formula_file"){
            Some(file) => println!("File specified: {}", file),
            None => println!("No file specified."),
        }
        let verbose = matches.is_present("verbose");
        println!("Is verbosity specified?: {}", verbose)
    */
    let opts = Opts::parse();
    match opts.formula_file{
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
}


/*
fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}
*/
