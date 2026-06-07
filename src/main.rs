use std::pin::Pin;
use clap::Parser;
use tokio::{fs::File, io::{self, AsyncWrite}};

mod downloaders;

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
async fn main() {
    let args = Cli::parse();

    let runner = async || {
        let output: Pin<Box<dyn AsyncWrite>> = match args.output {
            Some(x) => Box::pin(File::create(x).await?),
            None => Box::pin(io::stdout()),
        };

        if args.stream {
            return downloaders::streamed_download(args.url, output).await;
        }

        downloaders::linear_download(args.url, output).await
    };

    if let Err(e) = runner().await {
        println!("error: {e}");
    }
}
