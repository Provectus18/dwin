use std::{
    ffi::{CString, NulError},
    mem::zeroed,
};

use x11::xlib;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DwinError {
    #[error("display {0} not found")]
    DisplayNotFound(String),

    #[error("{0}")]
    NulString(#[from] NulError),
}

pub struct Dwin {
    display: *mut xlib::Display,
}

impl Dwin {
    pub fn new(display_name: &str) -> Result<Self, DwinError> {
        let display: *mut xlib::Display =
            unsafe { xlib::XOpenDisplay(CString::new(display_name)?.as_ptr()) };

        if display.is_null() {
            return Err(DwinError::DisplayNotFound(display_name.into()));
        }

        Ok(Dwin { display })
    }

    pub fn init(&self) -> Result<(), DwinError> {
        unsafe {
            xlib::XSelectInput(
                self.display,
                xlib::XDefaultRootWindow(self.display),
                xlib::SubstructureRedirectMask,
            );
        }

        Ok(())
    }

    pub fn run(&self) {
        println!("dwin running");
        let mut event: xlib::XEvent = unsafe { zeroed() };
        loop {
            unsafe {
                xlib::XNextEvent(self.display, &mut event);

                match event.get_type() {
                    xlib::MapRequest => {
                        self.create_window(event);
                    }
                    _ => {
                        println!("unknown event {:?}", event);
                    }
                }
            }
        }
    }

    fn create_window(&self, event: xlib::XEvent) {
        let event: xlib::XMapRequestEvent = From::from(event);
        unsafe { xlib::XMapRaised(self.display, event.window) };
    }
}