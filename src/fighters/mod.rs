mod mario;
mod donkey_kong;
mod link;
mod samus;
mod dark_samus;

pub fn install() {
    mario::install();
    donkey_kong::install();
    link::install();
    samus::install();
    dark_samus::install();
}