
extern crate gitlab_api as gitlab;

use std::env;
use gitlab::GitLab;
use gitlab::Pagination;

fn main() {
    let token = match env::var("GITLAB_TOKEN") {
        Ok(val) => val,
        Err(_)  => panic!("Please set environment variable 'GITLAB_TOKEN'"),
    };

    let mut gl = GitLab::new_https("gitlab.com", &token);
    gl.set_pagination(Pagination{page: 1, per_page: 100});
    println!("gl: {:?}", gl);

    let groups = gl.groups().unwrap();
    println!("groups: {:?}", groups);
}
