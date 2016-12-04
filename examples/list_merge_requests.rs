extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;


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

    let gl = GitLab::new_insecure(&hostname, &token);

    let project_id = 142;
    let merge_requests_ids = vec![423, 409];

    let merge_requests = gl.merge_requests(project_id).list();
    println!("merge_requests: {:?}", merge_requests);

    let merge_request = gl.merge_requests(project_id).single(merge_requests_ids[0]).list();
    println!("merge_request: {:?}", merge_request);

    let merge_requests = gl.merge_requests(project_id).iid(vec![merge_requests_ids[0]]).list();
    println!("merge_requests: {:?}", merge_requests);

    let merge_requests = gl.merge_requests(project_id).iid(merge_requests_ids).list();
    println!("merge_requests: {:?}", merge_requests);
}
