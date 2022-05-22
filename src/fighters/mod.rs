mod mario;
mod donkey_kong;
mod link;

pub fn install() {
    mario::install();
    donkey_kong::install();
    link::install();
}