use std::env;
use std::fs::File;

mod byte_code;
mod lex;
mod parse;
mod value;
mod vm;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} script", args[0]);
        return;
    }
    let file = File::open(&args[1]).unwrap();

    let proto = parse::load(file);
    vm::ExeState::new().execute(&proto);
}
