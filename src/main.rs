mod api;

use clap::Parser;
use eyre::Result;
use http::Uri;
use std::{fs, path::PathBuf};
use tracing::info;
use ureq::{AgentBuilder, Proxy};

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt().compact().init();

    let args = Args::parse();
    let mut builder = AgentBuilder::new();
    builder = builder.user_agent(concat!(
        env!("CARGO_PKG_NAME"),
        '/',
        env!("CARGO_PKG_VERSION")
    ));
    if let Some(proxy) = args.proxy {
        builder = builder.proxy(Proxy::new(proxy.to_string())?);
    }
    let client = api::Client::from(builder.build());

    let outdir = if let Some(pb) = &args.output {
        pb.clone()
    } else {
        std::env::current_dir()?
    };
    fs::create_dir_all(&outdir)?;

    for id in &args.ids {
        let gallery = client.g(*id)?;
        let mut outdir = outdir.clone();

        if let Some(tt) = gallery
            .title
            .english
            .as_deref()
            .or(gallery.title.japanese.as_deref())
            .or(gallery.title.pretty.as_deref())
        {
            info!("downloading {tt}!");
            outdir.push(tt.replace(['/', '\\', '|', '<', '>', ':', '"', '?', '*'], " "));
        } else {
            info!("downloading {id}!");
            outdir.push(id.to_string());
        }

        fs::create_dir_all(&outdir)?;

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

            let img = client.i(&url)?;
            fs::write(path, img)?;
        }
    }

    Ok(())
}

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    ids: Vec<u32>,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, long)]
    proxy: Option<Uri>,
}
