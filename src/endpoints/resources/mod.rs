pub mod credits;
pub mod voices;
pub mod images;

use std::{fs::File, io::{self, Read, Write}};
pub use crate::client::*;
pub use crate::prelude::*;
pub use serde::{Deserialize, Serialize};

use rand::Rng;

pub struct MultipartFormData {
    pub boundary: String,
    pub body: Vec<u8>,
}

impl MultipartFormData {
    pub fn new() -> Self {
        Self {
            boundary: format!(
                "-----------------------------{}", rand::thread_rng().gen::<u64>()),
            body: Vec::new(),
        }
    }

    pub fn add_text(&mut self, name: &str, value: &str) -> io::Result<()> {
        write!(self.body, "--{}\r\n", self.boundary)?;
        write!(self.body, "Content-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n", name, value)?;
        Ok(())
    }

    pub fn add_file(&mut self, name: &str, path: &str) -> io::Result<()> {
        if !path.contains(".") {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid file path"));
        }
        write!(self.body, "--{}\r\n", self.boundary)?;
        write!(self.body, "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n", name, path)?;
        write!(self.body, "Content-Type: image/{}\r\n\r\n", path.split_once(".").unwrap().1)?;
        let mut file = File::open(path)?;
        file.read_to_end(&mut self.body)?;
        write!(self.body, "\r\n")?;


        Ok(())
    }

    pub fn end_body(&mut self) -> io::Result<()> {
        write!(self.body, "--{}--\r\n", self.boundary)?;
        Ok(())
    }

}
