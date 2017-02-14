extern crate regex;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

fn read_file(filename: String) -> Result<String, io::Error> {
    let mut file = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn print_matched_line(file: String, re: Regex) {
    println!("{}",
             match read_file(file.clone()) {
                 Ok(content) => filter_matched_line(content, re),
                 Err(reason) => panic!(reason),
             });
}

fn filter_matched_line(content: String, re: Regex) -> String {
    let lines: Vec<String> = content.split("\n")
        .filter(|line| line.len() > 0)
        .filter(|line| re.is_match(line))
        .map(|line| format!("{}", line))
        .collect();
    lines.join("\n")
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let pattarns = args[1].clone();
    let file = args[2].clone();

    let re = Regex::new(&pattarns).unwrap();

    print_matched_line(file, re);

}
