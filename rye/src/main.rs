use std::process;

use crate::utils::QuietExit;

mod bootstrap;
mod cli;
mod consts;
mod installer;
mod lock;
mod piptools;
mod platform;
mod pyproject;
mod sources;
mod sync;
mod utils;

pub fn main() -> Result<(), anyhow::Error> {
    match cli::execute() {
        Ok(()) => Ok(()),
        Err(err) => {
            if let Some(QuietExit(code)) = err.downcast_ref() {
                process::exit(*code);
            } else {
                Err(err)
            }
        }
    }
}
