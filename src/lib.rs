#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#[allow(non_snake_case)]
#[allow(unused_macros)]
mod fighters;
mod hooks;

#[skyline::main(name = "amiibo-balance-patch")]
pub fn main() {
    fighters::install();
    skyline::install_hook!(hooks::replace_param_float);
}
