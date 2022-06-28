mod donkey_kong;
mod link;
mod mario;
mod samus;

pub fn install() {
    mario::install();
    donkey_kong::install();
    link::install();
    samus::install();
}
