use nalgebra::{DMatrix, RowDVector};

use crate::solver::LinearSystem;

enum ReadMode {
    Number,
    Name
}

struct ParseState {
    number_buffer:Option<i32>,
    parity:i32,
    mode:ReadMode
}

impl ParseState {
    fn submit(&mut self) -> Option<f32> {
        if let ReadMode::Number = self.mode {
            Some((self.number_buffer? * self.parity) as f32)
        } else if let Some(n) = self.number_buffer{
            Some((n * self.parity) as f32)
        } else {
            Some(self.parity as f32)
        }
        
        
    }

    fn append_number(&mut self, n:i32) {
        if let Some (x) = self.number_buffer {
            self.number_buffer = Some (x*10 + n)
        } else {
            self.number_buffer = Some (n)
        }
    }
}


/// Parses a string of an equation into a vector. 
/// the i-th float in the vector is the factor of the i-th term
/// 
/// # Examples
/// 
/// ```
/// use nalgebra::RowDVector;
/// let line = "2 + 1x1 + 0x2 - 4x3";
/// let res = lin_solver::parser::parse_line(line).unwrap();
/// 
/// assert_eq!(res, RowDVector::from_vec(vec![2.0, 1.0,  0.0, -4.0]));
/// ```
/// 
/// Variable names are ignored:
/// ```
/// use nalgebra::RowDVector;
/// let line = "0 + 1 + 2a - 3a";
/// let res = lin_solver::parser::parse_line(line).unwrap();
/// 
/// assert_eq!(res, RowDVector::from_vec(vec![0.0, 1.0, 2.0, -3.0]));
/// ```
/// 
pub fn parse_line(line:&str) -> Result<RowDVector<f32>, &'static str> {
    let mut result:Vec<f32> = vec![];
    let mut state = ParseState {
        number_buffer:None,
        parity:1,
        mode:ReadMode::Number
    };
    for elem in line.chars() {
        match elem {
            ' ' => continue,
            'a'..='z' => {
                match state.mode {
                    ReadMode::Name => break,
                    ReadMode::Number => state.mode = ReadMode::Name
                }
            }
            '+'|'-' => {
                if let Some(n) = state.submit() { result.push(n) }
                state.number_buffer = None;
                state.parity = if elem == '+' {1} else {-1};
                state.mode = ReadMode::Number;
            }
            _ => 
                if let ReadMode::Number = state.mode {
                    if let Some(i) = ('0'..='9').position(|y| y == elem) { 
                        state.append_number(i as i32);
                        continue;
                    }
                    else {
                        return Err("Unexpected token");
                    }
                }
        }
    }
    if let Some(n) = state.submit() { result.push(n) }
    Ok(RowDVector::from_vec( result ))
}

/// Parses a string containing multiple lines into a system of linear equations
/// 
/// # Important details
/// Each line should contain the same number of terms in the same order.\
/// repeatedly calls [parse_line]
/// 
/// # Examples
/// The first line corresponds to the objective function
/// ```
/// use nalgebra::{RowDVector, Matrix2};
/// let input = String::from(   "0 + x
///                             3 - x
///                             -2 + 4x");
/// let res = lin_solver::parser::parse_to_system(input).unwrap();
/// assert_eq!(res.objective,   RowDVector::from_vec(vec![0.0, 1.0]));
/// assert_eq!(res.constraints, Matrix2::new( 3.0, -1.0,
///                                                 -2.0, 4.0));
/// ```
pub fn parse_to_system(arg:String) -> Result<LinearSystem, &'static str> {

    let mut vectors = arg.lines().map(|s| parse_line(s));
    let objective = vectors.next()
                .expect("Need at least one equation for the objective function")?;
    let vectors :Result<Vec<RowDVector<f32>>,_> = vectors.collect();
    let constraints:DMatrix<f32> = DMatrix::from_rows(&vectors?[..]);
    Ok(LinearSystem { objective, constraints })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant() {
        let input = "1";
        let vec = parse_line(input);

        assert_eq!(vec, 
            Ok(RowDVector::from_vec(vec![1.0]))
        );
    }

    #[test]
    fn variable() {
        let input = "4 -x1 + 3x2";
        let vec = parse_line(input);

        assert_eq!(vec, 
            Ok(RowDVector::from_vec(vec![4.0 , -1.0, 3.0]))
        );
    }

    #[test]
    fn zero_value() {
        let input = "-0 +2x1 -0x2";
        let vec = parse_line(input);

        assert_eq!(vec, 
            Ok(RowDVector::from_vec(vec![0.0, 2.0, 0.0]))
        );
    }
}