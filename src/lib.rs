mod forthvm;
mod init_dictionary;
mod input_stream;

#[cfg(test)]
mod tests;

pub const CELL: i32 = 4;

pub use forthvm::{ForthVM, VmConfig};
pub use input_stream::*;

use crate::forthvm::NEXT;

/// memory map
/// where is stuff stored in memory
pub mod mmap {
    /// top of parameter stack
    pub const DSP: usize = 4;
    /// base (bottom) of parameter stack
    pub const S0: usize = 8;
    /// top of forth return stack
    pub const RSP: usize = 12;
    /// base (bottom) of forth return stack
    pub const R0: usize = 16;
    /// top of vm call stack
    pub const CTOP: usize = 20;
    /// base (bottom) of vm call stack
    pub const CBASE: usize = 24;
    /// top of locals stack
    pub const LTOP: usize = 28;
    /// base of locals stack
    pub const LBASE: usize = 32;
    /// next free byte in  forth dictionary  
    pub const HERE: usize = 36;
    /// last added word
    pub const LATEST: usize = 40;
    /// interpreter counter. points to next word to execute
    pub const IC: usize = 44;
    /// help var
    pub const A0: usize = 48;
    /// are we compiling (1) or executing (0)
    pub const STATE: usize = 52;
    /// word writes here
    pub const IN_STREAM: usize = 56;
    /// number base
    pub const BASE: usize = 60;

    pub const START_ADR: usize = 64;

    pub const COLD_START: usize = 68;

    pub const INPUT_BUFFER: usize = 72;
    pub const INPUT_BUFFER_IDX: usize = 76;
    /// docol code location
    pub const DOCOL: usize = 80;
    /// start of forth dictionary
    pub const DICT: usize = DOCOL + 64;
}
pub const MAX_WORD_LEN: usize = 13;

pub const LEN_MASK: u8 = 0x1f;
pub const HIDDEN: u8 = 0x20;
pub const IMMEDIATE: u8 = 0x80;

pub const TRUE: i32 = 0x1;
pub const FALSE: i32 = 0x0;

// STATE
pub const INTERPRETING: i32 = 0;
pub const COMPILING: i32 = 1;

#[rustfmt::skip]
const fn docol() -> [u8; 47] {
    use toyvm::opcode::*;
    let ic = mmap::IC.to_ne_bytes();
    let rsp = mmap::RSP.to_ne_bytes();
    let a0 = mmap::A0.to_ne_bytes();
    [
        I32_CONST, ic[0], ic[1], ic[2], ic[3], 
        I32_LOAD, 
        I32_CONST, rsp[0], rsp[1], rsp[2], rsp[3], 
        I32_LOAD, 
        I32_STORE, 
        // adjust stack top
        I32_CONST, rsp[0], rsp[1], rsp[2], rsp[3], 
        DUP, 
        I32_LOAD, 
        I32_CONST, 4, 0, 0, 0, 
        SUB,
        SWAP, 
        I32_STORE,
        // adjust ic
        I32_CONST, a0[0], a0[1], a0[2], a0[3],
        I32_LOAD,
        I32_CONST, 4, 0, 0, 0,
        ADD,
        I32_CONST, ic[0], ic[1], ic[2], ic[3],
        I32_STORE,
        NEXT,
    ]
}

pub(crate) fn align(idx: i32) -> i32 {
    (idx + 3) & !3
}
