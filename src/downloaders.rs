use std::{error, pin::Pin};
use futures_util::StreamExt;
use tokio::io::{AsyncWrite, AsyncWriteExt};

type Error = Box<dyn error::Error>;
type Output = Pin<Box<dyn AsyncWrite>>;

pub async fn linear_download(url: String, mut output: Output) -> Result<(), Error> {
    let content = reqwest::get(url).await?.bytes().await?;

    output.write_all(&content).await?;
    output.flush().await?;
    Ok(())
}

pub async fn streamed_download(url: String, mut output: Output) -> Result<(), Error> {
    let mut stream = reqwest::get(url).await?.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        output.write_all(&chunk).await?;
    }

    output.flush().await?;
    Ok(())
}
