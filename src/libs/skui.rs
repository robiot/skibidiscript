use super::{LibFunctions, LibState, Library};
use crate::{error, interpreter::Interpreter, parser::Expr};

use std::collections::HashMap;
use std::time::Duration;

use std::thread::sleep;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::pump_events::EventLoopExtPumpEvents;
use winit::platform::pump_events::PumpStatus;
use winit::window::{Window, WindowId};

pub const LIBRARY_NAME: &str = "skui";

#[derive(Debug)]
struct WindowInfo {
    width: u32,
    height: u32,
    title: String,
}

#[derive(Default, Debug)]
pub struct SkuiState {
    window: Option<Window>,
    window_info: Option<WindowInfo>,

    // Stuff
    active_event: Option<WindowEvent>,
}


// https://www.reddit.com/r/rust/comments/1dnaase/rust_and_winit_0303/
pub fn load_skui_library() -> Library {
    let mut functions: LibFunctions = HashMap::new();
    functions.insert("createWindow".to_string(), create_window);

    Library {
        functions,
        state: LibState::SkuiState(SkuiState {
            window: None,
            window_info: None,
            active_event: None,
        }),
    }
}

// All functions
fn create_window(itp: &mut Interpreter, args: Vec<Expr>) -> Result<Expr, error::ParseError> {
    let width = itp.expr_to_number(itp.consume_argument(&args, 3, 0)?)? as u32;
    let height = itp.expr_to_number(itp.consume_argument(&args, 3, 1)?)? as u32;
    let title = itp.expr_to_string(itp.consume_argument(&args, 3, 2)?)?;

    // Ensure dimensions are valid
    if width <= 0 || height <= 0 {
        return Err(error::ParseError::GeneralError {
            line: itp.line,
            message: "Window dimensions must be greater than zero.".to_string(),
        });
    }

    println!(
        "Creating window with width: {} and height: {}",
        width, height
    );

    let mut event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = SkuiState::new(WindowInfo {
        width,
        height,
        title,
    });
    // event_loop.run_app(&mut app).unwrap();

    // we got a golden start
    let status = event_loop.pump_app_events(Some(Duration::ZERO), &mut app);

    // loop {
    //     let timeout = Some(Duration::ZERO);
    //     let status = event_loop.pump_app_events(timeout, &mut app);

    //     if let PumpStatus::Exit(exit_code) = status {
    //         break;
    //     }

    //     // Sleep for 1/60 second to simulate application work
    //     //
    //     // Since `pump_events` doesn't block it will be important to
    //     // throttle the loop in the app somehow.
    //     println!("Update()");
    //     sleep(Duration::from_millis(16));
    // }

    // createa a window using winit

    Ok(Expr::Boolean(true))
}


// The state of the skui/ winit wrapper
impl SkuiState {
    fn new(window_info: WindowInfo) -> Self {
        SkuiState {
            window: None,
            window_info: Some(window_info),
            active_event: None,
        }
    }

    fn get_event(&mut self) -> Expr {
        let event = self.active_event.take().unwrap();

        let event_code = match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                // event_loop.exit();

                "close_requested"
            }
            // WindowEvent::RedrawRequested => {
            //     // Redraw the application.
            //     //
            //     // It's preferable for applications that do not render continuously to render in
            //     // this event rather than in AboutToWait, since rendering in here allows
            //     // the program to gracefully handle redraws requested by the OS.

            //     // Draw.

            //     // Queue a RedrawRequested event.
            //     //
            //     // You only need to call this if you've determined that you need to redraw in
            //     // applications which do not always need to. Applications that redraw continuously
            //     // can render here instead.
            //     // self.window.as_ref().unwrap().request_redraw();

            // }
            _ => "",
        };

        Expr::StringLiteral(event_code.to_string())
    }
}

impl ApplicationHandler for SkuiState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_info = self.window_info.as_ref().unwrap();

        let window_attributes = Window::default_attributes()
            .with_inner_size(winit::dpi::LogicalSize::new(
                window_info.width,
                window_info.height,
            ))
            .with_title(window_info.title.clone()); // Set the window title

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.active_event = Some(event.clone());

        println!("frame");
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            // WindowEvent::RedrawRequested => {
            //     // Redraw the application.
            //     //
            //     // It's preferable for applications that do not render continuously to render in
            //     // this event rather than in AboutToWait, since rendering in here allows
            //     // the program to gracefully handle redraws requested by the OS.

            //     // Draw.

            //     // Queue a RedrawRequested event.
            //     //
            //     // You only need to call this if you've determined that you need to redraw in
            //     // applications which do not always need to. Applications that redraw continuously
            //     // can render here instead.
            //     self.window.as_ref().unwrap().request_redraw();
            // }
            _ => (),
        }
    }
}
