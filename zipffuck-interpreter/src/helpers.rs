#![allow(dead_code)]

use std::io::{stdin, stdout, Read, Write};

/// Reads the first commandline argument passed to the program.
/// Panics if the no such argument exists.
pub fn read_first_argument() -> String {
    std::env::args().nth(1).expect("No first argument given.")
}

/// Reads a single byte from `stdin`, which does not necessarily mean a single character.
/// Treats EOF in the stream as a zero.
pub fn read_byte_from_stdin() -> u8 {
    stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .unwrap_or(0)
}

/// Flushes `stdout`, ensuring that no data is left in its buffer.
/// Panics if not all bytes could be written due to I/O errors, EOF being reached or stdout being closed.
pub fn flush_stdout() {
    stdout().flush().expect("Could not flush stdout");
}
