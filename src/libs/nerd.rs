use std::collections::HashMap;

use crate::{error, interpreter::Interpreter, parser::Expr};

use super::{LibFunctions, Library};

pub fn load_nerd_library() -> Library {
    let mut functions: LibFunctions = HashMap::new();
    functions.insert("randInt".to_string(), rand_int_builtin);
    // Add more functions as needed

    Library { functions }
}

fn rand_int_builtin(
    itp: &mut Interpreter,
    args: Vec<Expr>,
) -> Result<Expr, error::ParseError> {
    // Implement the randInt function logic here
    // if args.len() != 2 {
    //     return Err(error::ParseError::ArgumentMismatch {
    //         expected: 2,
    //         found: args.len(),
    //         line: itp.line,
    //     });
    // }

    // let min = itp.evaluate_expression(args[0].clone());
    // let max = itp.evaluate_expression(args[1].clone());

    // let min = match min {
    //     Ok(Expr::Number(n)) => n,
    //     _ => {
    //         return Err(error::ParseError::InvalidArgument {
    //             expected: "number".to_string(),
    //             found: "expression".to_string(),
    //             line: itp.line,
    //         });
    //     }
    // };
    // if min > max {
    //     panic!("randInt min should be less than max");
    // }
    // use rand::Rng;
    // let mut rng = rand::thread_rng();
    // rng.gen_range(min..=max)
    Ok(Expr::Number(0))
}
