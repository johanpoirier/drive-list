use std::{io};
use data::*;

pub trait Platform {
    fn new() -> Self;

    /// Returns a vector of filesystem drives information objects.
    fn drives(&self) -> io::Result<Vec<Drive>>;
}
