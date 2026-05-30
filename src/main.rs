use clap::Parser;

mod downloaders;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// rename the downloaded file
    #[clap(short, long)]
    filename: Option<String>,

    /// use a streamed downloader, instead of a linear downloader
    #[clap(short, long)]
    stream: bool,

    url: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let filename = args.filename.unwrap_or_else(|| get_filename(&args.url));

    let result = if args.stream {
        downloaders::streamed_download(args.url, filename).await
    } else {
        downloaders::linear_download(args.url, filename).await
    };

    match result {
        Ok(x) => println!("{x}"),
        Err(e) => eprintln!("ERROR: {e}"),
    }
}

fn get_filename(url: &str) -> String {
    let binding = url.to_string();
    let (_, filename) = binding.rsplit_once('/').unwrap_or(("", url));
    return filename.to_string();
}
