mod cli;
mod commands;
mod config;

fn main() {
    let value = crate::cli::parse();
    value.subcmd.call(value.config);
}
