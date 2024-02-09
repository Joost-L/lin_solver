use std::ops::Add;

#[derive(Debug)]
pub struct SystemEq {
    pub var_len:usize,
    pub objective:Vec<f32>,
    pub constraints:Vec<Vec<f32>>
}


/// Zips two vectors by adding their elements
/// 
/// # Panics
/// If the two vectors do not have the same length
/// 
/// # Examples
/// 
/// ```
/// let a = vec![0,1,2,3];
/// let b = vec![0,2,4,6];
/// let res = lin_solver::solver::add_vec(&a, &b);
/// 
/// assert_eq!(res,vec![0,3,6,9]);
/// ```
pub fn add_vec<T>(a:&Vec<T>, b:&Vec<T>) -> Vec<T>
    where T:Add<Output = T> + Copy
{
    if a.len() != b.len() {
        panic!("Length of vector a does not match length of vector b!\na.len():{}\nb.len():{}",a.len(),b.len())
    }
    a.iter().zip(b.iter()).map(|(x,y)| *x + *y).collect()
}

fn substitute(x_definition: &Vec<f32>, x_index:usize, formula:&mut Vec<f32>) {
    let k = formula[x_index];
    formula[x_index] = 0.0;

    let scaled_x = x_definition.iter().map(|a| a * k).collect();
    *formula = add_vec(formula,&scaled_x);
} 

/// Rewrite a linear formula by swapping a basic and non-basic variable
/// 
/// # Examples
/// Rewrite w = 4 + 2 x\
/// to x = -2 + 0.5 w 
/// ```
/// let mut formula = vec![4.0, 2.0];
/// lin_solver::solver::rewrite(&mut formula, 1);
/// 
/// assert_eq!(formula, vec![-2.0, 0.5]) 
/// ```
pub fn rewrite(formula:&mut Vec<f32>, x_index:usize) {
    let x_factor = formula[x_index];

    //swap  w = ... + q x + ...
    //to    q x = ... - w + ...
    let f_factor = -1.0/x_factor;
    formula[x_index] = -1.0; 

    *formula = formula.iter().map(|a| a * f_factor).collect();
}

impl SystemEq {
    fn first_positive(&self) -> Option<usize> {
        let mut objective_iter = self.objective.iter();
        objective_iter.next(); //ignore constant

        for (i,x) in objective_iter.enumerate() {
            if *x > 0.0 {
                return Some(i + 1);
            } else {
                continue;
            }
        };
        None
    }


    
    /// Given the index of a variable, will rewrite that variable as a basic variable
    /// 
    /// # Examples
    /// ```
    /// let mut system = lin_solver::solver::SystemEq {
    ///     var_len:2,
    ///     objective:vec![1.0,2.0],
    ///     constraints:vec![
    ///         vec![4.0,-2.0],
    ///         vec![1.0, 1.0]
    ///     ]
    /// };
    /// system.rewrite_system(1);
    /// 
    /// assert_eq!(system.constraints[0],vec![2.0, -0.5]);
    /// assert_eq!(system.constraints[1],vec![3.0, -0.5]);
    /// assert_eq!(system.objective, vec![5.0, -1.0]);
    /// ```
    pub fn rewrite_system(&mut self, x_index:usize) {
        //find most restrictive constraint
        let mut most_res:Option<(usize,f32)> = None;

        for (i, constraint) in self.constraints.iter().enumerate() {
            if constraint[x_index] >= 0.0 {continue;}

            let local_max = constraint[0]/constraint[x_index];
            if let Some((_,current_max)) = most_res {
                if local_max <= current_max {
                    break;
                }
            }
            most_res = Some((i,local_max));
        }

        if let Some((formula_index,_)) = most_res {
            let formula = &mut self.constraints[formula_index];
            rewrite(formula, x_index);
            
            substitute(formula, x_index, &mut self.objective);
            let formula = formula.to_owned();
            for (i,constraint) in self.constraints.iter_mut().enumerate() {
                if i != formula_index {
                    substitute(&formula, x_index, constraint)
                }
            }
        }
    }
}