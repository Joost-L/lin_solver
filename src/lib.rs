//! Parses a linear programming problem from a string using [parser] and finds the optimal solution
//! using [solver].\
//! \
//! Every line will need to contain the same amount of variables and in the same order (the parser is blind for names)
//! The solver will keep rewriting the system until it can read out the solution. You can also directly create a [SystemEq](solver::SystemEq) and solve that.\
//! \
//! It is assumed that every variable cannot be negative.
//! 
//! # Example
//! ```
//! let input ="1 + 2x
//!             4 - 2x
//!             0 + x";
//! let system = lin_solver::parser::parse_to_system(input);
//! let res = system.solve();
//! assert_eq!(5.0, res);
//! ```
//! # Furture
//! I want to rework this crate using proper arrays and matrices\
//! More flexible parsing that can read variable names

pub mod parser;
pub mod solver;

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