use std::env;
use lin_solver;
use lin_solver::parser;
use std::process;
use std::fs;
//get file name from args
//read out file
//Parse as linear program

fn main() {
    let args = env::args();
    let file_path = lin_solver::read_args(args).unwrap_or_else(|err|{
        eprint!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    let file_contents = fs::read_to_string(file_path).expect("Couldn't read file");
    let matrix = parser::parse_arg(file_contents);

    println!("Resulting Equation: {:?}",matrix)
}
