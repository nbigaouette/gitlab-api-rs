extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;
use gitlab::merge_requests;


fn main() {
    env_logger::init().unwrap();
    info!("starting up");

    let hostname = match env::var("GITLAB_HOSTNAME") {
        Ok(val) => val,
        Err(_) => {
            let default = String::from("gitlab.com");
            println!("Please set environment variable 'GITLAB_HOSTNAME'. Using default '{}'.",
                     default);
            default
        }
    };

    let token = match env::var("GITLAB_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            panic!("Please set environment variable 'GITLAB_TOKEN'. Take it from \
                    http://{}/profile/account",
                   hostname);
        }
    };

    let gl = GitLab::new(&hostname, &token);

    let merge_requests = gl.merge_requests(142).list();
    println!("merge_requests: {:?}", merge_requests);

    let merge_requests = gl.merge_requests(142).iid(vec![418]).list();
    println!("merge_requests: {:?}", merge_requests);

    let merge_requests = gl.merge_requests(142).iid(vec![419, 420]).list();
    println!("merge_requests: {:?}", merge_requests);
}
