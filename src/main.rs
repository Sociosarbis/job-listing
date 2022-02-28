use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use job_listing_core::{fetch_job_list, Payload, StateType};
use std::{
    env::current_dir,
    fs::{read, write},
    path::PathBuf,
};

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
    write(
        current_dir().unwrap().join(cli.output),
        serde_json::to_string_pretty(&ret).unwrap(),
    )
    .expect("output failed");
    progress.finish_with_message("has fetched successfully!");
}
