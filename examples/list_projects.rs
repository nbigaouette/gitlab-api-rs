
extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;
// use gitlab::Pagination;
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

// // Projects
// // List projects
// let projects = gl.projects().list()
// let projects = gl.projects().archived(...).visibility(...).order_by(...).sort(...).search(...).simple(...).list()
// // List all projects (admin)
// let projects = gl.projects().all().list()
// let projects = gl.projects().all().archived(...).visibility(...).order_by(...).sort(...).search(...).list()
// // Single project
// let project = gl.projects().id(142).list()
// // Search
// let projects = gl.projects().search(...).list()
// // Owned
// let projects = gl.projects().owned().archived(...).visibility(...).order_by(...).sort(...).search(...).list()

    let gl = GitLab::new_insecure(&hostname, &token);

    // let projects = gl.projects().list();
    let projects = gl.projects().archived(false).list();
    println!("projects: {:?}", projects);





    // // for i in 1..82 {
    // //     gl.set_pagination(Pagination{page: i, per_page: 1});
    // //     println!("projects: {:?}", gl.projects_list().unwrap());
    // // }
    // // gl.set_pagination(Pagination {
    // //     page: 1,
    // //     per_page: 100,
    // // });
    // let projects = gl.projects_list(projects::Listing::new()).unwrap();
    // println!("projects: {:?}", projects);
    // // FIXME: Project's members are private
    // // for project in projects {
    // //     println!("{:?}", project.path_with_namespace);
    // // }
    //
    // let projects = gl.projects_all(projects::all::Listing::new()).unwrap();
    // println!("projects: {:?}", projects);
    //
    // let listing = projects::id::Listing::new(projects::id::ListingId::Id(10));
    // let projects = gl.project_id(listing).unwrap();
    // println!("projects: {:?}", projects);
}
