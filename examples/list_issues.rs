extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;
// use gitlab::Pagination;
use gitlab::issues;
use gitlab::Lister;

use gitlab::errors::*;


fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
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

    let gl = GitLab::new(&hostname, &token).chain_err(|| "failure to create GitLab instance")?;
    // let gl = GitLab::new(&hostname, &token)
    //     .chain_err(|| "failure to create GitLab instance")?
    //     .scheme("http").port(80);
    // let gl = gl.scheme("http").port(80);

    let issues = gl.issues().list().chain_err(|| "cannot get issues")?;
    println!("issues: {:?}", issues);

    let opened_issues =
        gl.issues().state(issues::State::Opened).list().chain_err(|| "cannot get issues")?;
    println!("opened_issues: {:?}", opened_issues);

    let closed_issues =
        gl.issues().state(issues::State::Closed).list().chain_err(|| "cannot get issues")?;
    println!("closed_issues: {:?}", closed_issues);

    let issue = gl.issues().single(142, 739).list().chain_err(|| "cannot get issues")?;
    println!("issue: {:?}", issue);

    let group_issues = gl.issues()
        .group(21)
        .state(issues::State::Closed)
        .list()
        .chain_err(|| "cannot get issues")?;
    println!("group_issues: {:?}", group_issues);

    let project_issues = gl.issues()
        .project(142)
        .state(issues::State::Opened)
        .list()
        .chain_err(|| "cannot get issues")?;
    println!("project_issues: {:?}", project_issues);

    Ok(())
}
