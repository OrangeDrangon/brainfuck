use std::{fs, path::PathBuf};

use clap::Parser;

use brainfuck::{BrainfuckVM, BrainfuckVMOptions};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    
    let program = fs::read_to_string(args.file).expect("you entered a shit path try again");

    let mut bfvm = BrainfuckVM::new(&program, None, BrainfuckVMOptions::default());

    bfvm.run();

    assert!(!bfvm.not_done());
}
