use std::pin::Pin;

use clap::Parser;
use futures_util::StreamExt;
use thiserror::Error;
use tokio::{
    fs::File,
    io::{self, AsyncWrite, AsyncWriteExt, BufWriter},
};

#[derive(Error, Debug)]
enum AsyncError {
    #[error("cannot create file - {0}")]
    CannotCreateFile(io::ErrorKind),

    #[error("failed to read remote file from URL")]
    RequestError(#[from] reqwest::Error),

    #[error("failed to write data - {0}")]
    WritngFailure(io::ErrorKind),
}

/// Command-line arguments passed to the application.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Write to a file instead of stdout
    #[clap(short, long)]
    output: Option<String>,

    /// HTTP(S) address to the online file
    url: String,
}

/// Runs the application.
#[tokio::main]
async fn main() {
    let args = Cli::parse();

    if let Err(e) = download(args.url, args.output).await {
        println!("\x1b[91;1merror:\x1b[0m {e}");
    }
}

/// Reads the contents of a remote file and write it to either stdout or a file.
async fn download(url: String, output: Option<String>) -> Result<(), AsyncError> {
    let to_file_error = |e: io::Error| AsyncError::CannotCreateFile(e.kind());
    let to_writing_error = |e: io::Error| AsyncError::WritngFailure(e.kind());

    let mut stream = reqwest::get(url).await?.bytes_stream();

    let output: Pin<Box<dyn AsyncWrite>> = match output {
        Some(path) => Box::pin(File::create(path).await.map_err(to_file_error)?),
        None => Box::pin(io::stdout()),
    };

    let mut buffer = BufWriter::new(output);

    while let Some(item) = stream.next().await {
        buffer.write_all(&item?).await.map_err(to_writing_error)?;
    }

    buffer.flush().await.map_err(to_writing_error)?;
    Ok(())
}
