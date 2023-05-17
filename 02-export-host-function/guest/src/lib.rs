extern "C" {
    fn from_host();
    fn from_host2() -> i32;
}

#[no_mangle]
pub extern "C" fn sum(x: i32) -> i32 {
    x + unsafe {
        from_host();
        10
    }
}

#[no_mangle]
pub extern "C" fn foo() -> i32 {
    unsafe {
        from_host2()
    }
}
