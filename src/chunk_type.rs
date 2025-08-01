#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
	type_code: [u8; 4],
}

impl ChunkType {
	pub fn bytes(&self) -> [u8; 4] {
		self.type_code
	}

	pub fn is_critical(&self) -> bool {
		self.type_code[0].is_ascii_uppercase()
	}

	pub fn is_public(&self) -> bool {
		self.type_code[1].is_ascii_uppercase()
	}

	pub fn is_reserved_bit_valid(&self) -> bool {
		self.type_code[2].is_ascii_uppercase()
	}

	pub fn is_safe_to_copy(&self) -> bool {
		self.type_code[3].is_ascii_lowercase()
	}

	pub fn is_valid(&self) -> bool {
		self.type_code.iter().all(|b| b.is_ascii_alphabetic())
			&& self.is_reserved_bit_valid()
	}
}

impl std::fmt::Display for ChunkType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s: String = self.type_code.iter().map(|&b| b as char).collect();
		write!(f, "{s}")
	}
}

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
	#[error("chunk type contains non-ASCII characters")]
	NonAscii,

	#[error("chunk type has reserved bit set to 0 (must be 1)")]
	ReservedBit,
}

impl TryFrom<[u8; 4]> for ChunkType {
	type Error = ValidationError;

	fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
		let ct = ChunkType { type_code: value };

		if !ct.type_code.iter().all(|b| b.is_ascii_alphabetic()) {
			return Err(Self::Error::NonAscii);
		}

		if !ct.is_reserved_bit_valid() {
			return Err(Self::Error::ReservedBit);
		}

		Ok(ct)
	}
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
	#[error("expected 4 bytes, but was given {0}")]
	InvalidLength(usize),

	#[error("error converting string slice '{0}' into array")]
	FromSlice(#[from] std::array::TryFromSliceError),
}

impl ChunkType {
	// Helper for the FromStr impl below.
	/// Parses the given string into a 4-byte array.
	fn parse(s: &str) -> Result<[u8; 4], ParseError> {
		let bytes = s.as_bytes();
		let length = bytes.len();
		if length > 4 {
			return Err(ParseError::InvalidLength(length));
		}

		let type_code: [u8; 4] = bytes.try_into()?;
		Ok(type_code)
	}
}

#[derive(thiserror::Error, Debug)]
pub enum ChunkTypeError {
	#[error(transparent)]
	Parse(#[from] ParseError),

	#[error(transparent)]
	Validation(#[from] ValidationError),
}

impl std::str::FromStr for ChunkType {
	type Err = ChunkTypeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let type_code = Self::parse(s)?;

		Ok(ChunkType::try_from(type_code)?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use std::convert::TryFrom;
	use std::str::FromStr;

	#[test]
	pub fn test_chunk_type_from_bytes() {
		let expected = [82, 117, 83, 116];
		let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

		assert_eq!(expected, actual.bytes());
	}

	#[test]
	pub fn test_chunk_type_from_str() {
		let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
		let actual = ChunkType::from_str("RuSt").unwrap();
		assert_eq!(expected, actual);
	}

	#[test]
	pub fn test_chunk_type_is_critical() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_critical());
	}

	#[test]
	pub fn test_chunk_type_is_not_critical() {
		let chunk = ChunkType::from_str("ruSt").unwrap();
		assert!(!chunk.is_critical());
	}

	#[test]
	pub fn test_chunk_type_is_public() {
		let chunk = ChunkType::from_str("RUSt").unwrap();
		assert!(chunk.is_public());
	}

	#[test]
	pub fn test_chunk_type_is_not_public() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(!chunk.is_public());
	}

	#[test]
	pub fn test_chunk_type_is_reserved_bit_valid() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_reserved_bit_valid());
	}

	#[test]
	pub fn test_chunk_type_is_reserved_bit_invalid() {
		let raw = ChunkType { type_code: *b"Rust" };
		assert!(!raw.is_reserved_bit_valid());
	}

	#[test]
	pub fn test_chunk_type_is_safe_to_copy() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_safe_to_copy());
	}

	#[test]
	pub fn test_chunk_type_is_unsafe_to_copy() {
		let chunk = ChunkType::from_str("RuST").unwrap();
		assert!(!chunk.is_safe_to_copy());
	}

	#[test]
	pub fn test_valid_chunk_is_valid() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_valid());
	}

	#[test]
	pub fn test_invalid_chunk_is_rejected() {
		assert!(ChunkType::from_str("Rust").is_err());
		assert!(ChunkType::from_str("Ru1t").is_err());

		let raw = ChunkType { type_code: *b"Ru1t" };
		assert!(!raw.is_valid());
	}

	#[test]
	pub fn test_chunk_type_string() {
		let chunk = ChunkType::from_str("RuSt").unwrap();
		assert_eq!(&chunk.to_string(), "RuSt");
	}

	#[test]
	pub fn test_chunk_type_trait_impls() {
		let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
		let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
		let _chunk_string = format!("{}", chunk_type_1);
		let _are_chunks_equal = chunk_type_1 == chunk_type_2;
	}
}
