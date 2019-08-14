
use crate::vm::*;

pub fn is_instruction_valid(op_code: char) -> bool {
     for code in VM_INSTRUCTIONS.iter() {
         if *code == op_code {
             return true
         }
     }
    false
}

pub fn rotr(i: i32) -> i32 {  i / 3 + i % 3 * 19683 }

pub fn crz_op(x: usize, y: usize) -> usize {
        let mut i: usize = 0;

        const p9: [usize; 5] = [1, 9, 81, 729, 6561];

        const o_table: [[usize; 9]; 9] = [
            [4, 3, 3, 1, 0, 0, 1, 0, 0],
            [4, 3, 5, 1, 0, 2, 1, 0, 2],
            [5, 5, 4, 2, 2, 1, 2, 2, 1],
            [4, 3, 3, 1, 0, 0, 7, 6, 6],
            [4, 3, 5, 1, 0, 2, 7, 6, 8],
            [5, 5, 4, 2, 2, 1, 8, 8, 7],
            [7, 6, 6, 7, 6, 6, 4, 3, 3],
            [7, 6, 8, 7, 6, 8, 4, 3, 5],
            [8, 8, 7, 8, 8, 7, 5, 5, 4],
        ];

        for index in 0..5 {
            let s = y / p9[index] % 9;

            let d = x / p9[index] % 9;

            let oa = o_table[s][d];

            let oz = oa * p9[index];

            i += oz;
        }
        i
    }