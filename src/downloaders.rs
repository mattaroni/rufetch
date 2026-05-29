use std::error::Error;
use futures_util::StreamExt;
use reqwest::get;
use tokio::{fs::File, io::AsyncWriteExt, time::Instant};

type StringResult = Result<String, Box<dyn Error>>;

pub async fn linear_download(url: String, filename: String) -> StringResult {
    let start = Instant::now();
    let mut file = File::create(&filename).await?;
    let content = get(url).await?.bytes().await?;

    file.write_all(&content).await?;
    file.flush().await?;

    Ok(get_download_time(filename, start))
}

pub async fn streamed_download(url: String, filename: String) -> StringResult {
    let start = Instant::now();
    let mut file = File::create(&filename).await?;
    let mut stream = get(url).await?.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
    }

    file.flush().await?;

    Ok(get_download_time(filename, start))
}

fn get_download_time(filename: String, start: Instant) -> String {
    let duration = start.elapsed();
    return format!("Downloaded {filename} in {duration:?}");
}
