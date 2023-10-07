use crate::models::api_req::GithubApiReq;
use crate::models::api_resp::IssuesResponse;
use chrono::prelude::*;
use hyper::Client;
use hyper::{Body, Error, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use std::fs;

// 请求github api
#[tokio::main]
pub async fn req_api(req: &mut GithubApiReq) -> Result<Vec<IssuesResponse>, Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues?state=all&page={}&per_page={}",
        req.owner, req.repo, req.page, req.per_page
    );

    let request = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("User-Agent", "blog-archive")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {}", req.token))
        .body(Body::empty())
        .unwrap();

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    match client.request(request).await {
        Ok(resp) => {
            match resp.status() {
                StatusCode::OK => {
                    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
                    let body_str = String::from_utf8_lossy(&body_bytes);
                    let json = body_str.to_string()
                                        .replace("null", "\"\"")
                                        .replace("{", "`{`")
                                        .replace("}", "`}`");
                    // 解析 JSON 响应
                    match serde_json::from_str(&json) {
                        Ok(issues_list) => {
                            return Ok(issues_list);
                        }
                        Err(e) => {
                            println!("error:{}, json: {}", e, json);
                        }
                    }
                }
                StatusCode::UNAUTHORIZED => {
                    println!("error: {}", resp.status());
                }
                _ => {
                    println!("error: {}", resp.status());
                }
            }
            Ok(vec![])
        }
        Err(e) => panic!("error: {}", e),
    }
}

// handle issues
fn handle_issues(issues_list: &Vec<IssuesResponse>) -> String {
    println!("issues count: {}", issues_list.len());
    let mut text = String::new();
    for issue in issues_list {
        // 将日期字符串解析为 DateTime<Utc> 类型
        let parsed_date_time = DateTime::parse_from_rfc3339(&issue.created_at).unwrap();
        // 格式化日期和时间为字符串
        let formatted_date_time = parsed_date_time.format("%Y-%m-%d %H:%M:%S").to_string();
        text.push_str(
            format!(
                "- [{}]({}) - {}\n",
                issue.title, issue.html_url, formatted_date_time
            )
            .as_str(),
        );
    }
    text
}

// update readme.md
fn update_readme(text: String) {
    // text rewrite to file
    fs::write("output.txt", text).expect("Unable to write file");
}

// sync blog
pub fn sync_blog(req: &mut GithubApiReq) {
    let mut tag = true;
    let mut text = String::new();
    text.push_str("# Summary\n\n");
    while tag {
        let issues_list = req_api(req).expect("api request error");
        if issues_list.len() == 0 {
            break;
        }
        text.push_str(&handle_issues(&issues_list));
        if issues_list.len() < 100 {
            tag = false;
        }
        req.page += 1;
    }
    update_readme(text);
}
