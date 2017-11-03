use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate csv;
//extern crate serde;

fn read_csv(){

   let people = vec![
       ("Yamada", "Taro", 1990),
       ("Sato", "Jiro", 1920),
       ("Yoshida", "Osamu", 1993),
   ];

   let mut writer = csv::Writer::from_path("sample.csv").unwrap();

   //headerのserialize
   writer.serialize(("FirstName", "SecondName", "birth year"));

   for row in people{
       writer.serialize(row);       
   }
       
}


fn main() {
    read_csv();
    println!("Hello, world!");

    read_csv();

    let path = Path::new("hello.csv");
    let display = path.display();

    let mut file = match File::open(path){
        Ok(file) => file,
        Err(why) => panic!("couldn't open {} : {}", display, why),
    };
    
    let mut s = String::new();
    let mut count = 0; //郢晏･繝｣郢敖郢ｧ螳夲ｽｪ�ｽｭ邵ｺ�ｽｿ鬯溷ｸ呻ｿｽ�ｽｰ邵ｺ蜷晢ｿｽ�ｽｴ陷ｷ蛹ｻ竊楢�｢�ｿｽ髫包ｿｽ

    let skip_header = true;
    
    let mut output : Vec<Vec<&str>> = Vec::new();
    //荳豌励↓隱ｭ縺ｿ霎ｼ繧
    match file.read_to_string(&mut s){

        Err(why) => panic!("couldn't read contents {} : {}", display, why ),

        Ok(_)=>{
                let v: Vec<&str> = s.split("\n").collect();
                output.push(v);
        },
    };

    let mut output2:Vec<Vec<&str>> = Vec::new();

    for i in output{
        for s in i{
            if skip_header == false || count > 0 {
                output2.push(s.split(",").collect());
            }
            count +=1;
        }
    }

    for i in output2{
        let joined = i.join(",");
        
        println!("{}", joined);    
    }
}
