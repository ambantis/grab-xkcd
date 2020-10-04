use anyhow::Result;
use clap::Clap;
use serde::Deserialize;
use std::convert::TryFrom;

/// A utility to grab XKCD comics
#[derive(Clap)]
pub struct Args {
    /// Set a connection timeout
    #[clap(long, short, default_value = "30")]
    pub timeout: u64,
    /// Print output in a format
    #[clap(long, short, arg_enum, default_value = "text")]
    pub output: OutFormat,
    /// The comic to load
    #[clap(long, short, default_value = "0")]
    pub num: usize,
    /// Save image file to current directory
    #[clap(long, short)]
    pub save: bool,
}

#[derive(Clap, Copy, Clone)]
pub enum OutFormat {
    Json,
    Text,
}

const BASE_URL: &str = "https://xkcd.com";
const LATEST_COMIC: usize = 0;

struct XkcdClient {
    args: Args,
}

impl XkcdClient {
    fn new(args: Args) -> Self {
        XkcdClient { args }
    }

    fn run(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct ComicResponse {
    month: String,
    num: usize,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String,
}

impl TryFrom<String> for ComicResponse {
    type Error = anyhow::Error;

    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json).map_err(|e| e.into())
    }
}

struct Comic {
    title: String,
    num: usize,
    date: String,
    desc: String,
    img_url: String,
}

impl From<ComicResponse> for Comic {
    fn from(cr: ComicResponse) -> Self {
        Comic {
            title: cr.title,
            num: cr.num,
            date: format!("{}-{}-{}", cr.day, cr.month, cr.year),
            desc: cr.alt,
            img_url: cr.img,
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let client = XkcdClient::new(args);
    client.run()
}
