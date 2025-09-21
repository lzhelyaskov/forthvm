use crate::{
    COMPILING, FALSE, ForthVM, HIDDEN, IMMEDIATE, INTERPRETING, LEN_MASK, TRUE, VmConfig, mmap,
};

fn create_vm() -> ForthVM {
    let conf = VmConfig {
        memory_size_bytes: 0x10000,
        parameter_stack_size_cells: 256,
        return_stack_size_cells: 256,
        call_stack_size_cells: 256,
        locals_stack_size_cells: 256,
    };
    let mut vm = ForthVM::from_config(conf);
    vm.init_dictionary();
    vm
}
// + - * / mod
#[test]
fn test_arithmetic() {
    // TODO: abs

    let mut vm = create_vm();

    let test_add = vm.colon_def("test_add", &["+", "bye"]);
    let test_sub = vm.colon_def("test_sub", &["-", "bye"]);
    let test_mul = vm.colon_def("test_mul", &["*", "bye"]);
    let test_div = vm.colon_def("test_div", &["/", "bye"]);
    let test_mod = vm.colon_def("test_mod", &["mod", "bye"]);

    let test_min = vm.colon_def("test_min", &["min", "bye"]);
    let test_max = vm.colon_def("test_max", &["max", "bye"]);
    let test_neg = vm.colon_def("test_neg", &["negate", "bye"]);
    // let test_abs = dict.add_col_word_ex(&mut vm, "test_abs", &["abs", "bye"]);

    let test_add_one = vm.colon_def("test_add_one", &["1+", "bye"]);
    let test_add_four = vm.colon_def("test_add_four", &["4+", "bye"]);

    let test_sub_one = vm.colon_def("test_sub_one", &["1-", "bye"]);
    let test_sub_four = vm.colon_def("test_sub_four", &["4-", "bye"]);

    vm.push_i32(3);
    vm.push_i32(7);

    vm.run_word(test_add as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 10, "addition");

    vm.push_i32(10);
    vm.push_i32(5);

    vm.run_word(test_sub as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 5, "substraction 10 - 5");

    vm.push_i32(5);
    vm.push_i32(10);

    vm.run_word(test_sub as usize);
    let result = vm.pop_i32();

    assert_eq!(result, -5, "substraction 5 - 10");

    vm.push_i32(5);
    vm.push_i32(10);

    vm.run_word(test_mul as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 50, "multiplication");

    vm.push_i32(10);
    vm.push_i32(2);

    vm.run_word(test_div as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 5, "division");

    vm.push_i32(7);
    vm.push_i32(4);

    vm.run_word(test_mod as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 3, "modulo");

    vm.push_i32(7);
    vm.push_i32(4);

    vm.run_word(test_min as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 4, "min");

    vm.push_i32(7);
    vm.push_i32(4);

    vm.run_word(test_max as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 7, "max");

    vm.push_i32(-1);

    vm.run_word(test_neg as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 1, "neg -1");

    vm.push_i32(2);

    vm.run_word(test_neg as usize);
    let result = vm.pop_i32();

    assert_eq!(result, -2, "neg 2");

    vm.push_i32(9);
    vm.run_word(test_add_one as usize);
    let result = vm.pop_i32();
    assert_eq!(result, 10);

    vm.push_i32(9);
    vm.run_word(test_sub_one as usize);
    let result = vm.pop_i32();
    assert_eq!(result, 8);

    vm.push_i32(16);
    vm.run_word(test_sub_four as usize);
    let result = vm.pop_i32();
    assert_eq!(result, 12);

    vm.push_i32(16);
    vm.run_word(test_add_four as usize);
    let result = vm.pop_i32();
    assert_eq!(result, 20);
    /*
           vm.push_i32(-1);

           run_word(&mut vm, test_abs as usize);
           let result = vm.pop_i32();

           assert_eq!(result, 1, "abs -1");

           vm.push_i32(2);

           run_word(&mut vm, test_abs as usize);
           let result = vm.pop_i32();

           assert_eq!(result, 2, "abs 2");
    */
}

#[test]
fn test_logic() {
    // and or xor invert
    let mut vm = create_vm();

    let test_and = vm.colon_def("test_and", &["and", "bye"]);
    let test_or = vm.colon_def("test_or", &["or", "bye"]);
    let test_xor = vm.colon_def("test_xor", &["xor", "bye"]);
    let test_invert = vm.colon_def("test_invert", &["invert", "bye"]);

    vm.push_i32(0b0101);
    vm.push_i32(0b0110);

    vm.run_word(test_and as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 0b0100, "and");

    vm.push_i32(0b0101);
    vm.push_i32(0b0110);

    vm.run_word(test_or as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 0b0111, "or");

    vm.push_i32(0b0101);
    vm.push_i32(0b0110);

    vm.run_word(test_xor as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 0b0011, "xor");

    vm.push_i32(0b1010);

    vm.run_word(test_invert as usize);
    let result = vm.pop_i32() as u32;

    assert_eq!(result, 0b1111_1111_1111_1111_1111_1111_1111_0101, "invert");
}

#[test]
fn test_lit() {
    let mut vm = create_vm();

    let test_lit = vm.colon_def("test_lit", &["lit", "123", "bye"]);

    vm.run_word(test_lit as usize);

    let result = vm.pop_i32();
    assert_eq!(result, 123);
}

#[test]
fn test_branch() {
    let mut vm = create_vm();

    let test_branch = vm.colon_def(
        "test_branch",
        &[
            "branch", // 0
            "12",     // 1 ( (4 - 1) * 3 = 12 )
            "lit",    // 2
            "4711",   // 3
            "+",      // 4
            "bye",    // 5
        ],
    );

    vm.push_i32(42);
    vm.push_i32(58);

    vm.run_word(test_branch as usize);

    let result = vm.pop_i32();
    assert_eq!(result, 100);
}

#[test]
fn test_0branch() {
    let mut vm = create_vm();

    let test_0branch = vm.colon_def(
        "test_0branch",
        &["0branch", "12", "lit", "256", "lit", "512", "bye"],
    );

    vm.push_i32(0);

    vm.run_word(test_0branch as usize);

    assert_eq!(vm.pstack_depth(), 1);
    let result = vm.pop_i32();
    assert_eq!(result, 512);

    vm.push_i32(1);

    vm.run_word(test_0branch as usize);
    assert_eq!(vm.pstack_depth(), 2);

    let first = vm.pop_i32();
    let second = vm.pop_i32();

    assert_eq!(first, 512);
    assert_eq!(second, 256);
}

#[test]
fn test_param_stack() {
    let mut vm = create_vm();

    let test_dup = vm.colon_def("test_dup", &["dup", "bye"]);
    let test_swap = vm.colon_def("test_swap", &["swap", "bye"]);
    let test_drop = vm.colon_def("test_drop", &["drop", "bye"]);
    let test_2dup = vm.colon_def("test_2dup", &["2dup", "bye"]);

    // TODO: rot pick roll stack?
    // let test_over = dict.add_col_word_ex(&mut vm, "test_over", &["over", "bye"]);
    // let test_rot = dict.add_col_word_ex(&mut vm, "test_invert", &["rot", "bye"]);

    vm.push_i32(42);

    vm.run_word(test_dup as usize);
    let a = vm.pop_i32();
    let b = vm.pop_i32();

    assert_eq!(a, 42, "dup");
    assert_eq!(a, b, "dup");

    vm.push_i32(42);
    vm.push_i32(4711);

    vm.run_word(test_swap as usize);
    let a = vm.pop_i32();
    let b = vm.pop_i32();

    assert_eq!(a, 42, "swap");
    assert_eq!(b, 4711, "swap");

    vm.push_i32(42);
    vm.push_i32(4711);

    vm.run_word(test_drop as usize);
    let a = vm.pop_i32();

    assert_eq!(a, 42, "drop");

    vm.push_i32(55);
    vm.push_i32(66);

    // ( 55 66 -- 55 66 55 66)
    vm.run_word(test_2dup as usize);
    // vm.print_pstack();
    let a = vm.pop_i32();
    let b = vm.pop_i32();
    let c = vm.pop_i32();
    let d = vm.pop_i32();
    assert_eq!(a, 66);
    assert_eq!(b, 55);
    assert_eq!(c, 66);
    assert_eq!(d, 55);
}

#[test]
fn test_return_stack() {
    let mut vm = create_vm();

    let test_pushr = vm.colon_def("test_pushr", &[">r", "bye"]);
    let test_popr = vm.colon_def("test_popr", &[">r", "r>", "bye"]);

    vm.push_i32(42);

    assert_eq!(0, vm.rstack_depth());

    vm.run_word(test_pushr as usize);
    let rtop = vm.read_i32(mmap::RSP as i32);
    let value = vm.read_i32(rtop as i32 + 4);
    assert_eq!(value, 42, ">r");

    vm.push_i32(69);
    vm.run_word(test_popr as usize);
    let value = vm.pop_i32();
    assert_eq!(value, 69, "r>");
}

#[test]
fn test_comparison() {
    let mut vm = create_vm();

    let test_eq = vm.colon_def("test_eq", &["=", "bye"]);
    let test_eqz = vm.colon_def("test_eqz", &["0=", "bye"]);
    let test_gt = vm.colon_def("test_gt", &[">", "bye"]);
    let test_lt = vm.colon_def("test_lt", &["<", "bye"]);
    let test_ge = vm.colon_def("test_ge", &[">=", "bye"]);
    let test_le = vm.colon_def("test_le", &["<=", "bye"]);
    let test_neq = vm.colon_def("test_neq", &["<>", "bye"]);

    vm.push_i32(42);
    vm.push_i32(42);
    vm.run_word(test_eq as usize);
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, "= (eq) 1");

    vm.push_i32(42);
    vm.push_i32(69);
    vm.run_word(test_eq as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, "= (eq) 2");

    vm.push_i32(42);
    vm.push_i32(42);
    vm.run_word(test_neq as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, "<> (neq) 1");

    vm.push_i32(42);
    vm.push_i32(69);
    vm.run_word(test_neq as usize);
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, "<> (neq) 2");

    vm.push_i32(69);
    vm.run_word(test_eqz as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, "0= (eqz)");

    vm.push_i32(0);
    vm.run_word(test_eqz as usize);
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, "0= (eqz)");

    vm.push_i32(69);
    vm.push_i32(42);
    vm.run_word(test_gt as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, "> (gt) 1");

    vm.push_i32(42);
    vm.push_i32(69);
    vm.run_word(test_gt as usize);
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, "> (gt) 2");

    vm.push_i32(42);
    vm.push_i32(42);
    vm.run_word(test_gt as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, "> (gt) 3");

    vm.push_i32(42);
    vm.push_i32(42);
    vm.run_word(test_lt as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, "< (lt) 1");

    vm.push_i32(69);
    vm.push_i32(42);
    vm.run_word(test_lt as usize);
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, "< (lt) 2");

    vm.push_i32(42);
    vm.push_i32(69);
    vm.run_word(test_lt as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, "< (lt) 3");

    vm.push_i32(69);
    vm.push_i32(42);
    vm.run_word(test_ge as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, ">= (ge) 1");

    vm.push_i32(42);
    vm.push_i32(69);
    vm.run_word(test_ge as usize);
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, ">= (ge) 2");

    vm.push_i32(42);
    vm.push_i32(42);
    vm.run_word(test_ge as usize);
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, ">= (ge) 3");

    vm.push_i32(42);
    vm.push_i32(42);
    vm.run_word(test_le as usize);
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, "< (le) 1");

    vm.push_i32(69);
    vm.push_i32(42);
    vm.run_word(test_le as usize);
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, "< (le) 2");

    vm.push_i32(42);
    vm.push_i32(69);
    vm.run_word(test_le as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, "< (le) 3");
}

#[test]
fn test_cfa() {
    let mut vm = create_vm();

    let test_cfa = vm.colon_def("test_cfa", &[">cfa", "bye"]);

    let state_idx = vm.find("state").unwrap();

    vm.push_i32(state_idx);
    vm.run_word(test_cfa as usize);

    let result = vm.pop_i32();

    assert_eq!(vm.cfa(state_idx), result);
}

#[test]
fn test_state_and_brac() {
    let mut vm = create_vm();

    let test_state = vm.colon_def("test_state", &["state", "@", "bye"]);
    let test_rbrac = vm.colon_def("test_rbrac", &["]", "state", "@", "bye"]);
    let test_lbrac = vm.colon_def("test_lbrac", &["[", "state", "@", "bye"]);

    vm.run_word(test_state as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 0);
    assert!(vm.is_interpreting());

    vm.run_word(test_rbrac as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 1);
    assert!(vm.is_compiling());

    vm.run_word(test_lbrac as usize);
    let result = vm.pop_i32();

    assert_eq!(result, 0);
    assert!(vm.is_interpreting());
}

#[test]
fn test_immediate() {
    let mut vm = create_vm();
    let test_immediate = vm.colon_def("test_immediate", &["immediate", "bye"]);

    let len_addr = test_immediate + 4;
    let len_byte = vm.read_u8(len_addr);

    let expected_len = "test_immediate".len() as u8;

    assert_eq!(len_byte, expected_len);
    assert_ne!(len_byte & IMMEDIATE, IMMEDIATE);

    vm.run_word(test_immediate as usize);

    let len_byte = vm.read_u8(len_addr);
    assert_eq!(len_byte & IMMEDIATE, IMMEDIATE);
    let len = len_byte & LEN_MASK;
    let is_immed = len_byte & IMMEDIATE;

    assert_eq!(len, expected_len, "expected len");
    assert_eq!(is_immed, IMMEDIATE, "immediate flag");
}

#[test]
fn test_hidden() {
    let mut vm = create_vm();

    let test_hidden = vm.colon_def("test_hidden", &["hidden", "bye"]);

    vm.push_i32(test_hidden);

    let len_addr = test_hidden + 4;
    let len_byte = vm.read_u8(len_addr);

    let expected_len = "test_hidden".len() as u8;

    assert_eq!(len_byte, expected_len);
    assert_ne!(len_byte & HIDDEN, HIDDEN);
    vm.run_word(test_hidden as usize);

    let len_byte = vm.read_u8(len_addr);
    assert_eq!(len_byte & HIDDEN, HIDDEN);
    let len = len_byte & LEN_MASK;
    let is_hidden_ = len_byte & HIDDEN;

    assert_eq!(len, expected_len, "expected len");
    assert_eq!(is_hidden_, HIDDEN, "hidden flag");
}

#[test]
fn test_hide() {
    let mut vm = create_vm();

    vm.fill_input_buffer("base ");

    let test_hide = vm.colon_def("test_hide", &["hide", "bye"]);
    let base_idx = vm.find("base").unwrap();

    vm.run_word(test_hide as usize);

    let len_adr = base_idx + 4;
    let len_byte = vm.read_u8(len_adr);

    assert_eq!(len_byte & HIDDEN, HIDDEN);
}

#[test]
fn test_consts_and_vars() {
    let mut vm = create_vm();

    let test_true_false = vm.colon_def("test_true_false", &["true", "false", "bye"]);
    let test_docol = vm.colon_def("test_docol", &["DOCOL", "bye"]);
    let test_base = vm.colon_def("test_base", &["base", "@", "bye"]);
    let test_base_hex = vm.colon_def("test_base", &["base", "!", "bye"]);

    vm.run_word(test_true_false as usize);
    let value = vm.pop_i32();
    assert_eq!(value, FALSE, "false");
    let value = vm.pop_i32();
    assert_eq!(value, TRUE, "true");

    vm.run_word(test_docol as usize);
    let value = vm.pop_i32();
    assert_eq!(value, mmap::DOCOL as i32);

    vm.run_word(test_base as usize);
    let value = vm.pop_i32();
    assert_eq!(value, 10);

    vm.push_i32(16);
    vm.run_word(test_base_hex as usize);
    let value = vm.read_i32(mmap::BASE as i32);
    assert_eq!(value, 16);
}

#[test]
fn test_find() {
    let mut vm = create_vm();

    let test_find = vm.colon_def("test_find", &["find", "bye"]);
    let hidden_idx = vm.colon_def("hidden_test", &["hidden", "bye"]);

    let in_stream = vm.read_i32(mmap::IN_STREAM as i32) as usize;

    vm.write_str(in_stream, "drop");

    vm.push_i32(in_stream as i32 + 4);
    vm.push_i32(4);

    vm.run_word(test_find as usize);
    let value = vm.pop_i32();
    let expected = vm.find("drop").unwrap();
    assert_eq!(value, expected);

    // "[" word is IMMEDIATE
    vm.write_str(in_stream, "[");

    vm.push_i32(in_stream as i32 + 4);
    vm.push_i32(1);

    vm.run_word(test_find as usize);
    let value = vm.pop_i32();
    let expected = vm.find("[").unwrap();
    assert_eq!(value, expected);

    // hide "base" and try to find it (should fail)
    let base_idx = vm.find("base").unwrap();
    vm.push_i32(base_idx);
    vm.run_word(hidden_idx as usize);
    vm.write_str(in_stream, "base");
    vm.push_i32(in_stream as i32 + 4);
    vm.push_i32(4);
    vm.run_word(test_find as usize);
    let value = vm.pop_i32();
    assert_eq!(value, 0);
}

#[test]
fn test_tick() {
    let mut vm = create_vm();

    let test_tick = vm.colon_def("test_tick", &["'", "lit", "bye"]);

    vm.run_word(test_tick as usize);
    let lit_idx = vm.find("lit").unwrap();
    let result = vm.pop_i32();
    assert_eq!(result, vm.cfa(lit_idx) as i32);
}

#[test]
fn test_key() {
    let mut vm = create_vm();
    vm.fill_input_buffer("test 1234\n");
    let test_k = vm.colon_def("test_key", &["key", "bye"]);

    let expected = "test 1234\n".as_bytes();
    for c in expected {
        vm.run_word(test_k as usize);
        let k = vm.pop_i32() as u8;
        assert_eq!(k, *c);
    }
}

#[test]
fn test_create() {
    let mut vm = create_vm();

    let test_create = vm.colon_def("test_create", &["create", "bye"]);

    let in_stream = vm.read_i32(mmap::IN_STREAM as i32) as usize;
    vm.write_str(in_stream, "foobar ");

    vm.push_i32(in_stream as i32 + 4);
    vm.push_i32("foobar".len() as i32);

    let here = vm.here();
    let latest = vm.latest();

    vm.run_word(test_create as usize);

    let here_after = vm.here();
    let latest_after = vm.latest();

    assert_eq!(latest_after, here);

    let prev_ptr = vm.read_i32(latest_after);

    assert_eq!(prev_ptr, latest);

    let lfa = latest_after + 4;
    let len_byte = vm.read_u8(lfa);
    assert_eq!(len_byte, "foobar".len() as u8);

    for (i, c) in "foobar".as_bytes().iter().enumerate() {
        assert_eq!(*c, vm.read_u8(lfa + 1 + i as i32));
    }

    assert_eq!(here_after, latest_after + 12, "here after");
}

#[test]
fn test_word() {
    let mut vm = create_vm();
    let test_word = vm.colon_def("test_word", &["word", "bye"]);

    vm.fill_input_buffer("     \\sbvasd\n testing-word ");

    // testing-word
    vm.run_word(test_word as usize);

    let len = vm.pop_i32();
    let adr = vm.pop_i32();

    let in_stream = vm.read_i32(mmap::IN_STREAM as i32);

    let expected = "testing-word";

    assert_eq!(len, expected.len() as i32, "length");
    assert_eq!(adr, in_stream as i32, "address");

    for (i, c) in expected.as_bytes().iter().enumerate() {
        assert_eq!(*c, vm.read_u8(in_stream + i as i32));
    }
}

#[test]
fn test_char() {
    let mut vm = create_vm();
    let test_char = vm.colon_def("test_char", &["char", "char", "char", "bye"]);

    vm.fill_input_buffer("     \\sbvasd\n a bc c ");

    // testing-word
    vm.run_word(test_char as usize);

    let c = vm.pop_i32() as u8 as char;
    let b = vm.pop_i32() as u8 as char;
    let a = vm.pop_i32() as u8 as char;

    assert_eq!(a, 'a');
    assert_eq!(b, 'b');
    assert_eq!(c, 'c');
}

#[test]
fn test_number() {
    let mut vm = create_vm();

    vm.fill_input_buffer("127 -1234 12b4 12ab 1010 - ");

    let test_number = vm.colon_def("test_number", &["word", "number", "bye"]);

    vm.run_word(test_number as usize);

    let f = vm.pop_i32();
    let n = vm.pop_i32();

    assert_eq!(f, 0, "number error flag");
    assert_eq!(n, 127, "number result");

    vm.run_word(test_number as usize);

    let f = vm.pop_i32();
    let n = vm.pop_i32();

    assert_eq!(f, 0, "number -1234 error flag");
    assert_eq!(n, -1234, "number -1234 result");

    vm.run_word(test_number as usize);

    let f = vm.pop_i32();
    let n = vm.pop_i32();

    assert_eq!(f, 2, "number 12b4 error flag");
    assert_eq!(n, 0, "number 12b4 result");

    vm.set_base(16);

    vm.run_word(test_number as usize);

    let f = vm.pop_i32();
    let n = vm.pop_i32();

    assert_eq!(f, 0, "number 12ab error flag");
    assert_eq!(n, 0x12ab, "number 12ab result");

    vm.set_base(2);

    vm.run_word(test_number as usize);

    let f = vm.pop_i32();
    let n = vm.pop_i32();

    assert_eq!(f, 0, "number 1010 error flag");
    assert_eq!(n, 0b1010, "number 1010 result");

    vm.set_base(2);

    vm.run_word(test_number as usize);

    let f = vm.pop_i32();
    let n = vm.pop_i32();

    assert_eq!(f, 0, "number 12ab error flag");
    assert_eq!(n, 0, "number 12ab result");
}

#[test]
fn test_execute() {
    let mut vm = create_vm();

    let test_execute = vm.colon_def("test_execute", &["execute", "bye"]);
    let add_four = vm.find("4+").unwrap();
    let add_four_cfa = vm.cfa(add_four);

    vm.push_i32(4711);
    vm.push_i32(add_four_cfa);
    vm.run_word(test_execute as usize);
    let value = vm.pop_i32();

    assert_eq!(value, 4715);

    let two_dup = vm.find("2dup").unwrap();
    let two_dup_cfa = vm.cfa(two_dup);

    vm.push_i32(1);
    vm.push_i32(2);
    vm.push_i32(two_dup_cfa);
    vm.run_word(test_execute as usize);

    assert_eq!(2, vm.pop_i32());
    assert_eq!(1, vm.pop_i32());
    assert_eq!(2, vm.pop_i32());
    assert_eq!(1, vm.pop_i32());
}

#[test]
fn test_comma() {
    let mut vm = create_vm();

    let test_comma = vm.colon_def("test_comma", &[",", "bye"]);

    let here_before = vm.here();
    vm.push_i32(80);
    vm.run_word(test_comma as usize);

    let here_after = vm.here();

    assert_eq!(here_after, here_before + 4);

    let value = vm.read_i32(here_before);

    assert_eq!(value, 80);
}

#[test]
fn test_colon() {
    let mut vm = create_vm();

    vm.fill_input_buffer("square ");

    let test_colon = vm.colon_def("test_colon", &[":", "bye"]);
    let latest = vm.latest();
    let here = vm.here();

    vm.run_word(test_colon as usize);

    vm.print_memory_dump(vm.latest() as usize);

    let latest_after = vm.latest();
    assert_eq!(here, latest_after, "latest set");
    assert_eq!(latest, vm.read_i32(latest_after), "previous set");
    let len = vm.read_u8(latest_after + 4);
    assert_eq!(len & LEN_MASK, 6, "len set");
    let xt = vm.cfa(latest_after);
    assert_eq!(vm.read_i32(xt), mmap::DOCOL as i32, "DOCOL is set");

    assert_eq!(len & HIDDEN, HIDDEN);
    assert_eq!(vm.state(), 1, "compiling");
}

#[test]
fn test_compilation() {
    let mut vm = create_vm();

    vm.fill_input_buffer(": square dup * ; ");

    let test_compile = vm.colon_def(
        "test_compile",
        &["interpret", "interpret", "interpret", "interpret", "bye"],
    );
    let latest = vm.latest();
    vm.run_word(test_compile as usize);

    let square_idx = vm.find("square");
    assert!(square_idx.is_some());

    let idx = square_idx.unwrap();
    assert_eq!(idx, vm.latest(), "latest has been updated");
    assert_eq!(latest, vm.read_i32(idx), "previous");

    let len = vm.read_u8(idx + 4);
    assert_eq!(len, 6, "len");

    let cfa = vm.cfa(idx);
    assert_eq!(mmap::DOCOL, vm.read_i32(cfa) as usize, "docol is set");

    assert_eq!(
        vm.read_i32(cfa + 4),
        vm.cfa(vm.find("dup").unwrap()),
        "dup cfa set"
    );

    assert_eq!(
        vm.read_i32(cfa + 8),
        vm.cfa(vm.find("*").unwrap()),
        "* cfa set"
    );

    let exit = vm.find("exit").unwrap();
    assert_eq!(vm.read_i32(cfa + 12), vm.cfa(exit), "exit cfa set");

    assert_eq!(vm.here(), cfa + 16, "empty space from here");

    assert_eq!(vm.state(), INTERPRETING, "should no longer be compiling");

    // test compiled word

    let test_square = vm.colon_def("test_square", &["square", "bye"]);

    vm.push_i32(4);
    vm.run_word(test_square as usize);

    assert_eq!(16, vm.pop_i32());
}

#[test]
fn test_interpret() {
    let mut vm = create_vm();
    vm.fill_input_buffer("128 4+ 777 ");

    let test_interpret_lit = vm.colon_def("test_interpret_lit", &["interpret", "bye"]);

    // testing-word
    vm.run_word(test_interpret_lit as usize);
    let value = vm.pop_i32();
    assert_eq!(value, 128, "interpret lit");

    vm.push_i32(38);
    vm.run_word(test_interpret_lit as usize);
    let value = vm.pop_i32();
    assert_eq!(value, 42, "interpret exec");

    let here = vm.here();
    vm.set_state(COMPILING); // compiling

    vm.run_word(test_interpret_lit as usize);
    let lit_xt = vm.read_i32(here);
    assert_eq!(
        lit_xt,
        vm.cfa(vm.find("lit").unwrap()) as i32,
        "interpret compile lit: lit xt "
    );
    let value = vm.read_i32(here + 4);
    assert_eq!(value, 777, "interpret compile lit: value");
    let new_here = vm.here();
    assert_eq!(new_here, here + 8, "interpret compile lit: updated here");
}

#[test]
fn test_quit() {
    let mut vm = create_vm();
    vm.fill_input_buffer(" : cells \n4 * \n; \\ some useless comment\n 16 cells\n bye foo");

    let quit = vm.find("quit").unwrap();

    vm.run_word(quit as usize);

    let result = vm.pop_i32();

    assert_eq!(result, 64);
}
