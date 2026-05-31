use std::{error::Error, pin::Pin};
use futures_util::StreamExt;
use reqwest::get;
use tokio::{io::{AsyncWrite, AsyncWriteExt}, time::Instant};

type Output = Pin<Box<dyn AsyncWrite>>;
type UnitResult = Result<(), Box<dyn Error>>;

pub async fn linear_download(url: String, mut output: Output) -> UnitResult {
    let start = Instant::now();
    let content = get(url).await?.bytes().await?;

    output.write_all(&content).await?;
    output.flush().await?;

    println!("Downloaded in {:?}", start.elapsed());
    Ok(())
}

pub async fn streamed_download(url: String, mut output: Output) -> UnitResult {
    let start = Instant::now();
    let mut stream = get(url).await?.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        output.write_all(&chunk).await?;
    }

    output.flush().await?;

    println!("Downloaded in {:?}", start.elapsed());
    Ok(())
}
