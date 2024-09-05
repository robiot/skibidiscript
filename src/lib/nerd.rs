use rand::Rng;

use crate::parser::Expr;

pub struct NerdLib;

impl NerdLib {
    pub fn execute_function_call(name: &str, args: Vec<Expr>) -> Result<Expr, String> {
        match name {
            "randInt" => {
                // if args.len() == 2 {
                //     let (min, max) = (args[0], args[1]);
                //     Ok(NerdLib::rand_int(Expr::Number(min), max))
                // } else {
                //     Err(format!("randInt requires 2 arguments, but got {}", args.len()))
                // }

                // todo figure out a good way to do builtIn function definitions
                Ok(Expr::Number(1))
            }
            _ => Err(format!("Unknown function: {}", name)),
        }
    }

    fn rand_int(min: i64, max: i64) -> Expr {
        Expr::Number(rand::thread_rng().gen_range(min..=max))
    }
}