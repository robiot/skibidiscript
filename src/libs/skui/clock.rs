use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::error;
use crate::interpreter::Interpreter;
use crate::parser::Expr;

#[derive(Debug)]
pub struct Clock {
    last_frame_time: Instant,  // Instant to store the last time a frame was rendered
    target_frame_duration: Duration,  // Target duration for each frame based on FPS
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            last_frame_time: Instant::now(),  // Initialize with current time
            target_frame_duration: Duration::from_secs(1) / 60,  // Default to 60 FPS
        }
    }
}

pub fn clock_set_fps_builtin(itp: &mut Interpreter, args: Vec<Expr>) -> Result<Expr, error::ParseError> {
    let fps = itp.expr_to_number(itp.consume_argument(&args, 1, 0)?)? as u32;

    let state = super::load_skui_state(itp)?;

    let clock = if let Some(clock) = &mut state.clock {
        clock
    } else {
        return Ok(Expr::Boolean(true));
    };

    // Calculate the target frame duration based on the provided FPS
    let target_frame_duration = Duration::from_secs(1) / fps;

    clock.target_frame_duration = target_frame_duration;

    Ok(Expr::Boolean(true))
}

pub fn clock_tick_builtin(itp: &mut Interpreter, _args: Vec<Expr>) -> Result<Expr, error::ParseError> {
    let state = super::load_skui_state(itp)?;

    let clock = if let Some(clock) = &mut state.clock {
        clock
    } else {
        return Ok(Expr::Boolean(true));
    };

    // Get current time
    let current_time = Instant::now();

    // Calculate the elapsed time since the last frame
    let elapsed_time = current_time.duration_since(clock.last_frame_time);

    // Sleep if necessary to maintain the target frame duration
    if elapsed_time < clock.target_frame_duration {
        let sleep_duration = clock.target_frame_duration - elapsed_time;
        sleep(sleep_duration);
    }

    // Update the last frame time to the current time
    let updated_time = Instant::now();  // Capture the time after sleeping
    let frame_time = updated_time.duration_since(clock.last_frame_time);  // The time taken for this frame

    // Update last_frame_time to the current time for the next tick
    clock.last_frame_time = updated_time;

    // Return the frame time in milliseconds (like Pygame's clock.tick() return value)
    let frame_time_ms = frame_time.as_millis() as i64;  // Convert to milliseconds

    Ok(Expr::Number(frame_time_ms))
}
