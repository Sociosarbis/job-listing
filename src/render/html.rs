use job_listing_core::Job;

static JOB_URL: &str = "https://m.lagou.com/wn/jobs";
static COMPANY_URL: &str = "https://m.lagou.com/wn/gongsi";

pub fn render(jobs: &Vec<Job>) -> String {
    let style = r#"<style>
    .container {
        background: #F5F5F5;
    }
    .logo {
        width: 34px;
    }
    .d-inline-block {
        display: inline-block;
    }
    .green {
        color: #00B38A;
    }
    .cr666 {
        color: #666;
    }
    .cr999 {
        color: #999;
    }
    .card {
        background: #fff;
        padding: 15px;
        margin-bottom: 10px;
    }
    .float-right {
        float: right;
    }
  </style>"#;
    let list: String = (0..jobs.len())
        .into_iter()
        .map(|i| {
            let job = &jobs[i];
            format!(
                r#"<li class="card cr999"><a href="{}" target="_blank">{}</a><span class="green float-right">{}{}</span></div>
    <div class="cr666">{}{}{}</div>
    <div><img class="logo" src="{}" alt="" />
      <span class="d-inline-block">
        <div><a href="{}" target="_blank">{}</a></div>
        <div>{} {}</div>
      </span>
    </li>"#,
                format!("{}/{}.html", JOB_URL, job.position_id),
                job.position_name,
                job.salary,
                if job.salary_month != 0 {
                    format!("·{}薪", job.salary_month)
                } else {
                    "".to_string()
                },
                job.district,
                if let Some(v) = &job.business_zone {
                    format!(" {}", v)
                } else {
                    "".to_string()
                },
                format!(" {}", job.work_year),
                job.company_logo,
                format!("{}/{}.html", COMPANY_URL, job.company_id),
                job.company_name,
                job.company_size,
                if let Some(v) = &job.industry_field {
                    v.clone()
                } else {
                    "".to_string()
                }
            )
        })
        .collect();
    format!("{}<ol class=\"container\">{}</ol>", style, list)
}
