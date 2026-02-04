use thiserror::Error;

use std::ffi::NulError;

#[derive(Error, Debug)]

pub enum DwinError {
    #[error("display {0} not found")]
    DisplayNotFound(String),

    #[error("{0}")]
    NulString(#[from] NulError),
}

pub struct Dwin {

}

impl Dwin {
    pub fn new(display_name: &str) -> Result<Self, DwinError> {
        Ok(Dwin {})
    }

    pub fn init(&self) -> Result<(), DwinError> {
        Ok(())
    }

    pub fn run(&self) {
        println!("dwin running");
    }
}