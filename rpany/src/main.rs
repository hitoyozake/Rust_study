use anyhow::{bail, ensure, Context, Result};

use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, fomula: &str) -> Result<i32> {
        let mut tokens = fomula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

#[derive(Parser,Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.1",
    author = "My Name",
    about = "program"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    fomula_file: Option<String>,
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()>
{
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;

        match calc.eval(&line){
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let opts = Opts::parse();

    if let Some(path) = opts.fomula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}
