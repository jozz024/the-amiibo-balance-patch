#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![feature(shrink_to)]

mod fighters;

#[skyline::main(name = "amiibo-balance-patch")]
pub fn main() {
    fighters::install();
}