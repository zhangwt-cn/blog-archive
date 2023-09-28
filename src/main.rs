mod models;
mod service;

use models::api_req::GithubApiReq;
use service::blog::*;

fn main() {
   let args: Vec<String> = std::env::args().collect();
   let mut req = GithubApiReq::new(args[1].clone(), args[2].clone(), args[3].clone());
   sync_blog(&mut req);
}
