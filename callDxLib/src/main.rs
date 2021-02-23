
#[link(name="DxLib_x64_d", kind="static")]
#[no_mangle]
extern {
    fn DxLib_Init()->i32;
    fn DxLib_End()->i32;
}


fn main() {
    unsafe {DxLib_Init();}
    println!("Hello, world!");
    //unsafe {DxLib_End();}
}

/*
#[link(name="foo", kind="static")]
extern {
    fn hoge();
}

fn main()
{
    unsafe{hoge();}
}
*/