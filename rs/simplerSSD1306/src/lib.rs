#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(non_snake_case)]
//#![feature(stmt_expr_attributes)]
extern crate alloc;

pub mod access;

pub mod ssd1306_cmd;
mod util;

pub mod ssd1306;

pub mod ssd1306_init_seq1;
pub use access::SSD1306Access;
pub use ssd1306::SSD1306;
pub use ssd1306_cmd::*;
