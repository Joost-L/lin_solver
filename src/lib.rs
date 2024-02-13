//! Parses a linear programming problem from a string using [parser] and finds the optimal solution
//! using [solver].\
//! \
//! Every line will need to contain the same amount of variables and in the same order (the parser is blind for names).
//! The first line is read to be the linear objective function while all other lines are constraints of the form: f >= 0 \
//! The solver will keep rewriting the system until it can read out the solution. You can also directly create a [SystemEq](solver::SystemEq) and solve that.\
//! \
//! It is assumed that every variable cannot be negative.
//! 
//! # Example
//! Remember that the first line is the objective function
//! ```
//! let input ="1 + 2x
//!             4 - 2x
//!             0 + x";
//! let system = lin_solver::parser::parse_to_system(input.to_string());
//! let res = system.unwrap().solve();
//! assert_eq!(5.0, res);
//! ```
//! # Furture
//! More flexible function types in nalgebra\
//! More flexible parsing that can read variable names

pub mod parser;
pub mod solver;
pub mod example;
extern crate nalgebra as na;


pub fn read_args(mut args:impl Iterator<Item = String>) -> Result<String,&'static str> {
    args.next();
    if let Some(x) = args.next() {
        return Ok(x);
    } else {
        return Err("Not enough arguments, no file specified!");
    }
}




// pub fn read_file() {
//     let matrix:Vec<Vec<i32>> =
// }