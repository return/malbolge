pub mod vm;
pub mod util;

#[macro_export]
macro_rules! malbolge {
        ($expression:expr) => {
        use vm::VirtualMachine;
        let mut o = VirtualMachine::new(false);
        o.load(Vec::from($expression));
        o.exec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::vm::*;
    use std::path::PathBuf;

    #[test]
    fn malbolge_hello(){
        let mut vm = VirtualMachine::new(false);
        let path = PathBuf::from("hello.mal");
        vm.load_program(path);
        vm.exec();
    }

    #[test]
    fn malbolge_load_path() {
        let vm = VirtualMachine::new(false).load_program(PathBuf::from("./cat.mal"));
        assert_eq!(vm, true);
    }

    #[test]
    fn malbolge_macro_string(){
        malbolge!(r#"(=<`#9]76Z{z2V0/S-Qr*)M:,+*)('&%$#"!~}|{z(Kw%$t"Vq0iAm,,j<h'`%"#);
    }
}