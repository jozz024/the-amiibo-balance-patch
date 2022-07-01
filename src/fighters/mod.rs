mod mario;
mod donkey_kong;
mod link;
mod samus;
mod dark_samus;
mod yoshi;
mod kirby;

pub fn install() {
    mario::install();
    donkey_kong::install();
    link::install();
    samus::install();
    dark_samus::install();
    yoshi::install();
    kirby::install();
}