#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![feature(stmt_expr_attributes)]
extern crate alloc;



pub mod access;

mod util;
pub mod ssd1306_cmd;

pub mod ssd1306;

pub mod ssd1306_init_seq1;
