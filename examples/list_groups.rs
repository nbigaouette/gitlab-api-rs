extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;


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

    let mut gl = gitlab::GitLab::new(&hostname, &token);
    gl.set_pagination(gitlab::Pagination {
        page: 1,
        per_page: 100,
    });
    println!("gl: {:?}", gl);

    let groups = gl.groups().list();
    println!("groups: {:?}", groups);

    let groups = gl.groups().details(gitlab::groups::ListingId::Id(21)).list();
    println!("groups: {:?}", groups);

    // let groups = gl.groups(gitlab::groups::Listing::new().skip_groups(vec![1, 2, 3]).clone());
    // println!("groups: {:?}", groups);
    //
    let owned_groups = gl.groups().owned().list();
    println!("owned_groups: {:?}", owned_groups);
}
