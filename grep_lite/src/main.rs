use regex::Regex;
use clap::{Arg,Command};

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
    .get_matches();

    
    let pattern = args.get_one::<String>("pattern")
        .expect("required by clap");
    
    let re = Regex::new(pattern).unwrap();

    let quote = "\
    Every face, every shop, bedroom window, public house, and dark square,
    is a picture feverishly turned--in search of what?
    It is the same with books.

    What do we seek through millions of pages?";
    
    for (idx,line) in quote.lines().enumerate(){
        let contains_substring = re.find(line); 
        match contains_substring {
            Some(_) => println!("{}:{}",idx,line),
            None => (),
        }
    };
}
