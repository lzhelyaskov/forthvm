use std::collections::HashMap;

use crate::align;
use crate::init_dictionary::make_string;
use crate::{COMPILING, INTERPRETING, LEN_MASK, MAX_WORD_LEN, docol, mmap};
use toyvm::VM;
use toyvm::opcode;

pub(crate) const NEXT: u8 = 0xff;
pub(crate) const OVER: u8 = NEXT - 1;
pub(crate) const ROT: u8 = OVER - 1;
pub(crate) const DIV_MOD: u8 = ROT - 1;

fn forth_opcodes(vm: &mut VM, ip: &mut usize, op: u8) -> bool {
    match op {
        NEXT => {
            // IC points to code_ptr of the  next word to execute.
            // jump to code_ptr, increase ic by 4 to point to the next word

            let ic = vm.read_i32(mmap::IC);
            let code_ptr = vm.read_i32(ic as usize) as usize;
            *ip = vm.read_i32(code_ptr) as usize;
            vm.write_i32(ic + 4, mmap::IC);
            vm.write_i32(code_ptr as i32, mmap::A0);
        }
        DIV_MOD => {
            let b = vm.pop_i32();
            let a = vm.pop_i32();

            let q = a / b;
            let r = a % b;

            vm.push_i32(r);
            vm.push_i32(q);
        }
        OVER => {
            // ( a b -- a b a )
            let b = vm.pop_i32();
            let a = vm.pop_i32();
            vm.push_i32(a);
            vm.push_i32(b);
            vm.push_i32(a);
        }
        ROT => {
            // ( a b c -- b c a )
            let c = vm.pop_i32();
            let b = vm.pop_i32();
            let a = vm.pop_i32();
            vm.push_i32(b);
            vm.push_i32(c);
            vm.push_i32(a);
        }
        _ => return false,
    }
    true
}

pub struct VmConfig {
    pub memory_size_bytes: usize,
    pub parameter_stack_size_cells: usize,
    pub return_stack_size_cells: usize,
    pub call_stack_size_cells: usize,
    pub locals_stack_size_cells: usize,
}

pub struct ForthVM {
    pub(crate) vm: VM,
    vocabulary: HashMap<i32, String>,
}

impl ForthVM {
    pub fn from_config(conf: VmConfig) -> Self {
        let memory = vec![0; conf.memory_size_bytes];
        let functions = Vec::new();

        let cstack_top = memory.len() - 4;
        let lstack_top = cstack_top - conf.call_stack_size_cells * 4;
        let rstack_top = lstack_top - conf.locals_stack_size_cells * 4;
        let pstack_top = rstack_top - conf.return_stack_size_cells * 4;

        let mut vm = VM::new(memory, functions, mmap::DSP, mmap::CTOP);

        vm.write_i32(pstack_top as i32, mmap::DSP);
        vm.write_i32(pstack_top as i32, mmap::S0);
        vm.write_i32(rstack_top as i32, mmap::RSP);
        vm.write_i32(rstack_top as i32, mmap::R0);
        vm.write_i32(lstack_top as i32, mmap::CTOP);
        vm.write_i32(lstack_top as i32, mmap::CBASE);
        vm.write_i32(cstack_top as i32, mmap::LTOP);
        vm.write_i32(cstack_top as i32, mmap::LBASE);

        vm.write_i32(0, mmap::SOURCE_ID);
        vm.write_i32(10, mmap::BASE);
        // vm.write_i32(0, mmap::IC);
        // vm.write_i32(0, mmap::A0);

        vm.write_i32(mmap::DICT as i32, mmap::HERE);

        vm.write_i32(rstack_top as i32 + 4, mmap::IN_STREAM);
        vm.write_i32(lstack_top as i32, mmap::INPUT_BUFFER);
        //  vm.write_i32(lstack_top as i32, mmap::INPUT_BUFFER_IDX);
        vm.write_i32(0, mmap::INPUT_BUFFER_IN);

        vm.write_u8(NEXT, mmap::COLD_START);

        vm.write(mmap::DOCOL, &docol());

        vm.add_unknown_op_handler(&forth_opcodes);
        ForthVM {
            vm,
            vocabulary: HashMap::new(),
        }
    }

    pub fn push_i32(&mut self, value: i32) {
        self.vm.push_i32(value);
    }

    pub fn pop_i32(&mut self) -> i32 {
        self.vm.pop_i32()
    }

    pub fn read_i32(&self, idx: i32) -> i32 {
        self.vm.read_i32(idx as usize)
    }

    pub fn write_i32(&mut self, value: i32, idx: i32) {
        self.vm.write_i32(value, idx as usize);
    }

    pub fn read_u8(&self, idx: i32) -> u8 {
        self.vm.read_u8(idx as usize)
    }

    pub fn write_u8(&mut self, value: u8, idx: i32) {
        self.vm.write_u8(value, idx as usize);
    }

    pub fn pstack_depth(&self) -> i32 {
        let base = self.read_i32(mmap::S0 as i32);
        let top = self.read_i32(mmap::DSP as i32);
        (base - top) / 4
    }

    pub fn rstack_depth(&self) -> i32 {
        let base = self.read_i32(mmap::R0 as i32);
        let top = self.read_i32(mmap::RSP as i32);
        (base - top) / 4
    }

    pub fn run_word(&mut self, word_idx: usize) {
        let _cfa = self.cfa(word_idx as i32);
        self.vm.write_i32(_cfa, mmap::START_ADR);
        self.vm.write_i32(mmap::START_ADR as i32, mmap::IC);
        let mut ip = mmap::COLD_START;

        self.vm.run(&mut ip).unwrap();
    }

    pub fn run_word_debug(&mut self, word_idx: usize) {
        self.print_dictionary();
        let _cfa = self.cfa(word_idx as i32);
        self.vm.write_i32(_cfa, mmap::START_ADR);
        self.vm.write_i32(mmap::START_ADR as i32, mmap::IC);
        let mut ip = mmap::COLD_START;

        let stdin = std::io::stdin();
        let mut buf = String::new();

        let op = self.read_u8(ip as i32);
        println!("{} {}", ip, toyvm::opcode::opcode(op));

        while self.vm.step(&mut ip).unwrap() {
            self.print_4th_vars();
            self.print_pstack();
            self.print_rstack();
            print_input_buffer(&self.vm);
            stdin.read_line(&mut buf).unwrap();

            let op = self.read_u8(ip as i32);
            println!("{} {}", ip, toyvm::opcode::opcode(op));
        }
    }

    pub fn fill_input_buffer(&mut self, s: &str) {
        fill_input_buffer(&mut self.vm, s);
    }

    // pub fn read_next_char(&mut self) -> Option<u8> {
    //     read_next_char(&mut self.vm)
    // }

    pub fn read_next_char(&mut self) -> Option<u8> {
        read_next_char_in(&mut self.vm)
    }

    pub fn write_str(&mut self, idx: usize, s: &str) {
        write_str(&mut self.vm, idx, s);
    }

    pub fn latest(&self) -> i32 {
        self.vm.read_i32(mmap::LATEST)
    }

    pub fn set_latest(&mut self, value: i32) {
        self.vm.write_i32(value, mmap::LATEST)
    }

    pub fn here(&self) -> i32 {
        self.vm.read_i32(mmap::HERE)
    }

    pub fn set_here(&mut self, value: i32) {
        self.vm.write_i32(value, mmap::HERE);
    }

    pub fn base(&self) -> i32 {
        self.read_i32(mmap::BASE as i32)
    }

    pub fn set_base(&mut self, value: i32) {
        self.write_i32(value, mmap::BASE as i32);
    }

    pub fn state(&self) -> i32 {
        self.read_i32(mmap::STATE as i32)
    }

    pub fn set_state(&mut self, value: i32) {
        self.write_i32(value, mmap::STATE as i32);
    }

    pub fn is_compiling(&self) -> bool {
        self.state() == COMPILING
    }

    pub fn is_interpreting(&self) -> bool {
        self.state() == INTERPRETING
    }

    pub fn cfa(&self, idx: i32) -> i32 {
        let len = self.read_u8(idx + 4) & LEN_MASK;
        let n = (len as usize).min(MAX_WORD_LEN) as i32;

        align(idx + n + 5)
    }

    pub fn find(&self, name: &str) -> Option<i32> {
        let mut current_word_idx = self.latest();
        loop {
            let len = (self.vm.read_u8(current_word_idx as usize + 4) & LEN_MASK) as usize;
            if len == name.len()
                && self.vm.memcmp_with(
                    current_word_idx as usize + 5,
                    &name.as_bytes()[..len.min(MAX_WORD_LEN)],
                )
            {
                return Some(current_word_idx);
            }

            // find next word
            current_word_idx = self.vm.read_i32(current_word_idx as usize);
            if current_word_idx == 0 {
                break;
            }
        }
        None
    }

    pub fn vm_call(&mut self, name: &str, f: toyvm::VmFn) -> (i32, i32) {
        let fn_idx = self.vm.add_function(f);
        let bytes = fn_idx.to_ne_bytes();
        let word_adr = self.builtin(
            name,
            &[
                opcode::I32_CONST,
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3],
                opcode::CALL_VM,
                NEXT,
            ],
        );
        (fn_idx as i32, word_adr)
    }

    pub fn builtin(&mut self, name: &str, code: &[u8]) -> i32 {
        self.builtin_ex(name, 0, code)
    }

    pub fn builtin_ex(&mut self, name: &str, flags: u8, code: &[u8]) -> i32 {
        let new_last_word_idx = self.here();
        self.write_previous_idx();
        self.write_name(name, flags);
        self.write_codeword_builtin();
        self.write_code(code);
        self.set_latest(new_last_word_idx);

        self.vocabulary
            .insert(self.cfa(new_last_word_idx), name.to_string());

        new_last_word_idx
    }

    pub fn colon_def(&mut self, name: &str, calls: &[&str]) -> i32 {
        self.colon_def_ex(name, 0, calls)
    }

    pub fn colon_def_ex(&mut self, name: &str, flags: u8, calls: &[&str]) -> i32 {
        let new_last_word_idx = self.here();
        self.write_previous_idx();
        self.write_name(name, flags);
        self.write_docol_addr();
        self.write_colon_def(calls);
        self.set_latest(new_last_word_idx);

        self.vocabulary
            .insert(self.cfa(new_last_word_idx), name.to_string());

        new_last_word_idx
    }

    fn compile_cell(&mut self, value: i32) {
        let next_empty_space = self.here();
        self.write_i32(value, next_empty_space);
        self.set_here(next_empty_space + 4);
    }

    pub(crate) fn write_previous_idx(&mut self) {
        self.compile_cell(self.latest());
    }

    pub(crate) fn write_name(&mut self, name: &str, flags: u8) {
        let next_empty_space = self.here();

        let len = name.len();
        self.write_u8(len as u8 | flags, next_empty_space);
        let n = len.min(MAX_WORD_LEN);
        self.vm
            .write(next_empty_space as usize + 1, &name.as_bytes()[..n]);

        self.set_here(align(next_empty_space + n as i32 + 1));
    }

    fn write_codeword_builtin(&mut self) {
        self.compile_cell(self.here() + 4);
    }

    fn write_code(&mut self, code: &[u8]) {
        let next_empty_space = self.here();
        let n = code.len() as i32;

        self.vm.write(next_empty_space as usize, code);
        self.set_here(align(next_empty_space + n));
    }

    fn write_docol_addr(&mut self) {
        self.compile_cell(mmap::DOCOL as i32);
    }

    pub(crate) fn write_colon_def(&mut self, calls: &[&str]) {
        for call in calls {
            let c = if let Some(w) = self.find(call) {
                self.cfa(w)
            } else if let Ok(n) = call.parse() {
                n
            } else {
                panic!("{call}?")
            };
            self.compile_cell(c);
        }
    }

    pub fn print_4th_vars(&self) {
        let dsp = self.vm.read_i32(mmap::DSP);
        let rsp = self.vm.read_i32(mmap::RSP);
        let ic = self.vm.read_i32(mmap::IC);
        let here = self.vm.read_i32(mmap::HERE);
        let latest = self.vm.read_i32(mmap::LATEST);
        let a0 = self.vm.read_i32(mmap::A0);
        let base = self.vm.read_i32(mmap::BASE);
        let state = {
            if self.vm.read_i32(mmap::STATE) == INTERPRETING {
                "interpreting"
            } else {
                "compiling"
            }
        };
        println!(
            "DSP    0x{:08x} ({})\tRSP    0x{:08x} ({})",
            dsp, dsp, rsp, rsp
        );
        println!(
            "IC     0x{:08x} ({})  \tA0     0x{:08x} ({})",
            ic, ic, a0, a0
        );
        println!(
            "HERE   0x{:08x} ({})\tLATEST 0x{:08x} ({})",
            here, here, latest, latest
        );
        println!("STATE  {}             BASE              ({})", state, base);
        let word = if let Some(w) = self.vocabulary.get(&a0) {
            w
        } else {
            "unknown"
        };
        println!("WORD: {word}")
    }

    pub fn print_word(&self, idx: i32) {
        let prev_idx = self.read_i32(idx);
        let name_len = self.read_u8(idx + 4) & LEN_MASK;
        let mut name = String::new();
        let n = name_len.min(MAX_WORD_LEN as u8) as i32;

        for i in 0..n {
            let c = self.read_u8(idx + 5 + i) as char;
            name.push(c);
        }

        let code_field_addr = self.cfa(idx);
        let code_ptr = self.read_i32(code_field_addr);

        println!(
            "{name}\t@{idx}\n\tprev: {prev_idx}\n\tlen: {name_len}\n\tcfa: {code_field_addr}\n\tcode_ptr: {code_ptr}"
        );
    }

    pub fn print_dictionary(&self) {
        let mut idx = self.vm.read_i32(mmap::LATEST);

        while idx != 0 {
            let prev_idx = self.read_i32(idx);
            self.print_word(idx);
            idx = prev_idx;
        }
    }

    pub fn print_pstack(&self) {
        let top = self.vm.read_i32(mmap::DSP);
        let mut base = self.vm.read_i32(mmap::S0);

        print!("pstack: ");

        while (top + 4) <= base {
            let value = self.vm.read_i32((base) as usize);
            print!("{value} ");
            base -= 4;
        }
        println!();
    }

    pub fn print_rstack(&self) {
        let top = self.vm.read_i32(mmap::RSP);
        let mut base = self.vm.read_i32(mmap::R0);

        print!("rstack: ");

        while (top + 4) <= base {
            let value = self.vm.read_i32((base) as usize);
            print!("{value} ");
            base -= 4;
        }
        println!();
    }
    pub fn print_memory_dump(&self, from: usize) {
        let mut idx = from;
        let mut memory = vec![0; 16 * 16];
        self.vm.read(from, &mut memory);

        println!("\n=========================================");
        for i in memory.chunks(16) {
            println!(
                "{idx:0>5} {:02x}{:02x}{:02x}{:02x} {:02x}{:02x}{:02x}{:02x} {:02x}{:02x}{:02x}{:02x} {:02x}{:02x}{:02x}{:02x} | {}{}{}{} {}{}{}{} {}{}{}{} {}{}{}{}",
                i[0],
                i[1],
                i[2],
                i[3],
                i[4],
                i[5],
                i[6],
                i[7],
                i[8],
                i[9],
                i[10],
                i[11],
                i[12],
                i[13],
                i[14],
                i[15],
                print_c(i[0]),
                print_c(i[1]),
                print_c(i[2]),
                print_c(i[3]),
                print_c(i[4]),
                print_c(i[5]),
                print_c(i[6]),
                print_c(i[7]),
                print_c(i[8]),
                print_c(i[9]),
                print_c(i[10]),
                print_c(i[11]),
                print_c(i[12]),
                print_c(i[13]),
                print_c(i[14]),
                print_c(i[15]),
            );

            idx += 16;
        }
    }
}

fn print_c(c: u8) -> char {
    let ascii = c as char;
    if ascii.is_ascii() && ascii.is_ascii_graphic() {
        ascii
    } else {
        '.'
    }
}

fn write_str(vm: &mut VM, idx: usize, s: &str) {
    let len: i32 = s.len() as i32;
    vm.write_i32(len, idx);
    vm.write(idx + 4, s.as_bytes());
}

fn print_input_buffer(vm: &VM) {
    let input_buffer = vm.read_i32(mmap::INPUT_BUFFER);
    let input_buffer_idx = vm.read_i32(mmap::INPUT_BUFFER_IN);

    let len = vm.read_u8(input_buffer as usize) as i32;

    let s = make_string(vm, len, input_buffer + 1);

    let len2 = len - input_buffer_idx;
    let s2 = make_string(vm, len2, input_buffer + 1 + input_buffer_idx);

    println!(
        "len: {len} input_buffer: {input_buffer} input_buffer_idx: {input_buffer_idx} s: \'{s}\' parse area \'{s2}\'"
    );
}

// pub(crate) fn read_next_char(vm: &mut VM) -> Option<u8> {
//     let input_buffer = vm.read_i32(mmap::INPUT_BUFFER);
//     let input_buffer_idx = vm.read_i32(mmap::INPUT_BUFFER_IDX);
//     let len = vm.read_u8(input_buffer as usize);

//     if len == 0 || input_buffer_idx - input_buffer > len as i32 {
//         return None;
//     }

//     let c = vm.read_u8(input_buffer_idx as usize);
//     vm.write_i32(input_buffer_idx + 1, mmap::INPUT_BUFFER_IDX);
//     Some(c)
// }

pub(crate) fn read_next_char_in(vm: &mut VM) -> Option<u8> {
    let input_buffer = vm.read_i32(mmap::INPUT_BUFFER);
    let input_buffer_in = vm.read_i32(mmap::INPUT_BUFFER_IN);
    let len = vm.read_u8(input_buffer as usize);

    if len == 0 || input_buffer_in >= len as i32 {
        return None;
    }

    let c = vm.read_u8((input_buffer + input_buffer_in + 1) as usize);
    vm.write_i32(input_buffer_in + 1, mmap::INPUT_BUFFER_IN);
    Some(c)
}

pub(crate) fn fill_input_buffer(vm: &mut VM, s: &str) {
    let input_buf = vm.read_i32(mmap::INPUT_BUFFER);

    let len = s.len();
    vm.write_u8(len as u8, input_buf as usize);
    vm.write(input_buf as usize + 1, s.as_bytes());

    vm.write_i32(0, mmap::INPUT_BUFFER_IN);
}
