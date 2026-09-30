extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SQLBridge;

impl SQLBridge {
    pub fn new() -> Self {
        SQLBridge
    }

    pub fn add_to_mirror(&mut self, _pkg: String, _data: Vec<u8>) {
        // Logik mirror
    }
}
