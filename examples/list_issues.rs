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

    let issues = gl.issues().list();
    println!("issues: {:?}", issues);

    let opened_issues = gl.issues().state(issues::State::Opened).list();
    println!("opened_issues: {:?}", opened_issues);

    let closed_issues = gl.issues().state(issues::State::Closed).list();
    println!("closed_issues: {:?}", closed_issues);

    let issue = gl.issues().single(142, 739).list();
    println!("issue: {:?}", issue);

//     let listing = issues::group::Listing::new(21).state(issues::ListingState::Closed).clone();
//     let group_issues = gl.group_issues(listing).unwrap();
//     println!("group_issues: {:?}", group_issues);
//
//     let listing = issues::project::Listing::new(142).state(issues::ListingState::Closed).clone();
//     let project_issues = gl.project_issues(listing).unwrap();
//     println!("project_issues: {:?}", project_issues);
}
