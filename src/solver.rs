use std::ops::Add;

#[derive(Debug)]
pub struct SystemEq {
    pub var_len:usize,
    pub objective:Vec<f32>,
    pub constraints:Vec<Vec<f32>>
}

pub fn add_vec<T>(a:&Vec<T>, b:&Vec<T>) -> Vec<T>
    where T:Add<Output = T> + Copy
{
    if a.len() != b.len() {
        panic!("Length of vector a does not match length of vector b!\na.len():{}\nb.len():{}",a.len(),b.len())
    }
    a.iter().zip(b.iter()).map(|(x,y)| *x + *y).collect()
}
    

impl SystemEq {
    fn substitute(&mut self, x_definition: &Vec<f32>, x_index:usize, formula_index:usize) {
        let formula :&mut Vec<f32> = &mut self.constraints[formula_index];
        let k = formula[x_index];
        formula[x_index] = 0.0;

        let scaled_x = x_definition.iter().map(|a| a * k).collect();
        *formula = add_vec(formula,&scaled_x);
    } 
}