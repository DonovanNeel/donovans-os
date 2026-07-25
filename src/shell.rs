use alloc::string::{String, ToString};
use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::{print, println};

//This is the static buffer that will be used to handle the keys entered for shell commands
lazy_static! {
    pub static ref SHELL_BUFFER: Mutex<Vec<char>> = Mutex::new(Vec::new());
}

/// Appends the chosen char to the shell buffer
pub fn append_char_to_buffer(c: char) {
    let mut shell_buffer = SHELL_BUFFER.lock();
    shell_buffer.push(c);
}

///Handles the backspace input
pub fn handle_backspace() {
    let mut shell_buffer = SHELL_BUFFER.lock();
    if shell_buffer.pop().is_some() {
        print!("{}", '\x08');
        print!(" ");
        print!("{}", '\x08');
    }
}
///Interpret the command
pub fn interpret_line() {
    let mut shell_buffer = SHELL_BUFFER.lock();
    let command = shell_buffer.iter().collect::<String>();

    println!(); // print a new line to make it look better

    match command.as_str() {
        "hello" => {
            do_hello();
        }
        "clear" => {
            do_clear();
        }
        _ => {
            handle_command_line(command);
        }
    }
    shell_buffer.clear();
}

fn handle_command_line(command_line: String) {
    let command = command_line.split(" ").collect::<Vec<&str>>();

    match command[0] {
        "echo" => {
            do_echo(&command[1..]);
        }
        "quick_math" => {
            do_quick_math(&command[1..]);
        }
        _ => println!("Command not found: {}", command_line),
    }
}

///Does quick math (add, sub, mul, div)
fn do_quick_math(args: &[&str]) {
    let expression = args.join("");
    if let Some((first_operand, operator, second_operand))
        = split_math_string(expression.as_str()) {
        match operator {
            '+' => {
                let sum = first_operand + second_operand;
                println!("{} + {} = {}", first_operand, second_operand, sum);
            }
            '-' => {
                let difference = first_operand - second_operand;
                println!("{} - {} = {}", first_operand, second_operand, difference);
            }
            '*' => {
                let product = first_operand * second_operand;
                println!("{} * {} = {}", first_operand, second_operand, product);
            }
            '/' => {
                let quotient = first_operand / second_operand;
                println!("{} / {} = {} (integer division)", first_operand, second_operand, quotient);
            }
            _ => {
                println!("Lol idk, maybe like 67 (Invalid operator)");
            }
        }
    }
}

///Helper for do quick math
fn split_math_string(string: &str) -> Option<(i32, char, i32)> {
    let operator_index = string.find(|c: char| !c.is_ascii_digit())?;

    let (first_operand_string, rest) = string.split_at(operator_index);

    let mut remaining = rest.chars();
    let operator = remaining.next()?;

    let first_operand = first_operand_string.parse().ok()?;
    let second_operand = remaining.as_str().parse::<i32>().ok()?;

    Some((first_operand, operator, second_operand))
}

///This function will echo back a phrase
fn do_echo(args: &[&str]) {
    let output = args.join(" ").to_string();

    println!("{}", output);
}

///This function says hello!
fn do_hello() {
    println!("Hello World! A coyote built me!!");
}

///A sussy function for getting rid of sussy things
fn do_clear() {
    for _ in 0..25 { println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"); }
}

