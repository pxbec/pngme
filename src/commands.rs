use std::fs;
use clap::Subcommand;
use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};
use crate::png::Png;
use crate::chunk::Chunk;

#[derive(Subcommand)]
#[command(about = "PNG file manipulation commands")]
pub enum Commands {
	#[command(visible_alias = "e")]
	Encode(EncodeArgs),

	#[command(visible_alias = "d")]
	Decode(DecodeArgs),

	#[command(visible_alias = "rm")]
	Remove(RemoveArgs),

	#[command(visible_alias = "p")]
	Print(PrintArgs),
}

pub fn encode(args: EncodeArgs) -> crate::Result<()> {
	let input_path = args.input.as_path();
	let output_path = args.output.as_deref().unwrap_or(input_path);

	let file_content = fs::read(input_path)?;
	let mut png = Png::try_from(file_content.as_slice())?;
	let chunk = Chunk::new(args.chunk_type, args.message.as_bytes().to_vec());
	png.append_chunk(chunk);
	fs::write(output_path, png.as_bytes())?;
	Ok(())
}

pub fn decode(args: DecodeArgs) -> crate::Result<()> {
	let file_content = fs::read(&args.input.as_path())?;
	let mut png = Png::try_from(file_content.as_slice())?;
	let chunk = png.remove_first_chunk(&args.chunk_type);
	if let Some(chunk) = chunk {
		println!("{}", chunk.data_as_str()?);
	} else {
		println!("No message found.")
	}
	Ok(())
}

pub fn remove(args: RemoveArgs) -> crate::Result<()> {
	let file_content = fs::read(&args.input.as_path())?;
	let mut png = Png::try_from(file_content.as_slice())?;
	if let Some(chunk) = png.remove_first_chunk(&args.chunk_type) {
		fs::write(&args.input, png.as_bytes())?;
		println!("Chunk with content \"{}\" removed.", chunk.data_as_str()?);
	} else {
		println!("No chunk found.");
	}
	Ok(())
}

pub fn print(args: PrintArgs) -> crate::Result<()> {
	let input_bytes = fs::read(&args.input.as_path())?;
	let png = Png::try_from(input_bytes.as_slice())?;
	for chunk in png.chunks() {
        println!("{}", chunk);
    }
	Ok(())
}