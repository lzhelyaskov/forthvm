use crate::mmap::{self, STATE};
use crate::{ForthVM, HIDDEN, IMMEDIATE, LEN_MASK, MAX_WORD_LEN, VmConfig};
use toyvm::opcode;

#[test]
fn test_input_buf() {
    let conf = VmConfig {
        memory_size_bytes: 0x10000,
        parameter_stack_size_cells: 256,
        return_stack_size_cells: 256,
        call_stack_size_cells: 256,
        locals_stack_size_cells: 256,
    };
    let mut vm = ForthVM::from_config(conf);
    let s = "testing input buffer!\n".to_string();
    vm.fill_input_buffer(&s);
    let mut result = String::new();
    while let Some(c) = vm.read_next_char() {
        // dbg!(c as char);
        result.push(c as char);
    }

    assert_eq!(result, s)
}

#[test]
fn test_write_previous_idx() {
    let conf = VmConfig {
        memory_size_bytes: 0x10000,
        parameter_stack_size_cells: 256,
        return_stack_size_cells: 256,
        call_stack_size_cells: 256,
        locals_stack_size_cells: 256,
    };
    let mut vm = ForthVM::from_config(conf);

    vm.write_i32(47, mmap::LATEST as i32);
    let here = vm.here();

    vm.write_previous_idx();

    let result = vm.read_i32(here);
    assert_eq!(result, 47);
    assert_eq!(vm.here(), here + 4);
}

#[test]
fn test_write_name_ex() {
    let conf = VmConfig {
        memory_size_bytes: 0x10000,
        parameter_stack_size_cells: 256,
        return_stack_size_cells: 256,
        call_stack_size_cells: 256,
        locals_stack_size_cells: 256,
    };
    let mut vm = ForthVM::from_config(conf);

    let here = vm.here();

    vm.write_name("foobarooo", HIDDEN | IMMEDIATE);

    let len_byte = vm.read_u8(here);

    assert_eq!(len_byte & HIDDEN, HIDDEN, "HIDDEN flag set");
    assert_eq!(len_byte & IMMEDIATE, IMMEDIATE, "IMMEDIATE flag set");
    assert_eq!(len_byte & LEN_MASK, 9);

    assert_eq!(vm.here(), here + 8);

    for (i, b) in "foobarooo".as_bytes().iter().take(MAX_WORD_LEN).enumerate() {
        assert_eq!(*b, vm.read_u8(here + 1 + i as i32));
    }
}

#[test]
fn test_write_col_def() {
    let conf = VmConfig {
        memory_size_bytes: 0x10000,
        parameter_stack_size_cells: 256,
        return_stack_size_cells: 256,
        call_stack_size_cells: 256,
        locals_stack_size_cells: 256,
    };
    let mut vm = ForthVM::from_config(conf);

    let drop = vm.builtin("drop", &[opcode::DROP, opcode::NEXT]);
    let swap = vm.builtin("swap", &[opcode::SWAP, opcode::NEXT]);
    let dup = vm.builtin("dup", &[opcode::DUP, opcode::NEXT]);
    let bye = vm.builtin("bye", &[opcode::END]);

    let here = vm.here();

    vm.write_colon_def(&["drop", "swap", "dup", "bye"]);

    assert_eq!(vm.here(), here + 16);

    assert_eq!(vm.cfa(drop), vm.read_i32(here));
    assert_eq!(vm.cfa(swap), vm.read_i32(here + 4));
    assert_eq!(vm.cfa(dup), vm.read_i32(here + 8));
    assert_eq!(vm.cfa(bye), vm.read_i32(here + 12));
}

#[test]
fn test_builtin() {
    let conf = VmConfig {
        memory_size_bytes: 0x10000,
        parameter_stack_size_cells: 256,
        return_stack_size_cells: 256,
        call_stack_size_cells: 256,
        locals_stack_size_cells: 256,
    };
    let mut vm = ForthVM::from_config(conf);
    let state = STATE.to_ne_bytes();
    let here = vm.here();

    vm.builtin(
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

    assert_eq!(here, vm.latest());
    assert_eq!(vm.read_i32(vm.latest()), 0, "previous word idx");

    let word_len = vm.read_u8(vm.latest() + 4);
    assert_eq!(word_len as usize, "state".len());

    for (i, c) in "state".as_bytes().iter().enumerate() {
        assert_eq!(*c, vm.read_u8(vm.latest() + 5 + i as i32));
    }

    assert_eq!(vm.read_i32(here + 4 + 8), here + 4 + 8 + 4);

    assert_eq!(vm.here(), here + 4 + 8 + 4 + 6 + 2, "new here");
}

#[test]
fn test_find() {
    let conf = VmConfig {
        memory_size_bytes: 0x10000,
        parameter_stack_size_cells: 256,
        return_stack_size_cells: 256,
        call_stack_size_cells: 256,
        locals_stack_size_cells: 256,
    };
    let mut vm = ForthVM::from_config(conf);
    let state = STATE.to_ne_bytes();
    let here = vm.here();

    vm.builtin(
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

    vm.builtin("drop", &[opcode::DROP, opcode::NEXT]);
    vm.builtin("swap", &[opcode::SWAP, opcode::NEXT]);
    vm.builtin("dup", &[opcode::DUP, opcode::NEXT]);
    vm.builtin("bye", &[opcode::END]);

    let idx = vm.find("swap");
    assert!(idx.is_some());

    let state_idx = vm.find("state");

    assert_eq!(state_idx, Some(here));
}

#[test]
fn test_colon_def() {
    let conf = VmConfig {
        memory_size_bytes: 0x10000,
        parameter_stack_size_cells: 256,
        return_stack_size_cells: 256,
        call_stack_size_cells: 256,
        locals_stack_size_cells: 256,
    };
    let mut vm = ForthVM::from_config(conf);

    vm.builtin("drop", &[opcode::DROP, opcode::NEXT]);
    vm.builtin("swap", &[opcode::SWAP, opcode::NEXT]);
    vm.builtin("dup", &[opcode::DUP, opcode::NEXT]);
    let bye_idx = vm.builtin("bye", &[opcode::END]);
    assert_eq!(bye_idx, vm.latest(), "bye_idx is latest");
    let here = vm.here();
    let idx = vm.colon_def("foobar", &["drop", "swap", "bye"]);

    assert_eq!(here, idx);
    assert_eq!(idx, vm.latest(), "latest is idx");

    assert_eq!(
        bye_idx,
        vm.read_i32(vm.latest()),
        "latest points to bye_idx"
    );

    let result = vm.find("foobar");
    assert!(result.is_some());

    // vm.add_col_word_ex("+", )
}
