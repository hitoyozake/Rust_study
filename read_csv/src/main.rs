use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    println!("Hello, world!");

    let path = Path::new("hello.csv");
    let display = path.display();

    let mut file = match File.open(path){
        Ok(file) => file,
        Err(why) => panic!("couldn't open {} : {}", display, why),
    }
    
    let mut str = String::new();
    let mut count = 0; //ヘッダを読み飛ばす場合に必要
    let skip_header = false;
    let mut output = Vec< Vec<&str> >::new();
    match file.read_to_string(&mut str){

        Err(why) => ("couldn't read contents {} : {}", display, why ),

        Ok(_)=>{
            if skip_header == false || count != 0 {
                let v: Vec<&str> = s.split(',').collect();


            }

            ++count;
            //結果はstrに入っている
            
        }

    }


}
