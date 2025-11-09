use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::{Arg, Command};
use regex::Regex;

fn main() {
    let args = Command::new("grep-lite")
        .version("0.1.1")
        .about("search pattern in text")
        .arg(
            Arg::new("pattern")
                .help("the pattern to search for")
                .required(true)
                .value_name("REGEX"),
        )
        .arg(
            Arg::new("input")
                .help("from file to search for")
                .value_name("file")
                .required(true),
        )
        .get_matches();

    let pattern = args.get_one::<String>("pattern").expect("required by clap");
    let re = Regex::new(pattern).unwrap();

    let input = args.get_one::<String>("input").expect("required by clap");

    let file = match File::open(input) {
        Ok(f) => f,
        Err(_e) => {
            eprintln!("provide full file path:");
            return;
        }
    };

    let reader = BufReader::new(file);

    process_file(reader, re);
}

fn process_file<T: BufRead + Sized>(reader: T, re: Regex) {
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let contains_substring = re.find(&line);
        match contains_substring {
            Some(_) => println!("{} : {}", idx, line),
            None => (),
        }
    }
}
