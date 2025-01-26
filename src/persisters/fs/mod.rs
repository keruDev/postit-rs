//! Module for all the supported file formats. 
//! 
//! The currently supported formats are:
//! - csv
//! - json

mod csv;
mod json;

pub use csv::Csv;
pub use json::Json;