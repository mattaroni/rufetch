use std::{error::Error, pin::Pin};

use clap::Parser;
use futures_util::StreamExt;
use tokio::{
    fs::File,
    io::{self, AsyncWrite, AsyncWriteExt, BufWriter},
};

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
        println!("error: {e}");
    }
}

/// Read the contents of a remote file and write it to either stdout or a file.
async fn download(url: String, output: Option<String>) -> Result<(), Box<dyn Error>> {
    let output: Pin<Box<dyn AsyncWrite>> = match output {
        Some(path) => Box::pin(File::create(path).await?),
        None => Box::pin(io::stdout()),
    };

    let mut buffer = BufWriter::new(output);
    let mut stream = reqwest::get(url).await?.bytes_stream();

    while let Some(item) = stream.next().await {
        buffer.write_all(&item?).await?;
    }

    buffer.flush().await?;
    Ok(())
}
