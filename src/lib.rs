mod meta;

// #[macro_use]
pub extern crate serde;

use serde::{Serialize,Deserialize};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;