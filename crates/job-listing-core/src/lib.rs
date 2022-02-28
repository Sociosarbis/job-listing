use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

static API_URL: &str = "https://gate.lagou.com/v1/entry/positionsearch/searchPosition/v2";
static USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) \
                           AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Mobile \
                           Safari/537.36 Edg/98.0.1108.62";
static ORIGIN: &str = "https://m.lagou.com";
static DEVICE_TYPE: &str = "{deviceType:1}";

#[derive(Serialize, Deserialize, Clone)]
pub struct Payload {
    pub city: Option<String>,
    pub keyword: Option<String>,
    #[serde(rename = "salaryLower", default)]
    pub salary_lower: i32,
    #[serde(rename = "salaryUpper", default = "Payload::default_salary_upper")]
    pub salary_upper: i32,
    #[serde(default)]
    pub sort: i32,
    #[serde(default = "Payload::default_page_size", rename = "pageSize")]
    pub page_size: i32,
    #[serde(default = "Payload::default_pages", skip_serializing)]
    pub pages: i32,
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
pub struct Job {
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

pub enum StateType {
    OnGoing(i32),
    Finished(i32),
}

pub async fn fetch_job_list<F>(payload: Payload, on_progress: Option<F>) -> Vec<Job>
where
    F: Fn(StateType),
{
    let client = Client::builder().user_agent(USER_AGENT).build().unwrap();
    let mut ret = vec![];
    let pages = payload.pages;
    for i in 0..pages {
        let mut payload = serde_json::to_value(payload.clone()).unwrap();
        payload["pageNo"] = json!(i as i32 + 1);
        if let Some(ref f) = on_progress {
            f(StateType::OnGoing(i));
        }
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
        if let Some(ref f) = on_progress {
            f(StateType::Finished(i));
        }
    }
    ret
}
