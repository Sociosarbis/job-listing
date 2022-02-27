use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    env::current_dir,
    fs::{read, write},
    path::PathBuf,
};

static API_URL: &str = "https://gate.lagou.com/v1/entry/positionsearch/searchPosition/v2";
static USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) \
                           AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Mobile \
                           Safari/537.36 Edg/98.0.1108.62";
static ORIGIN: &str = "https://m.lagou.com";
static DEVICE_TYPE: &str = "{deviceType:1}";

#[derive(Parser)]
struct Cli {
    /// config file path
    #[clap(parse(from_os_str))]
    config: PathBuf,

    /// json output path
    #[clap(parse(from_os_str))]
    output: PathBuf,
}

#[derive(Serialize, Deserialize, Clone)]
struct Payload {
    city: Option<String>,
    keyword: Option<String>,
    #[serde(rename = "salaryLower", default)]
    salary_lower: i32,
    #[serde(rename = "salaryUpper", default = "Payload::default_salary_upper")]
    salary_upper: i32,
    #[serde(default)]
    sort: i32,
    #[serde(default = "Payload::default_page_size", rename = "pageSize")]
    page_size: i32,
    #[serde(default = "Payload::default_pages", skip_serializing)]
    pages: i32,
}

impl Payload {
    fn default_pages() -> i32 {
        10
    }

    fn default_page_size() -> i32 {
        15
    }

    fn default_salary_upper() -> i32 {
        1000000
    }
}

#[derive(Serialize, Deserialize)]
struct Job {
    district: String,
    #[serde(rename = "businessZone")]
    business_zone: Option<String>,
    #[serde(rename = "companyId")]
    company_id: i32,
    #[serde(rename = "companyName")]
    company_name: String,
    #[serde(rename = "companySize")]
    company_size: String,
    #[serde(rename = "industryField")]
    industry_field: Option<String>,
    #[serde(rename = "positionName")]
    position_name: String,
    salary: String,
    #[serde(rename = "salaryMonth")]
    salary_month: i32,
    #[serde(rename = "workYear")]
    work_year: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config_file_path = current_dir().unwrap().join(cli.config);
    let payload: Payload =
        serde_json::from_str(&String::from_utf8(read(config_file_path).unwrap()).unwrap()).unwrap();
    let client = Client::builder().user_agent(USER_AGENT).build().unwrap();
    let mut ret = vec![];
    let pages = payload.pages;
    let progress = ProgressBar::new(pages as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .progress_chars("=> "),
    );
    for i in 0..pages {
        let mut payload = serde_json::to_value(payload.clone()).unwrap();
        payload["pageNo"] = json!(i as i32 + 1);
        progress.set_message(format!("fetching page {}", i + 1));
        let res = client
            .post(API_URL)
            .header("origin", ORIGIN)
            .header("x-l-req-header", DEVICE_TYPE)
            .header("referer", format!("{}/", ORIGIN))
            .json(&payload)
            .send()
            .await;
        if let Ok(data) = res {
            let data: Value = data.json().await.unwrap();
            if let Value::Array(arr) = &data["content"]["positionCardVos"] {
                if arr.is_empty() {
                    break;
                }
                for item in arr {
                    if let Value::Bool(is_hunter) = item["isHunter"] {
                        if !is_hunter {
                            ret.push(serde_json::from_value::<Job>(item.clone()).unwrap());
                        }
                    }
                }
            }
        }
        progress.inc(1);
    }
    write(
        current_dir().unwrap().join(cli.output),
        serde_json::to_string_pretty(&ret).unwrap(),
    )
    .expect("output failed");
    progress.finish_with_message("has fetched successfully!");
}
