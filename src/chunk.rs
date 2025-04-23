use std::{str, usize};
use crate::chunk_type::ChunkType;


#[derive(Debug, Clone)]
pub struct Chunk {
	chunk_type: ChunkType,
	data: Vec<u8>,
}

impl Chunk {
	pub const OVERHEAD_BYTES: usize = 12;
	
	pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
		Chunk {
			chunk_type,
			data: chunk_data,
		}
	}

	pub fn length(&self) -> u32 {
		self.data.len() as u32
	}

	pub fn chunk_type(&self) -> &ChunkType {
		&self.chunk_type
	}

	pub fn data(&self) -> &[u8] {
		self.data.as_slice()
	}

	pub fn crc(&self) -> u32 {
		const PNG_CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
		let mut digest = PNG_CRC.digest();
		digest.update(&self.chunk_type.bytes());
		digest.update(&self.data);
		digest.finalize()
	}

	pub fn data_as_str(&self) -> Result<&str, str::Utf8Error> {
		str::from_utf8(&self.data)
	}

	pub fn as_bytes(&self) -> Vec<u8> {
		self.length()
			.to_be_bytes()
			.iter()
			.chain(self.chunk_type.bytes().iter())
			.chain(self.data.iter())
			.chain(self.crc().to_be_bytes().iter())
			.copied()
			.collect()
	}
}

impl std::fmt::Display for Chunk {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let chunk_type_str = self.chunk_type.to_string();

		let data_str = match str::from_utf8(&self.data) {
			Ok(s) => s.to_string(),
			Err(_) => format!("{:?}", &self.data),
		};

		write!(f, "Chunk Type: {}\nData: {}", chunk_type_str, data_str)
	}
}

impl TryFrom<&[u8]> for Chunk {
	type Error = ();

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		// Minimum chunk size: 4 (len) + 4 (type) + 0 (msg) + 4 (crc)
		if value.len() < Self::OVERHEAD_BYTES {
			return Err(());
		}

		let length = u32::from_be_bytes(value[0..4].try_into().map_err(|_| ())?) as usize;
		if value.len() - Self::OVERHEAD_BYTES < length {
			return Err(());
		}

		let type_code: [u8; 4] = value[4..8].try_into().map_err(|_| ())?;
		let chunk_type = ChunkType::try_from(type_code)?;
		
		let data = value[8..8 + length].to_vec();
		
		let chunk = Chunk {
			chunk_type,
			data,
		};
		
		let crc_start = 8 + length;
		let crc = u32::from_be_bytes(value[crc_start..crc_start + 4].try_into().map_err(|_| ())?);
		if crc != chunk.crc() {
			return Err(());
		}

		Ok(chunk)
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use crate::chunk_type::ChunkType;
	use std::str::FromStr;

	fn testing_chunk() -> Chunk {
		let data_length: u32 = 42;
		let chunk_type = "RuSt".as_bytes();
		let message_bytes = "This is where your secret message will be!".as_bytes();
		let crc: u32 = 2882656334;

		let chunk_data: Vec<u8> = data_length
			.to_be_bytes()
			.iter()
			.chain(chunk_type.iter())
			.chain(message_bytes.iter())
			.chain(crc.to_be_bytes().iter())
			.copied()
			.collect();

		Chunk::try_from(chunk_data.as_ref()).unwrap()
	}

	#[test]
	fn test_new_chunk() {
		let chunk_type = ChunkType::from_str("RuSt").unwrap();
		let data = "This is where your secret message will be!".as_bytes().to_vec();
		let chunk = Chunk::new(chunk_type, data);
		assert_eq!(chunk.length(), 42);
		assert_eq!(chunk.crc(), 2882656334);
	}

	#[test]
	fn test_chunk_length() {
		let chunk = testing_chunk();
		assert_eq!(chunk.length(), 42);
	}

	#[test]
	fn test_chunk_type() {
		let chunk = testing_chunk();
		assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
	}

	#[test]
	fn test_chunk_string() {
		let chunk = testing_chunk();
		let chunk_string = chunk.data_as_str().unwrap();
		let expected_chunk_string = String::from("This is where your secret message will be!");
		assert_eq!(chunk_string, expected_chunk_string);
	}

	#[test]
	fn test_chunk_crc() {
		let chunk = testing_chunk();
		assert_eq!(chunk.crc(), 2882656334);
	}

	#[test]
	fn test_valid_chunk_from_bytes() {
		let data_length: u32 = 42;
		let chunk_type = "RuSt".as_bytes();
		let message_bytes = "This is where your secret message will be!".as_bytes();
		let crc: u32 = 2882656334;

		let chunk_data: Vec<u8> = data_length
			.to_be_bytes()
			.iter()
			.chain(chunk_type.iter())
			.chain(message_bytes.iter())
			.chain(crc.to_be_bytes().iter())
			.copied()
			.collect();

		let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

		let chunk_string = chunk.data_as_str().unwrap();
		let expected_chunk_string = String::from("This is where your secret message will be!");

		assert_eq!(chunk.length(), 42);
		assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
		assert_eq!(chunk_string, expected_chunk_string);
		assert_eq!(chunk.crc(), 2882656334);
	}


	#[test]
	fn test_invalid_chunk_from_bytes() {
		let data_length: u32 = 42;
		let chunk_type = "RuSt".as_bytes();
		let message_bytes = "This is where your secret message will be!".as_bytes();
		let crc: u32 = 2882656333;

		let chunk_data: Vec<u8> = data_length
			.to_be_bytes()
			.iter()
			.chain(chunk_type.iter())
			.chain(message_bytes.iter())
			.chain(crc.to_be_bytes().iter())
			.copied()
			.collect();

		let chunk = Chunk::try_from(chunk_data.as_ref());

		assert!(chunk.is_err());
	}

	#[test]
	pub fn test_chunk_trait_impls() {
		let data_length: u32 = 42;
		let chunk_type = "RuSt".as_bytes();
		let message_bytes = "This is where your secret message will be!".as_bytes();
		let crc: u32 = 2882656334;

		let chunk_data: Vec<u8> = data_length
			.to_be_bytes()
			.iter()
			.chain(chunk_type.iter())
			.chain(message_bytes.iter())
			.chain(crc.to_be_bytes().iter())
			.copied()
			.collect();

		let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

		let _chunk_string = format!("{}", chunk);
	}
}
