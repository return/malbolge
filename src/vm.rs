
use std::*;
use std::io::*;

use std::str;
use std::path::PathBuf;

use crate::util::*;

pub const VM_INSTRUCTIONS: &[char] = &['j', 'i', '*', 'p', '<', '/', 'v', 'o'];

// The Encryption and Decryption Tables for Malbolge Instructions.
pub const XLAT1: &str = "+b(29e*j1VMEKLyC})8&m#~W>qxdRp0wkrUo[D7,XTcA\"lI.v%{gJh4G\\-=O@5`_3i<?Z';FNQuY]szf$!BS/|t:Pn6^Ha";
pub const XLAT2: &str = "5z]&gqtyfr$(we4{WP)H-Zn,[%\\3dL+Q;>U!pJS72FhOA1CB6v^=I_0/8|jsb9m<.TVac`uY*MK'X~xDl}REokN:#?G\"i@";

pub struct VirtualMachine {
    debug: bool,
    max_value: usize,
    mem_size: usize,
    memory: Vec<usize>,
    stdin: Stdin,
    stdout: Stdout,
}

pub trait VMEventHandler {
    fn program_decrypted(src: Vec<usize>);
    fn on_execute_started();
    fn on_instruction_skipped(c_reg: usize , c_addr: usize);
    fn on_instruction_execute(a_reg: usize, c_reg: usize, c_addr: usize, d_reg: usize, e_reg: usize, instr: char);
    fn on_execute_finished();
}

impl VMEventHandler for VirtualMachine {

    fn program_decrypted(src: Vec<usize>) {
        println!("Program {:?} Decrypted", src);
    }

    fn on_execute_started() {
        println!("Program Started Executuing");
    }

    fn on_instruction_skipped(c_reg: usize , c_addr: usize){
        println!("{}",format!("Instruction Skipped at address: {} value: {} ", c_reg, c_addr));
    }

    fn on_instruction_execute(a_reg: usize, c_reg: usize, c_addr: usize, d_reg: usize, d_addr: usize, instr: char) {

        let asm: String = match instr {
            'j' => { String::from("MOV D, [D]") }
            'i' => { String::from("MOV C, [D]") }
            '*' => { String::from("ROTR [D]; MOV A, [D]")}
            'p' => { String::from("CRZ A, [D]; MOV A, [D]") }
            '<' => { format!("OUT A // \"{}\"",  (char::from_u32(a_reg as u32 & 0xFF).unwrap() )) }
            '/' => { String::from("IN A") }
            'o' => { String::from("NOP") }
            'v' => { String::from("EXIT") }
            _ =>  { String::from("???") }
        };

        let msg = format!("{} // op: ({}), A: {}, C: {}, [C]: {}, D: {}, [D]: {}", asm, instr, a_reg, c_reg, c_addr, d_reg, d_addr);

        println!("{}",msg);
    }

    fn on_execute_finished() {
        println!("Program Finished Executing");
    }
}


impl VirtualMachine {

    pub fn new(debug: bool) -> VirtualMachine {
        let max_value: usize = 59048;
        let mem_size = max_value + 1;

        VirtualMachine {
            debug: debug,
            max_value: max_value,
            mem_size: mem_size,
            memory: Vec::new(),
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    pub fn load_program(&mut self, program_path: PathBuf) -> bool {
        if program_path.is_file() {
            let d = fs::read(program_path).unwrap();
            self.load(d);
        } else {
            panic!("Failed to load a valid Malbolge Program");
        }
        true
    }

    pub fn load(&mut self, prog: Vec<u8>) -> bool {

            self.memory = vec![0u32 as usize; self.mem_size];

            let prog = prog;

            let mut source_index: usize = 0;
            let mut op_code_index: usize = 0;

            for x in prog {

                let d = usize::from(x);

                if x.is_ascii_whitespace() {
                    continue;
                }

                let mut op_code = 0;

                if d < 127 && d > 32 {

                    let xlat = (d - 33 + op_code_index) % 94;

                    op_code = XLAT1.chars().nth(xlat as usize).unwrap() as u8;

                    if !is_instruction_valid(op_code as char) {
                        panic!(format!{"Invalid VM instruction at line: {}, op_code: {}, decrypted_op_code: {}", source_index, x, op_code});
                    }
                }
                if source_index == self.mem_size {
                    panic!(format!{"Input file is too long!"});
                }

                self.memory.insert(op_code_index, d);
                op_code_index += 1;
                source_index += 1;
            }

            let mut source: Vec<usize> = vec![0u32 as usize; op_code_index];

            source.copy_from_slice( &self.memory[0..op_code_index]);

            if self.debug == true {
                self::VirtualMachine::program_decrypted(source);
            }

            while op_code_index < self.mem_size {

                let crzy = crz_op(self.memory[(op_code_index  - 1)], self.memory[ (op_code_index - 2)]);

                self.memory[op_code_index] = crzy;

                op_code_index += 1;
            }

        if self.debug == true {
            println!("Program Loaded, Size: {}", source_index);
        }

        true
    }


    pub fn exec(&mut self) {

        let mut reg_a: usize = 0;
        let mut reg_c: usize = 0;
        let mut reg_d: usize = 0;

        if self.debug == true {
            self::VirtualMachine::on_execute_started();
        }

        loop {

            if self.memory[reg_c] < 33 || self.memory[reg_c] > 126 {
                println!("Instructions Skipped, value {} in reg memory {} ", reg_c, self.memory[reg_c] );
                continue;
            }

            let xlat = (self.memory[reg_c] - 33 + reg_c) % 94;

            let instr_char = XLAT1.chars().nth(xlat).unwrap();

            match instr_char {
                'j' => { reg_d = self.memory[reg_d]; }
                'i' => { reg_c = self.memory[reg_d]; }
                '*' => {
                    self.memory[reg_d] = rotr( self.memory[reg_d] as i32) as usize;
                    reg_a = self.memory[reg_d];
                }
                'p' => {
                    self.memory[reg_d] = crz_op(reg_a, self.memory[reg_d]);
                    reg_a = self.memory[reg_d];
                }
                '<' => {
                    self.stdout.write_all(&[(reg_a & 0xFF) as u8]).expect("Failed to write to buffer.");
                    self.stdout.flush().unwrap();
                }
                '/' => {
                    let mut buf: [u8; 1] = Default::default();
                    let _handle = self.stdin.read(&mut buf).unwrap();
                    let mut in_var = buf[0];

                    if buf[0] == b"\n"[0] {
                        in_var = 10;
                    }
                    else if buf.is_empty() {
                        in_var = self.max_value as u8;
                    }
                    reg_a =  usize::from(in_var);
                }
                'v' => { return; }
                _ => {}
            };

            let xlat2 = (self.memory[reg_c] - 33) as usize;

            let instr_char_2 = XLAT2.chars().nth(xlat2 as usize).unwrap();

            self.memory[reg_c] = instr_char_2 as usize;

            if reg_c == self.max_value {
                reg_c = 0;
            }
            else {
                reg_c += 1;
            }
            if reg_d == self.max_value {
                reg_d = 0;
            } else {
                reg_d += 1;
            }
        }
    }
}