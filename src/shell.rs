use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::str::FromStr;
use spin::Mutex;
use lazy_static::lazy_static;
use pc_keyboard::KeyCode;
use crate::{print, println, shell, QemuExitCode};

//This is the static buffer that will be used to handle the keys entered for shell commands
lazy_static! {
    pub static ref SHELL_BUFFER: Mutex<Vec<char>> = Mutex::new(Vec::new());
    pub static ref COMMAND_CACHE_UP: Mutex<CommandCache> = Mutex::new(CommandCache::new());
    pub static ref COMMAND_CACHE_DOWN: Mutex<CommandCache> = Mutex::new(CommandCache::new());
}

pub struct CommandCache {
    command_list: Vec<Vec<char>>,
}
impl CommandCache {
    fn new() -> CommandCache {
        CommandCache {
            command_list: Vec::new(),
        }
    }
    fn push_line(&mut self, line: &Vec<char>) {
        if self.command_list.len() >= 20 { //Max Length is 20
            self.command_list.remove(0);
        }
        self.command_list.push(line.clone());
    }

    fn pop_line(&mut self) -> Option<Vec<char>> {
        self.command_list.pop()
    }
}

///Handle the up key
pub fn handle_up() {
    let mut shell_buffer = SHELL_BUFFER.lock();
    let mut cc_down = COMMAND_CACHE_DOWN.lock();
    let mut cc_up = COMMAND_CACHE_UP.lock();

    if let Some(command) = cc_up.pop_line() {
        cc_down.push_line(&shell_buffer);
        let length_of_current_buffer = shell_buffer.len();
        shell_buffer.clear();
        shell_buffer.extend(command.iter().cloned());
        for _i in 0..length_of_current_buffer {
            write_backspace();
        }
        print!("{}", shell_buffer.iter().collect::<String>());
    }
}

pub fn handle_down() {
    let mut shell_buffer = SHELL_BUFFER.lock();
    let mut cc_down = COMMAND_CACHE_DOWN.lock();
    let mut cc_up = COMMAND_CACHE_UP.lock();

    if let Some(command) = cc_down.pop_line() {
        cc_up.push_line(&shell_buffer);
        let length_of_current_buffer = shell_buffer.len();
        shell_buffer.clear();
        shell_buffer.extend(command.iter().cloned());
        for _i in 0..length_of_current_buffer {
            write_backspace();
        }
        print!("{}", shell_buffer.iter().collect::<String>());
    }
}

fn write_backspace() {
    print!("{}", '\x08');
    print!(" ");
    print!("{}", '\x08');
}

///Handles the character
pub fn handle_char(c: char) {
    append_char_to_buffer(c);
    print!("{}", c);
}

/// Appends the chosen char to the shell buffer
fn append_char_to_buffer(c: char) {
    let mut shell_buffer = SHELL_BUFFER.lock();
    shell_buffer.push(c);
}

///Handles the backspace input
pub fn handle_backspace() {
    let mut shell_buffer = SHELL_BUFFER.lock();
    if shell_buffer.pop().is_some() {
        write_backspace();
    }
}
///Interpret the command
pub fn interpret_line() {
    let mut shell_buffer = SHELL_BUFFER.lock();
    let mut cc_up = COMMAND_CACHE_UP.lock();
    let mut cc_down = COMMAND_CACHE_DOWN.lock();

    while let Some(command_flush) = cc_down.pop_line() {
        if !command_flush.is_empty() {
            cc_up.push_line(&command_flush)
        }
    }

    cc_up.push_line(&shell_buffer);
    let command = shell_buffer.iter().collect::<String>();

    println!(); // print a new line to make it look better

    match command.as_str() {
        "hello" => {
            do_hello();
        }
        "clear" => {
            do_clear();
        }
        "help" => {
            do_help();
        }
        "panic" => {
            do_panic();
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
        "spam" => {
            do_spam(&command[1..]);
        }
        _ => println!("Command not found: {}", command_line),
    }
}

///Spams the output the input amount of times
fn do_spam(args: &[&str]) {
    let expression = args.join(" ");
    if let Some((x_times, print_string)) = split_spam_args(expression.as_str()) {
        for _i in 0..x_times {
            print!("{}", print_string);
        }
        println!();
    }
}
///Helper function for do_spam()
fn split_spam_args(string: &str) -> Option<(i32, String)> {
    let space_index = string.find(|c: char| c == ' ')?;
    let (number_of_times, string_to_print) = string.split_at(space_index);
    Some((number_of_times.trim().parse::<i32>().ok()?, String::from(string_to_print.trim_start())))
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

///This function tells people what they can do! TODO: Add entries into here as more are made
fn do_help() {
    do_clear();
    println!("********************************************************************************");
    println!("These are the runnable commands and how to use them:");
    println!("clear - Clears the screen");
    println!("hello - Says hi");
    println!("help - Honestly, I have no idea on this one");
    println!("echo (string) - Prints a string to the console");
    println!("        string - a string to print");
    println!("quick_math (expression) - Calculates and gives the result of a math expression");
    println!("        expression - math expression in form n$m");
    println!("                 (n and m = operands, $ = operator [+-*/], insensitive to space)");
    println!("        Example Input: 1+3, 1+ 3, 1 +3, or 1 + 3");
    println!("panic - Triggers a todo panic (for debugging purposes)");
    println!("spam (n) (string) - Spams a string n times");
    println!("        n - the number of times to spam the string");
    println!("        string - a string to spam");
    println!("********************************************************************************");
    println!("\n\n\n\n\n\n\n");
}

///Exit the kernel
fn do_panic() {
    todo!();
}

///TODO: Implement this function without balking
pub fn handle_other_raw_key(p0: KeyCode) {

}