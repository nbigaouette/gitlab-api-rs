
extern crate gitlab_api as gitlab;

use std::env;


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
    gl.set_pagination(gitlab::Pagination {
        page: 1,
        per_page: 100,
    });
    println!("gl: {:?}", gl);

    // let groups = gl.groups(Default::default());
    let groups = gl.groups(gitlab::groups::Listing::new().skip_groups(vec![1,2,3]).clone());
    println!("groups: {:?}", groups);

    let owned_groups = gl.owned_groups();
    println!("owned_groups: {:?}", owned_groups);
}
