extern crate getopts;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use getopts::Options;

fn read_file(filename: String) -> Result<String, io::Error> {
    let mut file = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn add_line_number(content: String) -> String {
    let lines: Vec<String> = content.split("\n")
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(index, line)| format!("{}: {}", index + 1, line))
        .collect();
    lines.join("\n")
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_file(file_name: String) {
    println!("{}",
             match read_file(file_name.clone()) {
                 Ok(content) => content,
                 Err(reason) => panic!(reason),
             });
}

fn print_file_with_number(file_name: String) {
    println!("{}",
             match read_file(file_name.clone()) {
                 Ok(content) => add_line_number(content),
                 Err(reason) => panic!(reason),
             });
}

fn print_files(file_names: Vec<String>, with_option_n: bool) {
    if with_option_n {
        for file_name in file_names {
            print_file_with_number(file_name);
        }
    } else {
        for file_name in file_names {
            print_file(file_name);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("n", "", "show line numver");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let with_option_n = matches.opt_present("n");
    let files = if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    } else {
        matches.free
    };

    print_files(files, with_option_n);

}
