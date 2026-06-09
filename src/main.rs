use std::{error::Error, pin::Pin};

use clap::Parser;
use futures_util::StreamExt;
use tokio::{
    fs::File,
    io::{self, AsyncWrite, AsyncWriteExt, BufWriter},
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Rename the downloaded file
    #[clap(short, long)]
    output: Option<String>,

    url: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    if let Err(e) = download(args.url, args.output).await {
        println!("error: {e}");
    }
}

pub async fn download(url: String, output: Option<String>) -> Result<(), Box<dyn Error>> {
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
