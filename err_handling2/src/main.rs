use thiserror::Error;
use anyhow::{Context, Result};

/* this error */
#[derive(Error, Debug)]
enum MyError {
    #[error("failed to read string fromn {0}")]
    ReadError(String),
    #[error(transparent)] // transparentの場合は標準エラーそのまま表示?
    ParseError(#[from] std::num::ParseIntError),
}

fn get_int_from_file_this_error() -> Result<i32, MyError>
{
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|_| MyError::ReadError(path.into()))?;

    num_str.trim().parse::<i32>().map(|t| t*2).map_err(MyError::from) // fromで受けられる
}


fn get_int_from_file() -> Result<i32>{
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t*2)
        .context("failed to parse strin")
}

fn main() {
    match get_int_from_file_this_error() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
