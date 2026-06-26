use clap::Parser;
use futures_util::StreamExt;
use reqwest::Response;
use thiserror::Error;
use tokio::{
    fs::File,
    io::{self, AsyncWriteExt, BufWriter, Stdout},
};

#[derive(Error, Debug)]
enum AsyncError {
    #[error("GET request failed")]
    RequestError(#[from] reqwest::Error),

    #[error("failed to write data; {0}")]
    IoError(#[from] io::Error),
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

struct ProgressTracker {
    total_size: u64,
    current_size: u64,
    printer: BufWriter<Stdout>,
}

impl ProgressTracker {
    fn new(total_size: u64) -> Self {
        let current_size = 0;
        let printer = BufWriter::new(io::stdout());

        Self {
            total_size,
            current_size,
            printer,
        }
    }

    async fn update_and_print(&mut self, size_increase: usize) -> Result<(), AsyncError> {
        self.current_size += size_increase as u64;
        let percent = self.current_size * 100 / self.total_size;
        let message = format!("\rDownload: {percent}% complete");
        self.printer.write_all(message.as_bytes()).await?;
        self.printer.flush().await?;

        Ok(())
    }

    async fn finish(&mut self) -> Result<(), AsyncError> {
        self.printer.write_all(b"\n").await?;
        self.printer.flush().await?;

        Ok(())
    }
}

async fn download(url: String, output: Option<String>) -> Result<(), AsyncError> {
    let response = reqwest::get(url).await?;

    match output {
        Some(path) => download_to_file(response, path).await,
        None => download_to_stdout(response).await,
    }
}

async fn download_to_file(response: Response, path: String) -> Result<(), AsyncError> {
    let mut progress_tracker = get_content_length(&response).map(ProgressTracker::new);

    if progress_tracker.is_none() {
        println!("warning: cannot determine file size");
    }

    let mut stream = response.bytes_stream();

    let file = File::create(path).await?;
    let mut buffer = BufWriter::new(file);

    while let Some(item) = stream.next().await {
        let chunk = item?;
        buffer.write_all(&chunk).await?;

        if let Some(tracker) = progress_tracker.as_mut() {
            tracker.update_and_print(chunk.len()).await?;
        }
    }

    buffer.flush().await?;

    if let Some(tracker) = progress_tracker.as_mut() {
        tracker.finish().await?;
    }

    Ok(())
}

async fn download_to_stdout(response: Response) -> Result<(), AsyncError> {
    let mut stream = response.bytes_stream();
    let mut buffer = BufWriter::new(io::stdout());

    while let Some(item) = stream.next().await {
        buffer.write_all(&item?).await?;
    }

    buffer.flush().await?;
    Ok(())
}

fn get_content_length(response: &Response) -> Option<u64> {
    response
        .headers()
        .get("content-length")?
        .to_str()
        .ok()?
        .parse()
        .ok()
}
