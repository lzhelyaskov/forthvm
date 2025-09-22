use toyvm::{
    VM,
    opcode::{self, NEXT},
};

use crate::{
    ForthVM, HIDDEN, IMMEDIATE, LEN_MASK, MAX_WORD_LEN, align,
    forthvm::{OVER, ROT, fill_input_buffer, read_next_char},
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
                opcode::NEXT,
            ],
        );
        self.builtin(
            "here",
            &[
                opcode::I32_CONST,
                here[0],
                here[1],
                here[2],
                here[3],
                opcode::NEXT,
            ],
        );

        self.builtin(
            "latest",
            &[
                opcode::I32_CONST,
                latest[0],
                latest[1],
                latest[2],
                latest[3],
                opcode::NEXT,
            ],
        );
        self.builtin(
            "dsp",
            &[
                opcode::I32_CONST,
                dsp[0],
                dsp[1],
                dsp[2],
                dsp[3],
                opcode::NEXT,
            ],
        );

        self.builtin(
            "rsp",
            &[
                opcode::I32_CONST,
                rsp[0],
                rsp[1],
                rsp[2],
                rsp[3],
                opcode::NEXT,
            ],
        );

        self.builtin(
            "s0",
            &[opcode::I32_CONST, s0[0], s0[1], s0[2], s0[3], opcode::NEXT],
        );

        self.builtin(
            "r0",
            &[opcode::I32_CONST, r0[0], r0[1], r0[2], r0[3], opcode::NEXT],
        );

        self.builtin(
            "base",
            &[
                opcode::I32_CONST,
                base[0],
                base[1],
                base[2],
                base[3],
                opcode::NEXT,
            ],
        );

        self.builtin(
            "DOCOL",
            &[
                opcode::I32_CONST,
                docol[0],
                docol[1],
                docol[2],
                docol[3],
                opcode::NEXT,
            ],
        );

        self.builtin(
            "F_IMMED",
            &[opcode::I32_CONST, IMMEDIATE, 0, 0, 0, opcode::NEXT],
        );
        self.builtin(
            "F_HIDDEN",
            &[opcode::I32_CONST, HIDDEN, 0, 0, 0, opcode::NEXT],
        );
        self.builtin(
            "F_LENMASK",
            &[opcode::I32_CONST, LEN_MASK, 0, 0, 0, opcode::NEXT],
        );

        self.builtin("true", &[opcode::I32_CONST, 1, 0, 0, 0, opcode::NEXT]);
        self.builtin("false", &[opcode::I32_CONST, 0, 0, 0, 0, opcode::NEXT]);
        self.builtin("not", &[opcode::EQZ, opcode::NEXT]);

        self.builtin("drop", &[opcode::DROP, opcode::NEXT]);
        self.builtin("2drop", &[opcode::DROP, opcode::DROP, opcode::NEXT]);
        self.builtin("swap", &[opcode::SWAP, opcode::NEXT]);
        self.builtin("dup", &[opcode::DUP, opcode::NEXT]);
        self.builtin("2dup", &[OVER, OVER, opcode::NEXT]);
        self.builtin(
            "?dup",
            &[
                opcode::DUP,
                opcode::JZI,  // 0
                5,            // 1
                0,            // 2
                0,            // 3
                0,            // 4
                opcode::DUP,  // 5
                opcode::NEXT, // 6
            ],
        );
        self.builtin("nip", &[opcode::SWAP, opcode::DROP, opcode::NEXT]);
        self.builtin("over", &[OVER, opcode::NEXT]);
        self.builtin("rot", &[ROT, opcode::NEXT]);
        self.builtin("tuck", &[opcode::SWAP, OVER, NEXT]);
        self.builtin("+", &[opcode::ADD, opcode::NEXT]);
        self.builtin("1+", &[opcode::INC, opcode::NEXT]);
        self.builtin("1-", &[opcode::DEC, opcode::NEXT]);
        self.builtin(
            "4+",
            &[opcode::I32_CONST, 4, 0, 0, 0, opcode::ADD, opcode::NEXT],
        );
        self.builtin(
            "4-",
            &[opcode::I32_CONST, 4, 0, 0, 0, opcode::SUB, opcode::NEXT],
        );

        self.builtin("-", &[opcode::SUB, opcode::NEXT]);
        self.builtin("*", &[opcode::MUL, opcode::NEXT]);
        self.builtin("/", &[opcode::DIV_S, opcode::NEXT]);
        self.builtin("mod", &[opcode::MOD_S, opcode::NEXT]);
        self.builtin("=", &[opcode::EQ, opcode::NEXT]);
        self.builtin("0=", &[opcode::EQZ, opcode::NEXT]);
        self.builtin("<>", &[opcode::EQ, opcode::EQZ, opcode::NEXT]);
        self.builtin("<", &[opcode::LT_S, opcode::NEXT]);
        self.builtin(">", &[opcode::GT_S, opcode::NEXT]);
        self.builtin("<=", &[opcode::LE_S, opcode::NEXT]);
        self.builtin(">=", &[opcode::GE_S, opcode::NEXT]);
        self.builtin("min", &[opcode::MIN, opcode::NEXT]);
        self.builtin("max", &[opcode::MAX, opcode::NEXT]);
        self.builtin("and", &[opcode::AND, opcode::NEXT]);
        self.builtin("or", &[opcode::OR, opcode::NEXT]);
        self.builtin("xor", &[opcode::XOR, opcode::NEXT]);
        self.builtin("invert", &[opcode::NOT, opcode::NEXT]);
        self.builtin("!", &[opcode::I32_STORE, opcode::NEXT]);
        self.builtin("@", &[opcode::I32_LOAD, opcode::NEXT]);
        self.builtin("c!", &[opcode::I32_STORE_8, opcode::NEXT]);
        self.builtin("c@", &[opcode::I32_LOAD_8, opcode::NEXT]);

        self.builtin(
            "negate",
            &[opcode::ZERO, opcode::SWAP, opcode::SUB, opcode::NEXT],
        );

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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
            ],
        );

        /*
            const ic    [ ic ]
            dup         [ ic ic ]
            load        [ ic *ic ]
            dup         [ ic *ic *ic]
            load        [ ic *ic offset ]
            add         [ ic (*ic offset) ]
            swap        [ (*ic offset) ic ]
            store       []
            next
        */
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
                opcode::NEXT,
            ],
        );

        let branch_code = (self.cfa(branch_adr) + 4).to_ne_bytes();

        /*
            brz code_of_branch
            const IC    [ ic ]
            dup         [ ic ic ]
            load        [ ic *ic ]
            const 4     [ ic *ic 4 ]
            add         [ ic (*ic + 4) ]
            swap        [ (*ic + 4) ic ]
            store       []
            next
        */
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
                opcode::NEXT,
            ],
        );

        /*
            const latest    [ latest ]
            load            [ adr ]
            const 4         [ adr 4 ]
            add             [ len_adr ]
            dup             [ len_adr  len_adr]
            i8.load         [ len_adr len ]
            const mask      [ len_adr len mask ]
            xor             [ len_adr new_len ]
            swap            [ new_len len_adr ]
            i8.store        []
            next            []
        */
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
                opcode::NEXT,
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
        self.vm_call("/mod", &div_mod);

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

fn _tell(vm: &VM, ptr: i32, n: i32) {
    let mut s = String::new();

    for i in 0..n {
        s.push(vm.read_u8((ptr + i) as usize) as char);
    }

    print!("{s}");
}

fn read_char(vm: &mut VM) {
    let (_, buf_ptr) = _word(vm);
    let c = vm.read_u8(buf_ptr as usize);
    vm.push_i32(c as i32);
}

fn div_mod(vm: &mut VM) {
    let b = vm.pop_i32();
    let a = vm.pop_i32();

    let q = a / b;
    let r = a % b;

    vm.push_i32(r);
    vm.push_i32(q);
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
