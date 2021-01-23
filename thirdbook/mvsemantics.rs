
fn main(){
    let mut important_data = "Hello, world!".to_string();
    //rustの = は原則 movesemantics. C++ は copy semanticsなので基本コピーとしての意味論になるが，rustはmoveになる
    important_data = calc_data(important_data); // 醤油うけんを私，返してもらう
    calc_dataref(&important_data);

    println!("{}", important_data);


    // 参照渡しをできる回数の話
    let mut x = 64;

    {
        let y = & x;
        let z = & x; // ２回めの参照渡し

        // let y = &x; let z = &mut x; // error: 可変と不変の参照渡しの両方を保つことはできない 
        // let mut y = &mut x; // let z = &mut x; // error : 可変な参照渡しは１個まで
    }
   
    
}

// 引数はmove semanticsで渡されるので所有権を保持する
fn calc_data(data: String) -> String {
    println!("{} @ calcData", data);
    data
}

// 参照にするとmoveではなくなる
// 参照を渡す方法を借用という
fn calc_dataref(data: &String){
    println!("{} @ calcDataRef", data);
}

