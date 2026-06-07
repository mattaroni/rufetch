use std::{error, pin::Pin};

use futures_util::StreamExt;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

type Error = Box<dyn error::Error>;
type Output = Pin<Box<dyn AsyncWrite>>;

pub async fn linear_download(url: String, mut output: Output) -> Result<(), Error> {
    let content = reqwest::get(url).await?.bytes().await?;

    output.write_all(&content).await?;
    output.flush().await?;
    Ok(())
}

pub async fn streamed_download(url: String, output: Output) -> Result<(), Error> {
    let mut buffer = BufWriter::new(output);
    let mut stream = reqwest::get(url).await?.bytes_stream();

    while let Some(item) = stream.next().await {
        buffer.write_all(&item?).await?;
    }

    buffer.flush().await?;
    Ok(())
}
