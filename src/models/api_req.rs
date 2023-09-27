
use std::env;

// api 请求参数
pub struct GithubApiReq {
    pub token: String,
    pub owner: String,
    pub repo: String,
}

impl GithubApiReq {
    pub fn new() -> Self {
        GithubApiReq {
            token: get_env_var("API_GITHUB_TOKEN"),
            owner: get_env_var("OWNER"),
            repo: get_env_var("REPO"),
        }
    }
}

// 获取环境变量
fn get_env_var(var_name: &str) -> String {
    match env::var_os(var_name) {
        Some(val) => val.into_string().unwrap(),
        None => {
            panic!("{} is not defined in the environment.", var_name)
        }
    }
}
