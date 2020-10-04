use anyhow::Result;
use clap::Clap;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::time::Duration;
use url::Url;

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
    #[clap(long, short)]
    pub num: Option<usize>,
    /// Save image file to current directory
    #[clap(long, short)]
    pub save: bool,
}

#[derive(Clap, Copy, Clone)]
pub enum OutFormat {
    Json,
    Text,
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

#[derive(Serialize)]
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

impl Comic {
    fn save(&self) -> Result<()> {
        let url = Url::parse(&*self.img_url)?;
        let img_name = url.path_segments().unwrap().last().unwrap();
        let p = std::env::current_dir()?;
        let p = p.join(img_name);
        let resp = reqwest::blocking::get(&self.img_url)?;
        std::fs::write(p, &*resp.bytes()?).map_err(|e| e.into())
    }

    fn print(&self, of: OutFormat) -> Result<()> {
        match of {
            OutFormat::Json => println!("{}", self),
            OutFormat::Text => println!("{}", serde_json::to_string(self)?),
        }
        Ok(())
    }
}

impl fmt::Display for Comic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\n\
            Comic No: {}\n\
            Date: {}\n\
            Description: {}\n\
            Image: {}\n",
            self.title, self.num, self.date, self.desc, self.img_url
        )
    }
}

const BASE_URL: &str = "https://xkcd.com";
const INFO_0_JSON: &str = "info.0.json";

struct XkcdClient {
    args: Args,
}

impl XkcdClient {
    fn new(args: Args) -> Self {
        XkcdClient { args }
    }

    fn run(&self) -> Result<()> {
        let url = if let Some(n) = self.args.num {
            format!("{}/{}/{}", BASE_URL, n, INFO_0_JSON)
        } else {
            format!("{}/{}", BASE_URL, INFO_0_JSON)
        };

        let http_client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(self.args.timeout))
            .build()?;
        let response: String = http_client.get(&url).send()?.text()?;
        let comic_response: ComicResponse = ComicResponse::try_from(response)?;
        let comic: Comic = comic_response.into();
        if self.args.save {
            comic.save()?;
        }
        comic.print(self.args.output)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let client = XkcdClient::new(args);
    client.run()
}
