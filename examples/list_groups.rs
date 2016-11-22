
extern crate gitlab_api as gitlab;

use std::env;
use gitlab::GitLab;
use gitlab::Pagination;
use gitlab::{GroupListerOptions, GroupListerOptionsOrderBy};


fn main() {
    let token = match env::var("GITLAB_TOKEN") {
        Ok(val) => val,
        Err(_) => panic!("Please set environment variable 'GITLAB_TOKEN'"),
    };

    let mut gl = GitLab::new_https("gitlab.com", &token);
    gl.set_pagination(Pagination {
        page: 1,
        per_page: 100,
    });
    println!("gl: {:?}", gl);

    // gl.groups_listing(Default::default());
    gl.groups_listing(GroupListerOptions { order_by: Some(GroupListerOptionsOrderBy::Path), ..Default::default() });
    // let groups = gl.groups().unwrap();
    // println!("groups: {:?}", groups);
}
