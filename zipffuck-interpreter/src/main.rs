use crate::helpers::read_byte_from_stdin;
use crate::helpers::read_first_argument;
use std::fs::read_to_string;
use std::io;
use std::io::ErrorKind;

mod helpers;

fn main() -> io::Result<()> {
    let input_file = read_first_argument();
    let input = read_to_string(input_file)?;

    match interpret_zipffuck(input) {
        true => Ok(()),
        false => Err(io::Error::new(
            ErrorKind::Other,
            "Zipffuck interpreter returned false.",
        )),
    }
}

const CELLS: usize = 30_000;

const MOVE_RIGHT: char = '>';
const MOVE_LEFT: char = '<';
const INCREMENT: char = '+';
const DECREMENT: char = '-';
const PRINT: char = '.';
const READ: char = ',';
const JUMP_FORWARD: char = '[';
const JUMP_BACKWARD: char = ']';

fn interpret_zipffuck(code: String) -> bool {
    let brainfuck = convert_zipffuck(&code);
    is_valid(&brainfuck) && interpret_as(&brainfuck)
}

fn interpret_as(brainfuck: &[char]) -> bool {
    let mut memory = [0_u8; CELLS];
    let mut pointer: usize = 0;

    let mut index = 0;
    let length = brainfuck.len();

    while index < length {
        match brainfuck[index] {
            MOVE_RIGHT => {
                move_to_right(&mut pointer);
                index += 1;
            }
            MOVE_LEFT => {
                move_to_left(&mut pointer);
                index += 1;
            }
            INCREMENT => {
                increment(&mut memory[pointer]);
                index += 1;
            }
            DECREMENT => {
                decrement(&mut memory[pointer]);
                index += 1;
            }
            PRINT => {
                print_char(&memory[pointer]);
                index += 1;
            }
            READ => {
                read_char(&mut memory[pointer]);
                index += 1;
            }
            JUMP_FORWARD if memory[pointer] == 0 => {
                jump_forward(&mut index, brainfuck);
            }
            JUMP_BACKWARD if memory[pointer] != 0 => {
                jump_backward(&mut index, brainfuck);
            }
            _ => index += 1,
        }
    }
    true
}

fn is_valid(brainfuck: &[char]) -> bool {
    let mut jumpes = 0;

    for operation in brainfuck {
        match *operation {
            JUMP_BACKWARD => jumpes -= 1,
            JUMP_FORWARD => jumpes += 1,
            _ => continue,
        }
    }
    jumpes == 0
}

fn convert_zipffuck(code: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();

    let words = code.split_whitespace();
    for word in words {
        match get_brainfuck_operation(word) {
            Some(char) => result.push(char),
            None => continue,
        }
    }
    result
}

fn get_brainfuck_operation(word: &str) -> Option<char> {
    match word.replace('.', "").to_lowercase().as_str() {
        "time" | "be" | "last" | "of" | "and" | "not" | "the" | "it" | "yeah" | "will" => {
            Some(MOVE_RIGHT)
        }
        "year" | "have" | "other" | "in" | "that" | "out" | "a" | "i" | "no" | "would" => {
            Some(MOVE_LEFT)
        }
        "people" | "do" | "new" | "to" | "but" | "up" | "this" | "you" | "yes" | "can" => {
            Some(INCREMENT)
        }
        "way" | "say" | "good" | "for" | "or" | "so" | "his" | "he" | "well" | "could" => {
            Some(DECREMENT)
        }
        "man" | "go" | "old" | "on" | "as" | "then" | "which" | "they" | "aye" | "should" => {
            Some(PRINT)
        }
        "day" | "get" | "great" | "with" | "if" | "more" | "an" | "she" | "hello" | "may" => {
            Some(READ)
        }
        "thing" | "make" | "high" | "at" | "than" | "now" | "their" | "we" | "ha" | "must" => {
            Some(JUMP_FORWARD)
        }
        "child" | "see" | "small" | "by" | "when" | "just" | "what" | "there" | "dear"
        | "might" => Some(JUMP_BACKWARD),
        _ => None,
    }
}

fn move_to_right(pointer: &mut usize) {
    if *pointer == CELLS - 1 {
        *pointer = 0;
    } else {
        *pointer += 1;
    }
}

fn move_to_left(pointer: &mut usize) {
    if *pointer == 0 {
        *pointer = CELLS - 1;
    } else {
        *pointer -= 1;
    }
}

fn increment(cell: &mut u8) {
    *cell = cell.wrapping_add(1);
}

fn decrement(cell: &mut u8) {
    *cell = cell.wrapping_sub(1);
}

fn print_char(cell: &u8) {
    print!("{}", *cell as char);
}

fn read_char(cell: &mut u8) {
    *cell = read_byte_from_stdin();
}

fn jump_forward(index: &mut usize, brainfuck: &[char]) {
    let mut counter = 1;

    while counter > 0 {
        *index += 1;
        match brainfuck[*index] {
            JUMP_BACKWARD => counter -= 1,
            JUMP_FORWARD => counter += 1,
            _ => continue,
        }
    }
    *index += 1
}

fn jump_backward(index: &mut usize, brainfuck: &[char]) {
    let mut counter = 1;

    while counter > 0 {
        *index -= 1;
        if brainfuck[*index] == JUMP_FORWARD {
            counter -= 1;
        } else if brainfuck[*index] == JUMP_BACKWARD {
            counter += 1;
        }
    }
}
