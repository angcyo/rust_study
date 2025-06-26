use safer_ffi::ffi_export;

#[ffi_export]
fn ffi_test() -> bool {
    true
}

#[ffi_export]
fn ffi_test_string() -> safer_ffi::String {
    safer_ffi::String::from("Hello From Rust!")
}

#[ffi_export]
fn ffi_test_string2(input: safer_ffi::String) -> safer_ffi::String {
    safer_ffi::String::from(format!("{}\nHello From Rust!", input.to_string()))
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//---ffi 头文件生成---

#[test]
#[cfg(feature = "headers")]
fn generate_headers() -> std::io::Result<()> {
    safer_ffi::headers::builder()
        .to_file("../cdylib_demo.h")?
        .generate()
}
