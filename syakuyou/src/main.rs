
fn main() {
    let mut important_data = "Hello, Wolrd!".to_string();

    important_data = calc_data(important_data);

    calc_data_without_rent(&important_data);

    println!("{}", important_data);

}

fn calc_data_without_rent(data: &String){
    println!("{}", data);
}

fn calc_data(data: String)->String{
    println!("{}", data);
    data
}
