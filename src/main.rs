#![deny(clippy::pedantic)]

mod logger;
mod pixiv;
mod types;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;
use pixiv::Pixiv;

/// A command-line application to download images from pixiv.
#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct CLI {
    /// Download the images from the given pixiv illust id
    #[clap(short, long)]
    pub illust: Option<String>,

    /// Download the images from the given pixiv user id
    #[clap(short, long)]
    pub user: Option<String>,

    /// Cookie for pixiv login
    #[clap(short, long)]
    pub cookie: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLI::parse();

    let client = Pixiv::new(cli.cookie)?;

    if let Some(illust) = cli.illust {
        client.download_image(&illust, None).await?;

        return Ok(());
    }

    if let Some(user) = cli.user {
        client.download_user(&user).await?;

        return Ok(());
    }

    CLI::parse_from(&[&std::env::args().nth(0).context("what")?, "--help"]);

    Ok(())
}
