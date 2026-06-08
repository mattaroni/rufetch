use clap::Parser;

mod downloaders;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Rename the downloaded file
    #[clap(short, long)]
    output: Option<String>,

    url: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    if let Err(e) = downloaders::download(args.url, args.output).await {
        println!("error: {e}");
    }
}
