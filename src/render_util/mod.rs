mod button;
mod combo;
mod helper;
mod input;
mod input_text;
mod popup;
mod spinner;
mod style;
mod text;
mod tree;

pub use self::{
    button::*, combo::*, helper::*, input::*, input_text::*, popup::*, spinner::*, style::*,
    text::*, tree::*,
};

use nexus::imgui::{sys, Ui};
use std::ptr;

pub type Point = [f32; 2];

pub type Rect = (Point, Point);

pub fn next_window_size_constraints(size_min: [f32; 2], size_max: [f32; 2]) {
    unsafe {
        sys::igSetNextWindowSizeConstraints(size_min.into(), size_max.into(), None, ptr::null_mut())
    }
}

pub fn cycle_progress(ui: &Ui, period_ms: u32) -> f32 {
    let time = (1000.0 * ui.time()) as u32;
    let passed = time % period_ms;
    passed as f32 / period_ms as f32
}
