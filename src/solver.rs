use nalgebra::{RowDVector, DMatrix, Matrix, U1, Dim, RawStorageMut, RowVector};

#[derive(Debug)]
pub struct LinearSystem {
    pub objective : RowDVector<f32>,
    pub constraints: DMatrix<f32>
}

impl std::fmt::Display for LinearSystem {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"objective:{}constraints:{}",self.objective,self.constraints)
    }
}

fn substitute<D:Dim,S>(x_definition: &RowDVector<f32>, x_index:usize, formula:&mut Matrix<f32,U1,D,S>)
    where
        S:RawStorageMut<f32,U1,D>
{
    let k = formula[x_index];
    formula[x_index] = 0.0;

    let scaled_x = x_definition.scale(k);
    let _res :Vec<_> = formula.iter_mut().zip(scaled_x.iter()).map(|(f,x)| *f = *f + x).collect();
} 


/// # Examples
/// ```
/// use nalgebra::RowDVector;
/// let mut vec = RowDVector::from_vec(vec![-1.0, 2.0, 4.0]);
/// 
/// lin_solver::solver::scale(&mut vec,2.0);
/// assert_eq!(vec,RowDVector::from_vec(vec![-2.0, 4.0, 8.0]))
/// ```
pub fn scale<D:Dim,S>(formula:&mut RowVector<f32,D,S>, factor:f32)
    where
        S:RawStorageMut<f32,U1,D>
{
    let _:Vec<_> = formula.iter_mut().map(|x| *x = *x * factor).collect();
}

/// Rewrite a linear formula by swapping a basic and non-basic variable
/// 
/// # Examples
/// Rewrite w = 4 + 2 x\
/// to x = -2 + 0.5 w 
/// ```
/// use nalgebra::RowDVector;
/// let mut formula = RowDVector::from_vec(vec![4.0, 2.0]);
/// lin_solver::solver::rewrite(&mut formula, 1);
/// 
/// assert_eq!(formula, RowDVector::from_vec(vec![-2.0, 0.5])) 
/// ```
pub fn rewrite<D:Dim, S>(formula:&mut RowVector<f32,D,S>, x_index:usize)
    where
        S:RawStorageMut<f32,U1,D>,
{
    let x_factor = formula[x_index];

    //swap  w = ... + q x + ...
    //to    q x = ... - w + ...
    let f_factor = -1.0/x_factor;
    formula[x_index] = -1.0; 

    //divide by q
    scale(formula,f_factor);
}

impl LinearSystem {
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

    /// Finds the optimal solution for the objective function of SystemEq\
    /// Under the constraints given.
    /// 
    /// # Examples
    /// ```
    /// use nalgebra::{DMatrix, RowDVector};
    /// let mut system = lin_solver::solver::LinearSystem{
    ///     objective:RowDVector::from_vec(vec![1.0,2.0]),
    ///     constraints:DMatrix::from_row_slice(2,2, &[
    ///         4.0, -2.0,
    ///         1.0, 1.0
    ///     ])
    /// };
    /// let res = system.solve();
    /// 
    /// assert_eq!(res,5.0);
    /// assert_eq!(system.constraints, DMatrix::from_row_slice(2,2, &[2.0,-0.5, 3.0, -0.5]));
    /// ```
    pub fn solve(&mut self) -> f32 {
        let mut i:u8 = 0;
        while i < 100 {
            if let Some(x_index) = self.first_positive() {
                self.rewrite_system(x_index);
            } else {
                return self.objective[0];
            }
            i += 1;
        }
        panic!("Couldn't solve linear system within 100 moves")
    }
    
    /// Given the index of a variable, will rewrite that variable as a basic variable
    /// 
    /// # Panics
    /// If x never occurs negatively in the constraints\
    /// i.e. the system is unbounded on x.
    /// 
    /// # Examples
    /// ```
    /// use nalgebra::{DMatrix, RowDVector};
    /// let mut system = lin_solver::solver::LinearSystem{
    ///     objective:RowDVector::from_vec(vec![1.0,2.0]),
    ///     constraints:DMatrix::from_row_slice(2,2, &[
    ///         4.0, -2.0,
    ///         1.0, 1.0
    ///     ])
    /// };
    /// system.rewrite_system(1);
    /// 
    /// println!("{}", system.constraints);
    /// assert_eq!(system.constraints, DMatrix::from_row_slice(2,2, &[2.0,-0.5, 3.0, -0.5]));
    /// assert_eq!(system.objective, RowDVector::from_vec(vec![5.0, -1.0]));
    /// ```
    pub fn rewrite_system(&mut self, x_index:usize) {
        //find most restrictive constraint
        let mut most_res:Option<(usize,f32)> = None;

        for (i, constraint) in self.constraints.row_iter().enumerate() {
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
            let mut formula :RowDVector<f32> = self.constraints.row(formula_index).clone_owned();
            rewrite(&mut formula, x_index);
            
            substitute(&formula, x_index, &mut self.objective.row_mut(0));
            for (i,mut constraint) in self.constraints.row_iter_mut().enumerate() {
                if i != formula_index {
                    substitute(&formula, x_index, &mut constraint);
                }
            }
            self.constraints.set_row(formula_index, &formula);
        } else {
            panic!("System is unbounded!");
        }
    }
}