#![no_std]
extern crate alloc;

pub mod stdin;
pub mod stdout;

use stdin::Stdin;
use stdout::Stdout;

pub trait Console {
    fn stdin() -> Stdin;
    fn stdout() -> Stdout;
}
