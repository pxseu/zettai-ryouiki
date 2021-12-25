use structopt::StructOpt;

mod logger;
mod pixiv;
mod utils;

/// A command-line application to download images from pixiv.
#[derive(StructOpt)]
struct Cli {
    /// Download the images from the given pixiv illust id
    #[structopt(short, long)]
    pub illust: Option<String>,

    /// Download the images from the given pixiv user id
    #[structopt(short, long)]
    pub user: Option<String>,

    /// Cookie for pixiv login
    #[structopt(short, long)]
    pub cookie: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Cli::from_args();

    let client = reqwest::Client::new();

    if !args.illust.is_none() {
        pixiv::download_image(
            client.clone(),
            args.illust.unwrap(),
            None,
            args.cookie.clone(),
        )
        .await
        .unwrap();

        return Ok(());
    }

    if !args.user.is_none() {
        pixiv::download_user(client, args.user.unwrap(), args.cookie)
            .await
            .unwrap();

        return Ok(());
    }

    Cli::clap().print_help().unwrap();

    Ok(())
}
