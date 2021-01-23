
fn main(){
    let mut important_data = "Hello, world!".to_string();
    //rustの = は原則 movesemantics. C++ は copy semanticsなので基本コピーとしての意味論になるが，rustはmoveになる
    important_data = calc_data(important_data); // 醤油うけんを私，返してもらう
    calc_dataref(&important_data);

    println!("{}", important_data);
}

// 引数はmove semanticsで渡されるので所有権を保持する
fn calc_data(data: String) -> String {
    println!("{} @ calcData", data);
    data
}

// 参照にするとmoveではなくなる
fn calc_dataref(data: &String){
    println!("{} @ calcDataRef", data);
}

