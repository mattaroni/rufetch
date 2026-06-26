use clap::Parser;

mod downloader;

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

    if let Err(e) = downloader::download(args.url, args.output).await {
        println!("\x1b[91;1merror:\x1b[0m {e}");
    }
}
