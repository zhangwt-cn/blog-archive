

// api 请求参数
pub struct GithubApiReq {
    pub token: String,
    pub owner: String,
    pub repo: String,
}

impl GithubApiReq {
    pub fn new(token: String, owner: String, repo: String ) -> Self {
        GithubApiReq {
            token,
            owner,
            repo,
        }
    }
}

