mod api;

use clap::Parser;
use eyre::Result;
use std::path::PathBuf;
use tokio::fs;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().compact().init();

    let args = Args::parse();
    let client = api::Client::new()?;

    let gallery = client.g(args.id).await?;
    let output = match args.output {
        Some(pb) => pb,
        None => match gallery.title.english {
            Some(ref tt) => tt.into(),
            None => match gallery.title.japanese {
                Some(ref tt) => tt.into(),
                None => match gallery.title.pretty {
                    Some(ref tt) => tt.into(),
                    None => gallery.id.to_string().into(),
                },
            },
        },
    };

    let output = match fs::create_dir(&output).await {
        Ok(()) => output,
        Err(_) => gallery.id.to_string().into(),
    };
    for url in gallery.page_urls() {
        let url = url?;
        info!("downloading {url}");
        let img = client.i(&url).await?;

        let mut path = output.clone();
        path.push(
            url.into_parts()
                .path_and_query
                .expect("valid path")
                .path()
                .split('/')
                .last()
                .expect("file name"),
        );
        fs::write(path, img).await?;
    }

    Ok(())
}

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    id: u32,

    #[arg(short, long)]
    output: Option<PathBuf>,
}
