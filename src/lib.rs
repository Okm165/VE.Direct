#![cfg(not(doctest))]
//! # Example
//! ```
//! use tokio::io::AsyncReadExt;
//! use tokio_serial::SerialPortBuilderExt;
//! use std::time::Duration;
//! use crate::converter::convert;
//! use crate::parser::Parser;
//!
//! #[tokio::main]
//! async fn main() -> tokio_serial::Result<()> {
//!     // initialize serial port
//!     let mut port = tokio_serial::new("/dev/serial0", 19200)
//!         .timeout(Duration::from_secs(5))
//!         .open_native_async()?;
//!     
//!     // initialize buffer and parser
//!     let mut buf: Vec<u8> = vec![0; 2048];
//!     let mut parser = Parser::new();
//!
//!     loop{
//!         // read loop
//!         if let Ok(r) = port.read(&mut buf).await {
//!             // data from serial port are served in chunks so it takes couple loops to get one packet parsed
//!             if let Ok(parsed) = parser.parse_slice(&buf[..r]) {
//!                 // when it is parsed do conversion
//!                 println!("{:?}", convert(parsed));
//!             }
//!         }
//!
//!     }
//! }
//! ```

pub mod converter;
pub mod parser;

#[cfg(test)]
mod tests;

pub use self::converter::convert;
pub use self::converter::models::*;
pub use self::parser::models::*;
pub use self::parser::Parser;
