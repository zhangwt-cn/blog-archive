use crate::models::api_req::GithubApiReq;
use crate::models::api_resp::IssuesResponse;
use hyper::Client;
use hyper::{Body, Error, Method, Request, StatusCode};
use std::fs;
use hyper_tls::HttpsConnector;

// 请求github api
#[tokio::main]
pub async fn req_api(req: GithubApiReq) -> Result<(), Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues?state=all&page=1&per_page=100",
        req.owner, req.repo
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
    let resp = client.request(request).await?;
    match resp.status() {
        StatusCode::OK => {
            let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
            let body_str = String::from_utf8_lossy(&body_bytes);
            println!("Response body:\n{}", body_str);
            let json = body_str.to_string().replace("null", "\"\"");
            // 解析 JSON 响应
            let issues_list: Vec<IssuesResponse> =
                serde_json::from_str(&json).expect("JSON was not well-formatted");
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
    println!("issues count: {}", issues_list.len());
    let mut text = String::new();
    text.push_str("# Summary\n\n");
    for issue in issues_list {
        text.push_str(
            format!(
                "- [{}]({}) - {}\n",
                issue.title, issue.html_url, issue.created_at
            )
            .as_str(),
        );
    }
    update_readme(text);
}

// update readme.md
fn update_readme(text: String) {
    // text rewrite to file
    fs::write("README.md", text).expect("Unable to write file");
}
