#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![feature(shrink_to)]
#[allow(non_snake_case)]

mod fighters;

#[skyline::main(name = "amiibo-balance-patch")]
pub fn main() {
    fighters::install();
}