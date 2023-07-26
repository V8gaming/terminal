use terminal::Terminal;
mod update;
mod basic;
mod setup;
mod terminal;
mod image;

fn main() {
    let terminal = Terminal::new();
    terminal.run();
}