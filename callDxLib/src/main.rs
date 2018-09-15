#[link(name="DxLib_x64.dll")]
#[no_mangle]
extern "stdcall" {
    fn dx_DxLib_Init()->i32;
    fn dx_DxLib_End()->i32;
}


fn main() {

    unsafe {dx_DxLib_Init();}
    println!("Hello, world!");
    unsafe {dx_DxLib_End();}
}
