use std::env;
use std::path::Path;

enum Mode {
  CHARS,
  WORDS,
  LINES,
  ALL
}


struct Cli {
  flags: Vec<String>,
  args: Vec<String>
}

struct File {
  name: String,
  chars: u128,
  words: u128,
  lines: u64
}


fn get_flags(mut arguments: Vec<String>) -> Cli{
  arguments.remove(0);
  let mut flags: Vec<String> = Vec::new();
  let mut args: Vec<String> = Vec::new();

  for arg in arguments {
    if arg.starts_with("--") {
      flags.push(arg);
    } else {
      args.push(arg);
    }
  }

  return Cli {
    flags: flags,
    args: args
  }
}

fn read_file(path: String) -> File {
  let new_path = Path::new(&path);
  let temp: Vec<&str> = path.split(std::path::MAIN_SEPARATOR).collect();
  let chars: u128 = 0;
  let words: u128 = 0;
  let lines: u64 = 0;

  if new_path.exists() {

  }

  return File {
    name: String::from(temp[temp.len() - 1]),
    chars,
    words,
    lines,
   }
}

fn run(arguments: Cli, mode: Mode) {
  let mut total_chars: u128 = 0;
  let mut total_words: u128 = 0;
  let mut total_lines: u64 = 0;
  let len: usize = arguments.args.len();

  for file in arguments.args {
    let f: File = read_file(file);
    total_chars += f.chars;
    total_words += f.words;
    total_lines += f.lines;
    println!("{0} {1} {2} {3}", f.lines, f.words, f.chars, f.name);
  }
  if len > 1 {
    println!("{0} {1} {2} total", total_lines, total_words, total_chars);
  }
}

fn main() {
    let mut mode: Mode = Mode::ALL;
    let args: Vec<String> = env::args().collect();
    let parsed_arguments: Cli = get_flags(args);
    let flags = &parsed_arguments.flags;

    for i in flags {
      let i_ = String::from(i);
      match i_.as_str() {
        "-v" | "--version" => {
          println!("xc - version: 1");
          return;
        },
        "-l" | "--lines" => {
          mode = Mode::LINES;
        },
        "-m" | "--chars" => {
          mode = Mode::CHARS;
        },
        "-w" | "--words" => {
          mode = Mode::WORDS;
        }
        _ => mode = Mode::ALL,
      }

    }

    run(parsed_arguments, mode);
}