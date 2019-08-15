use clap::{Arg, App};

use malbolge::vm::*;
use std::*;

fn main() -> io::Result<()> {
    let matches = App::new("mbi")
        .version("1.0")
        .author("Calvin Hill <calvin@hakobaito.co.uk>")
        .about("A safe Malbolge interpreter written in Rust.")
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

#[cfg(test)]
mod test {

    extern crate malbolge;
    use malbolge::*;
    use std::io::{stderr, stdout, Write};

    use std::path::PathBuf;
    use std::process::Command;
    use std::env;

    #[cfg(debug_assertions)]
    const path: &str = "./target/debug/mbi";

    #[cfg(not(debug_assertions))]
    const path: &str = "./target/release/mbi";

    #[test]
    fn test_interp() {

        let output = Command::new(path)
            .args(&[format!("{}/{}",env!("CARGO_MANIFEST_DIR"),"examples/hello.mb")])
            .output().expect("Error, failed to execute interpreter");

        println!("status: {}", output.status);

        stdout().write_all(&output.stdout).unwrap();
        stderr().write_all(&output.stderr).unwrap();

        assert!(output.status.success(), true);
    }

}