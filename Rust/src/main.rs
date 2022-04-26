use std::env;
use std::fs::File;
use std::io::Read;
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

        let mut file: File = match File::open(&new_path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // incredibly inefficient to store whole file content in memory, but due to me being a
        // shitty programmer i do not know a better way to do this currently, if you read this,HELP
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {}
        }

        chars = s.len() as u128;
        lines = s.lines().count() as u64;
        // this does not count correctly :/
        words = s.trim().split_whitespace().collect::<String>().len() as u128;
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
                println!("{0} total", total_lines);
            }
            Mode::CHARS => {
                println!("{0} total", total_chars);
            }
            Mode::WORDS => {
                println!("{0} total", total_words);
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
                println!("Usage: \n\txc [FILES] [OPTIONS]\n \n-m  --chars \n\t Print characters in file\n \n-l  --lines \n\t Print lines in file\n \n-w  --words \n\t Print words in file\n");
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

    if files.len() <= 1 {
        return;
    }

    run(parsed_arguments, mode);
}
