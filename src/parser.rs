use crate::solver::SystemEq;

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



pub fn parse_line(line:&str) -> Result<Vec<f32>, &'static str> {
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
    Ok(result)
}

pub fn parse_arg(arg:String) -> Result<SystemEq, &'static str> {

    let mut lines = arg.lines();
    let objective = lines.next()
        .expect("Need at least one equation for the objective function");
    let objective = parse_line(objective)?;

    let mut constraints:Vec<Vec<f32>> = vec![];
    for line in lines {
        constraints.push(parse_line(line)?)
    }

    Ok(SystemEq { 
        var_len:objective.len(),
        objective, 
        constraints,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant() {
        let input = "1";
        let vec = parse_line(input);

        assert_eq!(vec, Ok(vec![1.0]))
    }

    #[test]
    fn variable() {
        let input = "4 -x1 + 3x2";
        let vec = parse_line(input);

        assert_eq!(vec, Ok(vec![4.0 , -1.0, 3.0]));
    }

    #[test]
    fn zero_value() {
        let input = "-0 +2x1 -0x2";
        let vec = parse_line(input);

        assert_eq!(vec, Ok(vec![0.0, 2.0, 0.0]))
    }
}