use std::collections::HashMap;

use crate::{error, interpreter::Interpreter, parser::Expr};

use super::{BuiltinFunction, LibFunctions, Library};




struct MyStruct {
    // Define some functions.
    pub state_number: i32,
}

impl MyStruct {
    fn foo(&self) {
        self.state_number += 1;
        println!("Called foo");
    }
    
    fn bar(&self) {
        println!("Called bar");
    }
    
    fn baz(&self) {
        println!("Called baz");
    }
}



fn vas() {
    let my_struct = MyStruct {state_number: 0};
    
    // Define a HashMap to store the function pointers.
    let mut lib_functions: HashMap<String, fn(&MyStruct)> = HashMap::new();
    
    // // Populate the HashMap with function pointers.
    // map.insert("foo".to_string(), MyStruct::foo);
    // map.insert("bar".to_string(), MyStruct::bar);
    // map.insert("baz".to_string(), MyStruct::baz);


    // let mut lib_functions: HashMap<String, BuiltinFunction> = HashMap::new();
    
    // Populate the HashMap with function pointers.
    lib_functions.insert("foo".to_string(), );
    
    
    // Example usage:
    if let Some(&func) = map.get("foo") {
        func(&my_struct); // Calls MyStruct::foo
    }
    
    if let Some(&func) = map.get("bar") {
        func(&my_struct); // Calls MyStruct::bar
    }
    
    if let Some(&func) = map.get("baz") {
        func(&my_struct); // Calls MyStruct::baz
    }
}


struct NerdLibrary {
    functions: HashMap<String, BuiltinFunction>,

    pub state_number: i32,
}

impl NerdLibrary {
    pub fn new() -> Self {
        let functions: LibFunctions = HashMap::new();
        functions.insert("testingVTWO".to_string(), testing_v2);
        // Add more functions as needed

        Self {
            functions,
            state_number: 0,
        }
    }

    pub fn set_number(
        &mut self,
        _itp: &mut Interpreter,
        _args: Vec<Expr>,
    ) -> Result<Expr, error::ParseError> {
        self.state_number += 1;

        Ok(Expr::Number(0))
    }

    pub fn get_number(
        &mut self,
        _itp: &mut Interpreter,
        _args: Vec<Expr>,
    ) -> Result<Expr, error::ParseError> {
        // self.state_number += 1;
        println!("State number: {}", self.state_number);

        Ok(Expr::Number(0))
    }
}


pub fn load_nerd_library() -> Library {
    let lib = NerdLibrary::new();

    let mut functions: LibFunctions = HashMap::new();
    functions.insert("randInt".to_string(), lib.get_number);
    // Add more functions as needed

    Library { functions }
}
