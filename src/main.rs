use std::{error, pin::Pin};
use clap::Parser;
use tokio::{fs::File, io::{self, AsyncWrite}};

mod downloaders;

type Error = Box<dyn error::Error>;
type Output = Pin<Box<dyn AsyncWrite>>;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Rename the downloaded file
    #[clap(short, long)]
    output: Option<String>,

    /// Use a streamed downloader, instead of a linear downloader
    #[clap(short, long)]
    stream: bool,

    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let output = decide_output(args.output).await?;

    if args.stream {
        downloaders::streamed_download(args.url, output).await?;
        return Ok(())
    }

    downloaders::linear_download(args.url, output).await?;
    Ok(())
}

async fn decide_output(filename: Option<String>) -> Result<Output, io::Error> {
    let output: Output = match filename {
        Some(x) => Box::pin(File::create(x).await?),
        None => Box::pin(io::stdout()),
    };

    Ok(output)
}
