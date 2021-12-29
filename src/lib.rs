#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod mario;

#[skyline::main(name = "amiibo-balance-patch")]
pub fn main() {
    mario::install();
}