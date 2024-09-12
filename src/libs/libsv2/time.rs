use std::collections::HashMap;

use crate::parser::Expr;

pub struct Library {
    pub functions: HashMap<String, BuiltinFunction>,
    // pub state: LibState,
}

pub type BuiltinFunction = fn(&mut Interpreter, Vec<Expr>) -> Expr;

pub type LibFunctions = HashMap<String, BuiltinFunction>;

pub struct Interpreter {
    pub libs: HashMap<String, Library>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            libs: HashMap::new(),
        }
    }

    pub fn load_library(&mut self, name: &str, lib: Library) {
        self.libs.insert(name.to_string(), lib);
    }

    // pub fn call_function(&mut self, lib_name: &str, func_name: &str, args: Vec<Expr>) -> Expr {
    //     let lib = self.libs.get_mut(lib_name).unwrap();
    //     let func = lib.functions.get(func_name).unwrap();

    //     func(self, args)
    // }
}

// TestLib
struct LibState {
    state_number: i32,
}

struct TestLib {
    state: LibState,
}

impl TestLib {
    fn new() -> Self {
        TestLib {
            state: LibState { state_number: 0 },
        }
    }

    fn increate_number(&mut self) {
        self.state.state_number += 1;
    }
}

pub fn start()  {
    let mut itp = Interpreter::new();

    let mut functions: LibFunctions = HashMap::new();

    let lib = TestLib::new();
    
    functions.insert("increase_number".to_string(), Box::new(move |test_lib: &mut TestLib, _args: Vec<i32>| {
        test_lib.increate_number();
    }) as Box<dyn FnMut(&mut TestLib, Vec<i32>)>);

    let lib = Library {
        functions: HashMap::new(),
    };

    itp.load_library("test", lib);

    // let args = vec![];
    // itp.call_function("test", "test", args);
}
