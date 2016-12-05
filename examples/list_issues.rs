extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;
// use gitlab::Pagination;
use gitlab::issues;


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
    // let gl = GitLab::new(&hostname, &token).scheme("http").port(80);
    // let gl = gl.scheme("http").port(80);

    let issues = gl.issues().list();
    println!("issues: {:?}", issues);

    let opened_issues = gl.issues().state(issues::State::Opened).list();
    println!("opened_issues: {:?}", opened_issues);

    let closed_issues = gl.issues().state(issues::State::Closed).list();
    println!("closed_issues: {:?}", closed_issues);

    let issue = gl.issues().single(142, 739).list();
    println!("issue: {:?}", issue);

    let group_issues = gl.issues().group(21).state(issues::State::Closed).list();
    println!("group_issues: {:?}", group_issues);

    let project_issues = gl.issues().project(142).state(issues::State::Opened).list();
    println!("project_issues: {:?}", project_issues);
}
