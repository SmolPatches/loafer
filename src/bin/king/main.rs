// ref: https://doc.rust-lang.org/book/ch20-01-single-threaded.html
mod cli;
mod subcommands;
// use clap to do command line parsing
// call host(p) or play_list(p)
fn main() {
    cli::setup();
}
