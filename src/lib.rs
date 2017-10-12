//! Collection of utilities for testing failure of io components, e.g. `std::io::Read`.
//!
//! # Usage
//!
//! ### In your Cargo.toml:
//!
//! ```toml
//! [dev-dependencies]
//! io-test-util = "*"
//! ```
//!
//! ### In your test:
//!
//! ```
//! extern crate io_test_util;
//! use io_test_util::ErrReader;
//! use std::io::{ErrorKind, Read};
//!
//! pub fn main() {
//!		let mut reader = ErrReader::new(ErrorKind::BrokenPipe);
//!		let res = reader.read(&mut[0; 1]);
//! 	assert!(res.is_err());
//! }
//! ```

mod error_reader;

pub use self::error_reader::ErrReader;
