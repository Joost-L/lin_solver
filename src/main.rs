use std::env;
use lin_solver;
use lin_solver::parser;
use std::process;
use std::fs;

fn main() -> Result<(),&'static str> {
    let args = env::args();
    let file_path = lin_solver::read_args(args).unwrap_or_else(|err|{
        eprint!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    let file_contents = fs::read_to_string(file_path).expect("Couldn't read file");
    let mut matrix = parser::parse_to_system(file_contents)?;

    println!("System: {}",matrix);

    let res = matrix.solve();
    println!("Solved System: value:{}\n{}", res, matrix);
    Ok(())
}
