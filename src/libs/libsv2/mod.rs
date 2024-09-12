use std::collections::HashMap;

use winit::window::Window;

use crate::parser::Expr;

pub mod time;
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
    // state_number: i32,
    window: Option<Window>,
}

struct TestLib {
    state: LibState,
}

impl TestLib {
    fn new() -> Self {
        TestLib {
            state: LibState { window: None },
        }
    }

    fn create_window(&mut self) {
        self.state.window = Window::new(); // or whatever similar
    }

    fn place_text(&mut self) {
        self.state.window.add_text("Hello, world!");
    }
}

pub fn start()  {
    let mut functions = HashMap::new();

    let lib = TestLib::new();
    
    functions.insert("createWindow",  lib.create_window);
    functions.insert("placeText",  lib.place_text);

    let mut itp = Interpreter::new();

    itp.load_library("testlib", lib);

    itp.call_function("testlib", "createWindow", vec![]);

    // outputs: 1
    itp.call_function("testlib", "placeText", vec![]);
}
