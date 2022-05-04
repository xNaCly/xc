use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

enum Mode {
    CHARS,
    WORDS,
    LINES,
    ALL,
}

struct Cli {
    flags: Vec<String>,
    args: Vec<String>,
}

struct Xfile {
    name: String,
    chars: u128,
    words: u128,
    lines: u64,
}

fn parse_args(mut arguments: Vec<String>) -> Cli {
    arguments.remove(0);
    let mut flags: Vec<String> = Vec::new();
    let mut args: Vec<String> = Vec::new();

    for arg in arguments {
        if arg.starts_with("--") || arg.starts_with("-") {
            flags.push(arg);
        } else {
            args.push(arg);
        }
    }

    return Cli { flags, args };
}

fn read_file(path: String) -> Xfile {
    let new_path = Path::new(&path);
    let temp: Vec<&str> = path.split(std::path::MAIN_SEPARATOR).collect();
    let mut chars: u128 = 0;
    let mut words: u128 = 0;
    let mut lines: u64 = 0;

    if new_path.exists() && new_path.is_file() {
        let display = new_path.display();

        let file = match File::open(&new_path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };


        let reader = BufReader::new(file);
        for line in reader.lines() {
            lines += 1;
            chars += 1; // Count the newline character

            let line = line.unwrap();
            chars += line.len() as u128;
            words += line.split_whitespace().count() as u128;
        }
    }

    return Xfile {
        name: String::from(temp[temp.len() - 1]),
        chars,
        words,
        lines,
    };
}

fn run(arguments: Cli, mode: Mode) {
    let mut total_chars: u128 = 0;
    let mut total_words: u128 = 0;
    let mut total_lines: u64 = 0;
    let len: usize = arguments.args.len();

    for file in arguments.args {
        let f: Xfile = read_file(file);
        total_chars += f.chars;
        total_words += f.words;
        total_lines += f.lines;
        match mode {
            Mode::LINES => {
                println!("{0:>3} {1:>2}", f.lines, f.name);
            }
            Mode::CHARS => {
                println!("{0:>3} {1:>2}", f.chars, f.name);
            }
            Mode::WORDS => {
                println!("{0:>3} {1:>2}", f.words, f.name);
            }
            _ => {
                println!(
                    "{0:>3} {1:>3} {2:>3} {3:>2}",
                    f.lines, f.words, f.chars, f.name
                );
            }
        }
    }
    if len > 1 {
        match mode {
            Mode::LINES => {
                println!("{0:>3} total", total_lines);
            }
            Mode::CHARS => {
                println!("{0:>3} total", total_chars);
            }
            Mode::WORDS => {
                println!("{0:>3} total", total_words);
            }
            _ => {
                println!(
                    "{0:>3} {1:>2} {2:>2} total",
                    total_lines, total_words, total_chars
                );
            }
        }
    }
}

fn main() {
    let mut mode: Mode = Mode::ALL;
    let args: Vec<String> = env::args().collect();
    let parsed_arguments: Cli = parse_args(args);
    let files = &parsed_arguments.args;
    let flags = &parsed_arguments.flags;

    for i in flags {
        let i_ = String::from(i);
        match i_.as_str() {
            "-v" | "--version" => {
                println!("xc - version: 1");
                return;
            }
            "-h" | "--help" => {
                println!("Usage: \n\
                          \txc [FILES] [OPTIONS]\n\
                          \n\
                          -m  --chars \n\
                          \tPrint characters in file\n\
                          \n\
                          -l  --lines \n\
                          \tPrint lines in file\n\
                          \n\
                          -w  --words \n\
                          \tPrint words in file");
                return;
            }
            "-l" | "--lines" => {
                mode = Mode::LINES;
            }
            "-m" | "--chars" => {
                mode = Mode::CHARS;
            }
            "-w" | "--words" => {
                mode = Mode::WORDS;
            }
            _ => mode = Mode::ALL,
        }
    }

    if files.len() < 1 {
        println!("xc: Not enough arguments.");
        return;
    }

    run(parsed_arguments, mode);
}
