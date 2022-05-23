#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#[allow(non_snake_case)]
#[allow(unused_macros)]

mod fighters;

#[skyline::main(name = "amiibo-balance-patch")]
pub fn main() {
    fighters::install();
}