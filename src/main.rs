mod models;
mod service;

use models::api_req::GithubApiReq;
use service::blog::req_api;


fn main() {
   let req = GithubApiReq::new();
   match req_api(req) {
       Ok(_) => println!("ok"),
       Err(e) => println!("error: {}", e),
   }
}



