use clap::{Arg, App};

use malbolge::vm::*;
use std::*;

fn main() -> io::Result<()> {
    let matches = App::new("mbi")
        .version("1.0")
        .author("Calvin Hill <calvin@hakobaito.co.uk>")
        .about("A safe malbolge interpreter written in Rust.")
        .arg(Arg::with_name("PROGRAM_FILE")
            .help("Sets the program file to execute")
            .required(false)
            .index(1))
        .get_matches();

    if let Some(e) = matches.value_of("PROGRAM_FILE") {
            let p: std::path::PathBuf = std::path::PathBuf::from(e);
            let mut vm = VirtualMachine::new(false);
            vm.load_program(p);
            vm.exec();
            Ok(())
    }
    else {
        Ok(println!("{}",matches.usage()))
    }
}