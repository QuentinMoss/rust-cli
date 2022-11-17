use text_colorizer::*;
use std::env;

#[allow(dead_code)]
#[derive(Debug)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String, 
}
fn main() {
    // Skip(1) ignores program name as arg
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!("{} wrong number of arguments given. Expect 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1);
    }
    
}

fn print_help() {
    eprintln!("{} - replace a string with a new string\r\n", "Find and Replace".green());
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>\r\n");
}
