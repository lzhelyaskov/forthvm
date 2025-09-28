use toyvm::{VM, opcode};

use crate::{
    ForthVM, HIDDEN, IMMEDIATE, LEN_MASK, MAX_WORD_LEN, align,
    forthvm::{DIV_MOD, NEXT, OVER, ROT, fill_input_buffer, read_next_char},
    input_stream::{in_stream_from_stdin, in_stream_is_terminal, in_stream_read_line},
    mmap,
};

impl ForthVM {
    pub fn init_dictionary(&mut self) {
        // state = 0 => INTERPRETING
        // state = 1 => COMPILING
        let state = mmap::STATE.to_ne_bytes();
        let here = mmap::HERE.to_ne_bytes();
        let latest = mmap::LATEST.to_ne_bytes();
        let dsp = mmap::DSP.to_ne_bytes();
        let rsp = mmap::RSP.to_ne_bytes();
        let s0 = self.vm.read_i32(mmap::S0).to_ne_bytes();
        let r0 = self.vm.read_i32(mmap::R0).to_ne_bytes();
        let base = mmap::BASE.to_ne_bytes();
        let docol = mmap::DOCOL.to_ne_bytes();
        let ic = mmap::IC.to_ne_bytes();
        let a0 = mmap::A0.to_ne_bytes();
        let not_3 = (!3_i32).to_ne_bytes();

        self.builtin(
            "state",
            &[
                opcode::I32_CONST,
                state[0],
                state[1],
                state[2],
                state[3],
                NEXT,
            ],
        );
        self.builtin(
            "here",
            &[opcode::I32_CONST, here[0], here[1], here[2], here[3], NEXT],
        );

        self.builtin(
            "latest",
            &[
                opcode::I32_CONST,
                latest[0],
                latest[1],
                latest[2],
                latest[3],
                NEXT,
            ],
        );
        self.builtin(
            "dsp",
            &[opcode::I32_CONST, dsp[0], dsp[1], dsp[2], dsp[3], NEXT],
        );

        self.builtin(
            "rsp",
            &[opcode::I32_CONST, rsp[0], rsp[1], rsp[2], rsp[3], NEXT],
        );

        self.builtin("s0", &[opcode::I32_CONST, s0[0], s0[1], s0[2], s0[3], NEXT]);
        self.builtin("r0", &[opcode::I32_CONST, r0[0], r0[1], r0[2], r0[3], NEXT]);

        self.builtin(
            "base",
            &[opcode::I32_CONST, base[0], base[1], base[2], base[3], NEXT],
        );

        self.builtin(
            "DOCOL",
            &[
                opcode::I32_CONST,
                docol[0],
                docol[1],
                docol[2],
                docol[3],
                NEXT,
            ],
        );

        self.builtin("F_IMMED", &[opcode::I32_CONST, IMMEDIATE, 0, 0, 0, NEXT]);
        self.builtin("F_HIDDEN", &[opcode::I32_CONST, HIDDEN, 0, 0, 0, NEXT]);
        self.builtin("F_LENMASK", &[opcode::I32_CONST, LEN_MASK, 0, 0, 0, NEXT]);

        self.builtin("true", &[opcode::I32_CONST, 1, 0, 0, 0, NEXT]);
        self.builtin("false", &[opcode::I32_CONST, 0, 0, 0, 0, NEXT]);
        self.builtin("not", &[opcode::EQZ, NEXT]);

        self.builtin("drop", &[opcode::DROP, NEXT]);
        self.builtin("2drop", &[opcode::DROP, opcode::DROP, NEXT]);
        self.builtin("swap", &[opcode::SWAP, NEXT]);
        self.builtin("dup", &[opcode::DUP, NEXT]);
        self.builtin("2dup", &[OVER, OVER, NEXT]);
        self.builtin(
            "?dup",
            &[
                opcode::DUP,
                opcode::JZI, // 0
                5,           // 1
                0,           // 2
                0,           // 3
                0,           // 4
                opcode::DUP, // 5
                NEXT,        // 6
            ],
        );
        self.builtin("nip", &[opcode::SWAP, opcode::DROP, NEXT]);
        self.builtin("over", &[OVER, NEXT]);
        // ( a b c -- b c a )
        self.builtin("rot", &[ROT, NEXT]);

        // ( a b c -- c a b)
        self.builtin("-rot", &[ROT, ROT, NEXT]);
        self.builtin("tuck", &[opcode::SWAP, OVER, NEXT]);
        self.builtin("+", &[opcode::ADD, NEXT]);
        self.builtin("1+", &[opcode::INC, NEXT]);
        self.builtin("1-", &[opcode::DEC, NEXT]);
        self.builtin("4+", &[opcode::I32_CONST, 4, 0, 0, 0, opcode::ADD, NEXT]);
        self.builtin("4-", &[opcode::I32_CONST, 4, 0, 0, 0, opcode::SUB, NEXT]);

        self.builtin("-", &[opcode::SUB, NEXT]);
        self.builtin("*", &[opcode::MUL, NEXT]);
        self.builtin("/", &[opcode::DIV_S, NEXT]);
        self.builtin("mod", &[opcode::MOD_S, NEXT]);
        self.builtin("/mod", &[DIV_MOD, NEXT]);
        self.builtin("=", &[opcode::EQ, NEXT]);
        self.builtin("0=", &[opcode::EQZ, NEXT]);
        self.builtin("0<", &[opcode::ZERO, opcode::LT_S, NEXT]);
        self.builtin("0>", &[opcode::ZERO, opcode::GT_S, NEXT]);
        self.builtin("<>", &[opcode::EQ, opcode::EQZ, NEXT]);
        self.builtin("<", &[opcode::LT_S, NEXT]);
        self.builtin(">", &[opcode::GT_S, NEXT]);
        self.builtin("u<", &[opcode::LT_U, NEXT]);
        self.builtin("u>", &[opcode::GT_U, NEXT]);
        self.builtin("<=", &[opcode::LE_S, NEXT]);
        self.builtin(">=", &[opcode::GE_S, NEXT]);
        self.builtin("min", &[opcode::MIN, NEXT]);
        self.builtin("max", &[opcode::MAX, NEXT]);
        self.builtin("and", &[opcode::AND, NEXT]);
        self.builtin("or", &[opcode::OR, NEXT]);
        self.builtin("xor", &[opcode::XOR, NEXT]);
        self.builtin("invert", &[opcode::NOT, NEXT]);
        self.builtin("!", &[opcode::I32_STORE, NEXT]);
        // ( n addr -- )
        self.builtin(
            "+!",
            &[
                opcode::DUP,       // ( n addr addr )
                opcode::I32_LOAD,  // ( n addr value )
                ROT,               // ( addr value n )
                opcode::ADD,       // ( addr value )
                opcode::SWAP,      // ( value addr )
                opcode::I32_STORE, // ( )
                NEXT,
            ],
        );
        self.builtin("@", &[opcode::I32_LOAD, NEXT]);
        self.builtin("c!", &[opcode::I32_STORE_8, NEXT]);
        self.builtin("c@", &[opcode::I32_LOAD_8, NEXT]);

        self.builtin("negate", &[opcode::ZERO, opcode::SWAP, opcode::SUB, NEXT]);

        self.builtin("bye", &[opcode::END]);
        self.builtin(
            "execute",
            &[
                opcode::DUP,
                opcode::I32_CONST,
                a0[0],
                a0[1],
                a0[2],
                a0[3],
                opcode::I32_STORE,
                opcode::I32_LOAD,
                opcode::BR,
            ],
        );

        self.builtin(
            "lit",
            &[
                opcode::I32_CONST,
                ic[0],
                ic[1],
                ic[2],
                ic[3],
                opcode::I32_LOAD,
                opcode::DUP,
                opcode::I32_LOAD,
                opcode::SWAP,
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                opcode::I32_CONST,
                ic[0],
                ic[1],
                ic[2],
                ic[3],
                opcode::I32_STORE,
                NEXT,
            ],
        );
        /*
            ic points to string.len
            ic + 4 points to first char
            ( -- c-addr len)
             // (idx + 3) & !3
        */

        self.builtin(
            "litstring",
            &[
                opcode::I32_CONST,
                ic[0],
                ic[1],
                ic[2],
                ic[3],
                opcode::I32_LOAD, // ( ic )
                opcode::DUP,      // ( ic ic )
                opcode::DUP,      // ( ic ic ic )
                opcode::I32_LOAD, // ( ic ic len )
                opcode::SWAP,     // ( ic len ic)
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                // opcode::I32_LOAD, // ( ic len c-addr )
                opcode::SWAP, // ( ic c-addr len )
                ROT,          // ( c-addr len ic )
                OVER,         // ( c-addr len ic len )
                opcode::ADD,  // ( c-addr len ic+len )
                opcode::I32_CONST,
                7,
                0,
                0,
                0,
                opcode::ADD, // ( c-addr len ic+len+3)
                opcode::I32_CONST,
                not_3[0],
                not_3[1],
                not_3[2],
                not_3[3],
                opcode::AND,
                opcode::I32_CONST,
                ic[0],
                ic[1],
                ic[2],
                ic[3],
                opcode::I32_STORE,
                NEXT,
            ],
        );

        self.builtin(
            "exit",
            &[
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::I32_LOAD,
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD, // adjust rtop value
                opcode::DUP,
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::I32_STORE,
                opcode::I32_LOAD,
                opcode::I32_CONST,
                ic[0],
                ic[1],
                ic[2],
                ic[3],
                opcode::I32_STORE,
                NEXT,
            ],
        );

        self.builtin(
            ">r",
            &[
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::I32_LOAD,
                opcode::I32_STORE,
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::DUP,
                opcode::I32_LOAD,
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::SUB,
                opcode::SWAP,
                opcode::I32_STORE,
                NEXT,
            ],
        );

        self.builtin(
            "r>",
            &[
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::I32_LOAD,
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                opcode::DUP,
                opcode::I32_LOAD,
                opcode::SWAP,
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::I32_STORE,
                NEXT,
            ],
        );
        self.builtin(
            "rdrop",
            &[
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::DUP,
                opcode::I32_LOAD,
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                opcode::SWAP,
                opcode::I32_STORE,
                NEXT,
            ],
        );
        self.builtin_ex(
            "rsp!",
            0,
            &[
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::I32_STORE,
                NEXT,
            ],
        );

        self.builtin_ex(
            "rsp@",
            0,
            &[
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::I32_LOAD,
                NEXT,
            ],
        );

        // TODO: write tests for dsp! , dsp@
        self.builtin_ex(
            "dsp!",
            0,
            &[
                opcode::I32_CONST,
                dsp[0],
                dsp[1],
                dsp[2],
                dsp[3],
                opcode::I32_STORE,
                NEXT,
            ],
        );

        self.builtin_ex(
            "dsp@",
            0,
            &[
                opcode::I32_CONST,
                dsp[0],
                dsp[1],
                dsp[2],
                dsp[3],
                opcode::I32_LOAD,
                NEXT,
            ],
        );

        let branch_adr = self.builtin(
            "branch",
            &[
                opcode::I32_CONST,
                ic[0],
                ic[1],
                ic[2],
                ic[3],
                opcode::DUP,
                opcode::I32_LOAD,
                opcode::DUP,
                opcode::I32_LOAD,
                opcode::ADD,
                opcode::SWAP,
                opcode::I32_STORE,
                NEXT,
            ],
        );

        let branch_code = (self.cfa(branch_adr) + 4).to_ne_bytes();

        self.builtin(
            "0branch",
            &[
                opcode::BRZI,
                branch_code[0],
                branch_code[1],
                branch_code[2],
                branch_code[3],
                opcode::I32_CONST,
                ic[0],
                ic[1],
                ic[2],
                ic[3],
                opcode::DUP,
                opcode::I32_LOAD,
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                opcode::SWAP,
                opcode::I32_STORE,
                NEXT,
            ],
        );

        self.builtin_ex(
            "immediate",
            IMMEDIATE,
            &[
                opcode::I32_CONST,
                latest[0],
                latest[1],
                latest[2],
                latest[3],
                opcode::I32_LOAD,
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                opcode::DUP,
                opcode::I32_LOAD_8,
                opcode::I32_CONST,
                IMMEDIATE,
                0,
                0,
                0,
                opcode::XOR,
                opcode::SWAP,
                opcode::I32_STORE_8,
                NEXT,
            ],
        );

        // TODO: right now same as "lit". redefine in terms of word find >cfa
        self.builtin_ex(
            "'",
            0,
            &[
                opcode::I32_CONST,
                ic[0],
                ic[1],
                ic[2],
                ic[3],
                opcode::I32_LOAD,
                opcode::DUP,
                opcode::I32_LOAD,
                opcode::SWAP,
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                opcode::I32_CONST,
                ic[0],
                ic[1],
                ic[2],
                ic[3],
                opcode::I32_STORE,
                NEXT,
            ],
        );

        // (idx + 3) & !3
        let mwl = (MAX_WORD_LEN as i32).to_ne_bytes();
        self.builtin(
            ">cfa",
            &[
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                opcode::DUP,        // [ (idx + 4) (idx + 4) ]
                opcode::I32_LOAD_8, // [ (idx + 4) len ]
                opcode::I32_CONST,
                LEN_MASK,
                0,
                0,
                0,           // [ (idx + 4) len len_mask ]
                opcode::AND, // [ (idx + 4) n ]
                opcode::I32_CONST,
                mwl[0],
                mwl[1],
                mwl[2],
                mwl[3],
                opcode::MIN,
                opcode::ADD, // // [ (idx + 4 + n) ]
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                opcode::I32_CONST,
                not_3[0],
                not_3[1],
                not_3[2],
                not_3[3],
                opcode::AND,
                NEXT,
            ],
        );

        self.builtin_ex(
            "[",
            IMMEDIATE,
            &[
                opcode::I32_CONST,
                0,
                0,
                0,
                0,
                opcode::I32_CONST,
                state[0],
                state[1],
                state[2],
                state[3],
                opcode::I32_STORE,
                NEXT,
            ],
        );

        self.builtin_ex(
            "]",
            0,
            &[
                opcode::I32_CONST,
                1,
                0,
                0,
                0,
                opcode::I32_CONST,
                state[0],
                state[1],
                state[2],
                state[3],
                opcode::I32_STORE,
                NEXT,
            ],
        );
        self.builtin_ex(
            "hidden",
            0,
            &[
                opcode::I32_CONST,
                4,
                0,
                0,
                0,
                opcode::ADD,
                opcode::DUP,
                opcode::I32_LOAD_8,
                opcode::I32_CONST,
                HIDDEN,
                0,
                0,
                0,
                opcode::XOR,
                opcode::SWAP,
                opcode::I32_STORE_8,
                NEXT,
            ],
        );

        self.vm_call("key", &key);
        self.vm_call("word", &word);

        self.vm_call(".", &print_top_value);
        self.vm_call("emit", &emit_char);
        self.vm_call("tell", &tell);
        self.vm_call("find", &find);

        self.vm_call("number", &number);
        self.vm_call(",", &comma);
        self.vm_call("create", &create);
        self.vm_call("char", &read_char);

        self.colon_def(">dfa", &[">cfa", "4+", "exit"]);
        self.colon_def("hide", &["word", "find", "hidden", "exit"]);

        self.colon_def(
            ":",
            &[
                "word", "create", "DOCOL", ",", "latest", "@", "hidden", "]", "exit",
            ],
        );

        self.colon_def_ex(
            ";",
            IMMEDIATE,
            &["lit", "exit", ",", "latest", "@", "hidden", "[", "exit"],
        );

        self.colon_def(
            "interpret",
            &[
                "word",    // 0
                "2dup",    // 1
                "lit",     // 2
                "0",       // 3
                ">r",      // 4
                "find",    // 5
                "dup",     // 6
                "0branch", // 7
                "56",      // 8 "$NOT_IN_DICT" (22 - 8 * 4 => 56 )
                "swap",    // 9
                "drop",    // 10
                "swap",    // 11
                "drop",    // 12
                "dup",     // 13
                "4+",      // 14
                "c@",      // 15
                "F_IMMED", // 16
                "and",     // 17
                ">r",      // 18
                ">cfa",    // 19
                "branch",  // 20
                "36",      // 21 "$IS_EXECUTING?" (30 - 21) 9 * 4 => 36
                // $NOT_IN_DICT:
                "r>",      // 22
                "1+",      // 23
                ">r",      // 24
                ">r",      // 25
                "number",  // 26
                "0=",      // 27
                "0branch", // 28
                "92",      // 29 "$PARSE_ERROR" (52 - 29) 23 * 4 => 92
                // $IS_EXECUTING?:
                "state",   // 30
                "@",       // 31
                "r>",      // 32
                "0=",      // 33
                "and",     // 34
                "0=",      // 35
                "0branch", // 36
                "28",      // 37 "$COMPILE" (44 - 37) 7 * 4 => 28
                "r>",      // 48
                "0branch", // 39
                "8",       // 40 "$EXEC_NON_LIT" (42 - 40) 2 * 4 => 8
                "exit",    // 41
                // $EXEC_NON_LIT:
                "execute", // 42
                "exit",    // 43
                // $COMPILE:
                "r>",      // 44
                "0branch", // 45
                "16",      // 46 "$COMPILE2" (50 - 46) 4 * 4 => 16
                "'",       // 47
                "lit",     // 48
                ",",       // 49
                // $COMPILE2:
                ",",    // 50
                "exit", // 51
                // $PARSE_ERROR:
                "rdrop", // 52  dropping no longer needed temporary values
                "rdrop", // 53
                "lit",   // 54
                "63",    // 55
                "emit",  // 56
                "exit",  // quit here would be better, but how?
            ],
        );
        self.colon_def(
            "quit",
            &[
                "r0",        // 0
                "rsp!",      // 1
                "interpret", // 2
                "branch",    // 3
                "-16",       // 4 ( 0 - 4 ) * 4
            ],
        );

        #[cfg(feature = "fileio")]
        self.init_fileio_words();
    }

    #[cfg(feature = "fileio")]
    fn init_fileio_words(&mut self) {
        let ro = fileio::F_READ.to_ne_bytes();
        let wo = fileio::F_WRITE.to_ne_bytes();
        let rw = (fileio::F_READ | fileio::F_WRITE).to_ne_bytes();
        self.builtin(
            "r/o",
            &[opcode::I32_CONST, ro[0], ro[1], ro[2], ro[3], NEXT],
        );
        self.builtin(
            "w/o",
            &[opcode::I32_CONST, wo[0], wo[1], wo[2], wo[3], NEXT],
        );
        self.builtin(
            "r/w",
            &[opcode::I32_CONST, rw[0], rw[1], rw[2], rw[3], NEXT],
        );

        self.vm_call("include", &fileio::include);
        self.vm_call("included", &fileio::included);

        self.vm_call("file-open", &fileio::file_open);
        self.vm_call("file-create", &fileio::file_create);
        self.vm_call("file-close", &fileio::file_close);
        self.vm_call("file-read", &fileio::file_read);
        self.vm_call("file-write", &fileio::file_write);
    }
}

fn print_top_value(vm: &mut VM) {
    let value = vm.pop_i32();
    print!("{value} ");
}

fn emit_char(vm: &mut VM) {
    let value = vm.pop_i32() as u8 as char;
    print!("{value}");
}

// TODO: reimplement as builtin
fn comma(vm: &mut VM) {
    let value = vm.pop_i32();
    let here = vm.read_i32(mmap::HERE);
    vm.write_i32(value, here as usize);
    vm.write_i32(here + 4, mmap::HERE);
}

fn create(vm: &mut VM) {
    let len = vm.pop_i32();
    let ptr = vm.pop_i32();

    let mut here = vm.read_i32(mmap::HERE);
    let current = here;
    let latest = vm.read_i32(mmap::LATEST);

    vm.write_i32(latest, here as usize);
    here += 4;

    vm.write_u8(len as u8, here as usize);
    here += 1;

    let n = (len as usize).min(MAX_WORD_LEN);

    vm.memcopy(ptr as usize, here as usize, n);

    here = align(here + n as i32);

    vm.write_i32(here, mmap::HERE);
    vm.write_i32(current, mmap::LATEST);
}

// ( c-addr len -- addr )
fn find(vm: &mut VM) {
    let len = vm.pop_i32();
    let ptr = vm.pop_i32();
    let w = _find(vm, len, ptr);
    vm.push_i32(w);
}

fn _find(vm: &VM, len: i32, ptr: i32) -> i32 {
    let mut w = vm.read_i32(mmap::LATEST);

    while w != 0 {
        let next = vm.read_i32(w as usize);
        let len2 = (vm.read_u8(w as usize + 4) & (LEN_MASK | HIDDEN)) as i32;
        if len == len2 && vm.memcmp(ptr as usize, w as usize + 5, MAX_WORD_LEN.min(len as usize)) {
            break;
        }
        w = next;
    }
    w
}

fn number(vm: &mut VM) {
    let len = vm.pop_i32();
    let adr = vm.pop_i32();
    let (n, f) = _number(vm, len, adr);

    vm.push_i32(n);
    vm.push_i32(f);
}

fn _number(vm: &mut VM, len: i32, caddr: i32) -> (i32, i32) {
    if len == 0 {
        return (0, 0);
    }
    let mut n: i32 = 1; // bytes read
    let mut c = vm.read_u8(caddr as usize) as char;
    let is_negative = if c == '-' {
        if len == 1 {
            return (0, 0);
        }
        c = vm.read_u8((caddr + n) as usize) as char;
        n += 1;
        true
    } else {
        false
    };

    let base = vm.read_i32(mmap::BASE);

    let mut result = 0;
    while n <= len {
        if let Some(digit) = c.to_digit(base as u32) {
            result *= base;
            result += digit as i32;
        } else {
            return (0, len - n + 1);
        }
        c = vm.read_u8((caddr + n) as usize) as char;
        n += 1;
    }

    if is_negative {
        result = -result;
    }
    (result, 0)
}

fn key(vm: &mut VM) {
    let c = _key(vm);
    vm.push_i32(c as i32);
}

fn _key(vm: &mut VM) -> char {
    if let Some(c) = read_next_char(vm) {
        c as char
    } else {
        let mut line = String::new();

        if in_stream_is_terminal() {
            print_prompt();
        }

        let n = in_stream_read_line(&mut line);
        if n == 0 {
            // lets hope it was eof
            in_stream_from_stdin();
            return '\n';
        }

        fill_input_buffer(vm, &line);
        read_next_char(vm).unwrap() as char
    }
}

fn print_prompt() {
    print!("\n> ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

// ( -- c-addr len)
fn word(vm: &mut VM) {
    let (len, in_stream) = _word(vm);
    vm.push_i32(in_stream);
    vm.push_i32(len);
}

fn _word(vm: &mut VM) -> (i32, i32) {
    let mut c = skip_white_space(vm);

    let mut len = 0;
    let buf_ptr = vm.read_i32(mmap::IN_STREAM);
    while !c.is_ascii_whitespace() {
        vm.write_u8(c as u8, (buf_ptr + len) as usize);
        len += 1;
        c = _key(vm);
    }
    // _tell(vm, buf_ptr, len);
    (len, buf_ptr)
}

fn tell(vm: &mut VM) {
    let len = vm.pop_i32();
    let buf_ptr = vm.pop_i32();
    _tell(vm, buf_ptr, len);
}

fn _tell(vm: &VM, ptr: i32, len: i32) {
    let s = make_string(vm, len, ptr);
    print!("{s}");
}

fn read_char(vm: &mut VM) {
    let (_, buf_ptr) = _word(vm);
    let c = vm.read_u8(buf_ptr as usize);
    vm.push_i32(c as i32);
}

fn skip_white_space(vm: &mut VM) -> char {
    let mut c = _key(vm);
    loop {
        while c.is_ascii_whitespace() {
            c = _key(vm);
        }

        if c == '\\' {
            c = _key(vm);
            while c != '\n' {
                c = _key(vm);
            }
        } else {
            break c;
        }
    }
}

fn make_string(vm: &VM, len: i32, ptr: i32) -> String {
    let mut s = String::new();

    for i in 0..len {
        let c = vm.read_u8((ptr + i) as usize) as char;
        s.push(c);
    }

    s
}

#[cfg(feature = "fileio")]
mod fileio {
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Write},
        os::fd::{FromRawFd, IntoRawFd},
    };

    use toyvm::VM;

    use crate::{
        in_stream_from_file,
        init_dictionary::{_word, make_string},
    };

    pub(crate) const F_READ: i32 = 1;
    pub(crate) const F_WRITE: i32 = 2;

    pub(crate) fn include(vm: &mut VM) {
        let (len, ptr) = _word(vm);
        _included(vm, len, ptr);
    }

    pub(crate) fn included(vm: &mut VM) {
        let len = vm.pop_i32();
        let ptr = vm.pop_i32();
        _included(vm, len, ptr);
    }

    fn _included(vm: &mut VM, len: i32, ptr: i32) {
        let path = make_string(vm, len, ptr);

        match File::open(&path) {
            Ok(file) => {
                in_stream_from_file(file);
            }
            Err(err) => {
                println!("could not open '{path}'\n {:?}", err);
            }
        }
    }

    pub(crate) fn file_open(vm: &mut VM) {
        _file_open(vm, false);
    }
    pub(crate) fn file_create(vm: &mut VM) {
        _file_open(vm, true);
    }

    fn _file_open(vm: &mut VM, create: bool) {
        let flags = vm.pop_i32();
        let len = vm.pop_i32();
        let ptr = vm.pop_i32();
        let path = make_string(vm, len, ptr);
        let result = OpenOptions::new()
            .read(flags & F_READ == F_READ)
            .write(flags & F_WRITE == F_WRITE)
            .create(create)
            .open(path);

        match result {
            Ok(file) => {
                let fd = file.into_raw_fd();
                vm.push_i32(fd);
                vm.push_i32(0); // error code 0 => success
            }
            Err(err) => {
                vm.push_i32(0); // no fd
                vm.push_i32(err.kind() as i32 + 1); // signal error
            }
        }
    }

    pub(crate) fn file_close(vm: &mut VM) {
        let fd = vm.pop_i32();
        let file = unsafe { File::from_raw_fd(fd) };
        drop(file);
    }

    pub(crate) fn file_read(vm: &mut VM) {
        let fd = vm.pop_i32();
        let len = vm.pop_i32() as usize;
        let ptr = vm.pop_i32() as usize;
        let mut file = unsafe { File::from_raw_fd(fd) };
        let mem = vm.memory_ref_mut();
        let result = file.read(&mut mem[ptr..ptr + len]);

        std::mem::forget(file);

        match result {
            Ok(u) => {
                vm.push_i32(u as i32);
                vm.push_i32(0);
            }
            Err(err) => {
                vm.push_i32(0);
                vm.push_i32(err.kind() as i32 + 1); // signal error
            }
        }
    }

    pub(crate) fn file_write(vm: &mut VM) {
        let fd = vm.pop_i32();
        let len = vm.pop_i32() as usize;
        let ptr = vm.pop_i32() as usize;
        let mut file = unsafe { File::from_raw_fd(fd) };
        let mem = vm.memory_ref();
        let result = file.write(&mem[ptr..ptr + len]);

        std::mem::forget(file);

        match result {
            Ok(u) => {
                vm.push_i32(u as i32);
                vm.push_i32(0);
            }
            Err(err) => {
                vm.push_i32(0);
                vm.push_i32(err.kind() as i32 + 1); // signal error
            }
        }
    }
}
