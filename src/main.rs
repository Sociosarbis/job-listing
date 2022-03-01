use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use job_listing_core::{fetch_job_list, Payload, StateType};
use std::{
    env::current_dir,
    fs::{read, write},
    path::PathBuf, str::FromStr
};

mod render;


use render::html;


enum Format {
    JSON,
    HTML
}

impl FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::JSON),
            "html" => Ok(Format::HTML),
            _ => Err("unknown format".to_string())
        }
    }
}


#[derive(Parser)]
struct Cli {
    /// config file path
    #[clap(parse(from_os_str))]
    config: PathBuf,

    /// json output path
    #[clap(parse(from_os_str))]
    output: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config_file_path = current_dir().unwrap().join(cli.config);
    let payload: Payload =
        serde_json::from_str(&String::from_utf8(read(config_file_path).unwrap()).unwrap()).unwrap();
    let pages = payload.pages;
    let progress = ProgressBar::new(pages as u64);
    
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .progress_chars("=> "),
    );
    let ret = fetch_job_list(
        payload,
        Some(|state| match state {
            StateType::Finished(_) => {
                progress.inc(1);
            }
            StateType::OnGoing(p) => {
                progress.set_message(format!("fetching page {}", p + 1));
            }
        }),
    )
    .await;
    let format = if let Some(ext) = cli.output.extension() {
        Format::from_str(ext.to_str().unwrap_or("")).unwrap_or(Format::JSON)
    } else {
        Format::JSON
    };
    let output = match format {
        Format::HTML => html::render(&ret),
        Format::JSON => serde_json::to_string_pretty(&ret).unwrap(),
    };
    write(
        current_dir().unwrap().join(cli.output),
        output,
    )
    .expect("output failed");
    progress.finish_with_message("has fetched successfully!");
}
