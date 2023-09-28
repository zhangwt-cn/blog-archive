

// api 请求参数
pub struct GithubApiReq {
    pub token: String,
    pub owner: String,
    pub repo: String,
    pub per_page: i32,
    pub page: i32,
}

impl GithubApiReq {
    pub fn new(token: String, owner: String, repo: String) -> Self {
        GithubApiReq {
            token,
            owner,
            repo,
            per_page: 100,
            page: 1,
        }
    }
}

