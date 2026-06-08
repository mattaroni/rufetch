use std::{error::Error, pin::Pin};

use futures_util::StreamExt;
use tokio::{
    fs::File,
    io::{self, AsyncWrite, AsyncWriteExt, BufWriter},
};

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
