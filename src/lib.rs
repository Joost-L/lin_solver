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