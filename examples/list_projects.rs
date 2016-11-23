
extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;
use gitlab::Pagination;
use gitlab::projects;


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

    let mut gl = GitLab::new_https(&hostname, &token);
    // for i in 1..82 {
    //     gl.set_pagination(Pagination{page: i, per_page: 1});
    //     println!("projects: {:?}", gl.projects().unwrap());
    // }
    // gl.set_pagination(Pagination {
    //     page: 1,
    //     per_page: 100,
    // });
    let projects = gl.projects(projects::Listing::new()).unwrap();
    println!("projects: {:?}", projects);
    // FIXME: Project's members are private
    // for project in projects {
    //     println!("{:?}", project.path_with_namespace);
    // }

    let projects = gl.projects_all(projects::all::Listing::new()).unwrap();
    println!("projects: {:?}", projects);

    let listing = projects::id::Listing::new(projects::id::ListingId::Id(10));
    let projects = gl.projects_id(listing).unwrap();
    println!("projects: {:?}", projects);
}
