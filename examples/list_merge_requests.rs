/*
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

    let merge_request = gl.merge_request(merge_requests::single::Listing::new(142, 418)).unwrap();
    println!("merge_request: {:?}", merge_request);

    let merge_requests = gl.merge_requests(merge_requests::Listing::new(142)).unwrap();
    println!("merge_requests: {:?}", merge_requests);
}
*/

fn main() {
    println!("Merge requests disabled until API stabilizes. See projects instead.");
}
