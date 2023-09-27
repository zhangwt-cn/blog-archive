use crate::models::api_req::GithubApiReq;
use crate::models::api_resp::IssuesResponse;
use reqwest::Error;
use reqwest::StatusCode;
use std::process::Command;

// 请求github api
#[tokio::main]
pub async fn req_api(req: GithubApiReq) -> Result<(), Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues?staet=all",
        req.owner, req.repo
    );
    let resp = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", "blog-archive")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {}", req.token))
        .send()
        .await?;

    match resp.status() {
        StatusCode::OK => {
            let issues_list: Vec<IssuesResponse> = resp.json().await?;
            handle_issues(issues_list)
        }
        StatusCode::UNAUTHORIZED => {
            println!("error: {}", resp.status());
        }
        _ => {
            println!("error: {}", resp.status());
        }
    }
    Ok(())
}

// handle issues
fn handle_issues(issues_list: Vec<IssuesResponse>) {
    let mut text = String::new();
    for issue in issues_list {
        text.push_str(
            format!("[{}]({}) - {}\n", issue.title, issue.url, issue.created_at).as_str(),
        );
    }
    update_readme(text);
}

// update readme.md
fn update_readme(text: String) {
    
    // text write to readme.md
    Command::new("echo")
        .arg(text)
        .arg(">")
        .arg("README.md")
        .output()
        .expect("echo failed");
}
