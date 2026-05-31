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
    let output = match decide_output(args.output).await {
        Ok(x) => x,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return
        },
    };

    let result = if args.stream {
        downloaders::streamed_download(args.url, output).await
    } else {
        downloaders::linear_download(args.url, output).await
    };

    match result {
        Ok(_) => (),
        Err(e) => eprintln!("ERROR: {e}"),
    }
}

async fn decide_output(output_name: Option<String>) -> Result<Pin<Box<dyn AsyncWrite>>, io::Error> {
    let output: Pin<Box<dyn AsyncWrite>> = match output_name {
        Some(x) => Box::pin(File::create(x).await?),
        None => Box::pin(io::stdout()),
    };

    Ok(output)
}
