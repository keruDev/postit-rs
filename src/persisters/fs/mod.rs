//! Module for file management.
//!
//! The currently supported formats are:
//! - csv
//! - json
//! - xml

mod csv;
mod file;
mod json;
mod xml;

pub use csv::Csv;
pub use file::{File, Format};
pub use json::Json;
pub use xml::Xml;
