use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// An image message encoder CLI program for PNG files
#[derive(Debug, Parser)]
#[command(name = "pngme")]
#[command(about = "An image message encoder CLI program for PNG files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encodes the message in the PNG file
    #[command(arg_required_else_help = true)]
    Encode {
        /// Path to the PNG file
        file_path: PathBuf,
        /// The type of the chunk
        chunk_type: String,
        /// The message to encode in the PNG file
        message: String,
        /// The output file
        output_file: Option<PathBuf>
    },

    /// Decodes the message in the PNG file
    #[command(arg_required_else_help = true)]
    Decode {
        /// Path to the PNG file
        file_path: PathBuf,
        /// The type of the chunk
        chunk_type: String,
    },

    /// Removes the message in the PNG file
    #[command(arg_required_else_help = true)]
    Remove {
        /// Path to the PNG file
        file_path: PathBuf,
        /// The type of the chunk
        chunk_type: String,
    },

    /// Prints the message in the PNG file
    #[command(arg_required_else_help = true)]
    Print {
        /// Path to the PNG file
        file_path: PathBuf,
    }
}