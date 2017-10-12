use std::io::{self, ErrorKind};

/// An implementation of `std::io::Read` that fails on the first call to `read` and throws
/// an `std::io::Error` with the given ErrorKind.
#[derive(Debug, PartialEq)]
pub struct ErrReader {
	/// The ErrorKind to put into the `std::io::Error`.
	pub kind: ErrorKind,
}

impl ErrReader {
	/// Construct a new ErrReader.
	pub fn new(kind: ErrorKind) -> Self {
		Self {
			kind,
		}
	}
}

impl io::Read for ErrReader {
	fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
		Err(io::Error::new(self.kind, ""))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::io::Read;

	#[test]
	fn should_throw_correct_error() {
		let mut reader = ErrReader::new(ErrorKind::BrokenPipe);
		let res = reader.read(&mut [0; 3]);

		assert!(res.is_err());
		if let Err(err) = res {
			assert_eq!(io::ErrorKind::BrokenPipe, err.kind());
		}
	}
}
