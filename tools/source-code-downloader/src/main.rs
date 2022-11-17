use std::{
    collections::BTreeSet,
    fs::{create_dir_all, write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Parser)]
struct Args {
    /// AtCoder contest id
    #[clap(short, long)]
    contest_id: String,

    /// directory to save the code
    #[clap(short, long)]
    out: PathBuf,

    /// number of source code to download for each problem
    #[clap(short, long, default_value = "50")]
    limit: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let client = Client::builder().gzip(true).build()?;

    let problem_ids = list_problems(&args.contest_id, &client).await?;

    for problem_id in problem_ids {
        let mut downloaded = 0;
        for page in 1.. {
            log::info!("scraping page={} for {}", page, problem_id);
            let ids = list_submissions(&args.contest_id, &problem_id, page, &client).await?;
            if ids.is_empty() {
                break;
            }

            for submission_id in ids {
                let (code, problem_id) =
                    download_submission(&args.contest_id, submission_id, &client).await?;
                let path = args.out.join(&args.contest_id).join(&problem_id);
                create_dir_all(&path)?;
                let path = path.join(format!("{}_{}.py", problem_id, submission_id));

                write(path, code)?;
                downloaded += 1;
                if downloaded >= args.limit {
                    break;
                }
            }

            if downloaded >= args.limit {
                break;
            }
        }
    }

    Ok(())
}

async fn download_submission(
    contest_id: &str,
    submission_id: i64,
    client: &Client,
) -> Result<(String, String)> {
    let url = format!(
        "https://atcoder.jp/contests/{}/submissions/{}",
        contest_id, submission_id
    );
    let body = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("pre").expect("invalid selector");

    let code = document
        .select(&selector)
        .flat_map(|e| e.text())
        .collect::<String>();

    let selector = Selector::parse("a").expect("invalid selector");
    let problem_id = document
        .select(&selector)
        .filter_map(|e| e.value().attr("href"))
        .filter_map(|h| h.strip_prefix(&format!("/contests/{}/tasks/", contest_id)))
        .next()
        .context("no problem id")?
        .to_string();
    Ok((code, problem_id))
}

async fn list_submissions(
    contest_id: &str,
    problem_id: &str,
    page: usize,
    client: &Client,
) -> Result<Vec<i64>> {
    let url = format!(
        r"https://atcoder.jp/contests/{}/submissions?f.LanguageName=Python3&f.Status=AC&orderBy=created&page={}&f.Task={}",
        contest_id, page, problem_id
    );
    let mut submission_ids = vec![];
    let body = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").expect("invalid selector");
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Some(a) = href.strip_prefix(&format!("/contests/{}/submissions/", contest_id)) {
                let id = a.parse::<i64>()?;
                submission_ids.push(id);
            }
        }
    }
    Ok(submission_ids)
}

async fn list_problems(contest_id: &str, client: &Client) -> Result<BTreeSet<String>> {
    let url = format!(r"https://atcoder.jp/contests/{}/tasks", contest_id);
    let body = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").expect("invalid selector");
    let mut problem_ids = BTreeSet::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Some(problem_id) = href.strip_prefix(&format!("/contests/{}/tasks/", contest_id))
            {
                problem_ids.insert(problem_id.to_string());
            }
        }
    }
    Ok(problem_ids)
}
