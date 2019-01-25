use std::{io};
use data::*;
use super::common::*;

pub struct PlatformImpl;

impl Platform for PlatformImpl {
    #[inline(always)]
    fn new() -> Self {
        PlatformImpl
    }

    fn drives(&self) -> io::Result<Vec<Drive>> {
        let empty = vec![];
        Ok(empty)
    }
}