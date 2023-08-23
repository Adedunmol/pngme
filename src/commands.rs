use std::{path::PathBuf, fs, str::FromStr, process};

use crate::{args::{Cli, Commands}, Result, chunk_type::ChunkType, chunk::Chunk, png::Png};

pub fn run(args: &Cli) -> Result<()> {

        if let Commands::Encode { 
            file_path, 
            chunk_type, 
            message, 
            output_file 
        } = &args.command {
            encode(file_path, chunk_type, message, output_file)?
        } else if let Commands::Decode {
            file_path,
            chunk_type
        } = &args.command {
            decode(file_path, chunk_type)?
        } else if let Commands::Remove {
            file_path,
            chunk_type
        } = &args.command {
            remove(&file_path, &chunk_type)?
        }

    Ok(())
}

fn encode(file_path: &PathBuf, chunk_type: &str, message: &str, output_file: &Option<PathBuf>) -> Result<()> {
    
    if file_path.extension().unwrap() != "png" {
        return Err("This program takes only PNG files".into())
    }

    let file = fs::read(file_path)?;

    let mut png = Png::try_from(file.as_slice())?;

    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    png.append_chunk(chunk);

    match output_file {

        Some(output_file) => { 
            let _ = fs::write(output_file, png.as_bytes());

            println!("New file has been created and message encoded successfully!");
        },
        None => {
            let _ = fs::write(file_path, png.as_bytes());

            println!("Message encoded successfully!");
        }

    }

    Ok(())
}

fn decode(file_path: &PathBuf, chunk_type: &str) -> Result<()> {

    if file_path.extension().unwrap() != "png" {
        return Err("This program takes only PNG files".into())
    }

    let file = fs::read(file_path)?;

    let png = Png::try_from(file.as_slice())?;

    match png.chunk_by_type(chunk_type) {
        Some(chunk) => {
            println!("Message: {:?}", chunk.data_as_string().unwrap());
        }
        None => println!("No message hidden in this image with this chunk type")
    }

    Ok(())
}

fn remove(file_path: &PathBuf, chunk_type: &str) -> Result<()> {

    if file_path.extension().unwrap() != "png" {
        return Err("This program takes only PNG files".into())
    }

    let file = fs::read(file_path)?;

    let mut png = Png::try_from(file.as_slice())?;

    png.remove_chunk(chunk_type)?;

    let _ = fs::write(file_path, png.as_bytes())?;

    println!("Message has been removed successfully!");

    Ok(())
}