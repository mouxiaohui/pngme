use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};

use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Read, Write},
    path::PathBuf,
    str::FromStr,
};

use crate::args::{DecodeArgs, EncodeArgs, PngMeArgs, PrintArgs, RemoveArgs};

pub fn run_cmd(pngme_args: PngMeArgs) -> Result<()> {
    match pngme_args {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print(args),
    }
}

fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = read_png_file(&args.file_path)?;

    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.into());

    png.append_chunk(chunk);

    match args.output_file {
        Some(path) => write_png_file(&path, png)?,
        None => write_png_file(&args.file_path, png)?,
    }

    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let png = read_png_file(&args.file_path)?;
    let chunk = png
        .chunk_by_type(&args.chunk_type)
        .ok_or(Error::new(ErrorKind::InvalidInput, "Chunk Type not found"))?;

    println!("Decode Message: {}", chunk.data_as_string()?);

    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = read_png_file(&args.file_path)?;
    png.remove_chunk(&args.chunk_type)?;
    write_png_file(&args.file_path, png)?;

    println!("Remove the success!");

    Ok(())
}

fn print(args: PrintArgs) -> Result<()> {
    let png = read_png_file(&args.file_path)?;
    for chunk in png.chunks() {
        if let Ok(data) = chunk.data_as_string() {
            println!("Chunk Type: {}, Data: {}", chunk.chunk_type(), data)
        };
    }

    Ok(())
}

fn read_png_file(path: &PathBuf) -> Result<Png> {
    let mut contents: Vec<u8> = Vec::new();
    let file = File::open(path)?;
    let mut buf = BufReader::new(file);
    buf.read_to_end(&mut contents)?;

    Png::try_from(contents.as_ref())
}

fn write_png_file(path: &PathBuf, png: Png) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(png.as_bytes().as_ref())?;

    Ok(())
}
