
extern crate gitlab_api as gitlab;

use std::env;
use gitlab::GitLab;
use gitlab::Pagination;

fn main() {
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
    let projects = gl.projects(gitlab::projects::Listing::new()).unwrap();
    println!("projects: {:?}", projects);
    // FIXME: Project's members are private
    // for project in projects {
    //     println!("{:?}", project.path_with_namespace);
    // }

    let projects = gl.projects_all(gitlab::projects::all::Listing::new()).unwrap();
    println!("projects: {:?}", projects);

    let projects = gl.projects_id(gitlab::projects::id::Listing::new(gitlab::projects::id::ListingId::Id(10))).unwrap();
    println!("projects: {:?}", projects);
}
