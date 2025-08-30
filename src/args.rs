use std::str::FromStr;
use std::path::PathBuf;
use clap::Args;
use crate::chunk_type::ChunkType;


#[derive(Args)]
pub struct EncodeArgs {
    #[arg(short, long, help = "Path to the PNG file to process")]
    pub(crate) input: PathBuf,

    #[arg(value_parser = ChunkType::from_str, help = "Chunk type (4 ASCII letters)")]
    pub(crate) chunk_type: ChunkType,

    #[arg(help = "Message to embed in the PNG file")]
    pub(crate) message: String,

    #[arg(short, long, value_name = "FILE", help = "Output file path (defaults to input file if not specified)")]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
	#[arg(short, long, help = "Path to the PNG file to process")]
	pub(crate) input: PathBuf,

	#[arg(value_parser = ChunkType::from_str)]
	pub(crate) chunk_type: ChunkType,
}

#[derive(Args)]
pub struct RemoveArgs {
	#[arg(short, long, help = "Path to the PNG file to process")]
	pub(crate) input: PathBuf,

	/// 4-character chunk type
	#[arg(value_parser = ChunkType::from_str)]
	pub(crate) chunk_type: ChunkType,
}

#[derive(Args)]
pub struct PrintArgs {
	#[arg(short, long, help = "Path to the PNG file to process")]
	pub(crate) input: PathBuf,
}