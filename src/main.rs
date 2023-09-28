mod models;
mod service;

use models::api_req::GithubApiReq;
use service::blog::req_api;

fn main() {
   let args: Vec<String> = std::env::args().collect();
   let req = GithubApiReq::new(args[1].clone(), args[2].clone(), args[3].clone());
   match req_api(req) {
      Ok(_) => println!("sync blog success!"),
      Err(e) => println!("error: {}", e),
   }
}
