// Inspired by http://python-gitlab.readthedocs.io/en/stable/

extern crate hyper;
extern crate rustc_serialize;

pub mod gitlab;
pub mod projects;

pub use gitlab::GitLab;
pub use projects::{Project,Projects};




#[cfg(test)]
mod tests {
    use std::env;
    use gitlab::GitLab;
    use hyper;

    #[test]
    fn it_works() {

        let token = match env::var("GITLAB_TOKEN") {
            Ok(val) => val,
            Err(_)  => panic!("Please set environment variable 'GITLAB_TOKEN'"),
        };

        let gl = GitLab::new("http", "gitlab.com", 80, "XXXXXXXXXXXXX");
        println!("gl: {:?}", gl);
        assert_eq!(gl.attempt_connection().unwrap().status, hyper::status::StatusCode::Unauthorized);

        // let gl = GitLab::new("http", "gitlab.com", 80, &token);
        // println!("gl: {:?}", gl);
        // assert_eq!(gl.attempt_connection().unwrap().status, hyper::Ok);
        //
        // let gl = GitLab::new_http("gitlab.com", &token);
        // println!("gl: {:?}", gl);
        // assert_eq!(gl.attempt_connection().unwrap().status, hyper::Ok);

        let gl = GitLab::new_https("gitlab.com", &token);
        println!("gl: {:?}", gl);
        assert_eq!(gl.attempt_connection().unwrap().status, hyper::Ok);

        // Example from GitLab: https://docs.gitlab.com/ce/api/#basic-usage
        let expected_url = "https://gitlab.example.com:443/api/v3/projects?private_token=XXXXXXXXXXXXX";
        let gl = GitLab::new_https("gitlab.example.com", "XXXXXXXXXXXXX");
        assert_eq!(gl.build_url("projects"), expected_url);
    }
}
