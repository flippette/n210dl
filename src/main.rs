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

    let outdir = if let Some(pb) = &args.output {
        pb.clone()
    } else {
        std::env::current_dir()?
    };
    fs::create_dir_all(&outdir).await?;

    for id in &args.ids {
        let gallery = client.g(*id).await?;
        let mut outdir = outdir.clone();

        if let Some(tt) = gallery
            .title
            .english
            .as_deref()
            .or(gallery.title.japanese.as_deref())
            .or(gallery.title.pretty.as_deref())
        {
            info!("downloading {tt}!");
            outdir.push(
                tt.replace(['/', '\\', '|', '<', '>', ':', '"', '?', '*'], " "),
            );
        } else {
            info!("downloading {id}!");
            outdir.push(id.to_string());
        }

        fs::create_dir_all(&outdir).await?;

        for (i, url) in gallery.page_urls().enumerate() {
            let url = url?;
            info!("{id}: {} / {}", i + 1, gallery.num_pages);
            let mut path = outdir.clone();
            path.push(
                url.path_and_query()
                    .expect("valid path")
                    .path()
                    .split('/')
                    .last()
                    .expect("file name"),
            );

            let img = client.i(&url).await?;
            fs::write(path, img).await?;
        }
    }

    Ok(())
}

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ids: Vec<u32>,

    #[arg(short, long)]
    output: Option<PathBuf>,
}
