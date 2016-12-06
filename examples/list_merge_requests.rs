extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

use gitlab::GitLab;

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

    let gl = GitLab::new(&hostname, &token);
    // let gl = GitLab::new(&hostname, &token).scheme("http").port(80);
    // let gl = gl.scheme("http").port(80);

    let project_id = 142;
    let merge_requests_ids = vec![409, 410];
    let merge_requests_iids = vec![3, 4];

    let merge_requests =
        gl.merge_requests(project_id).list().chain_err(|| "cannot get merge request")?;
    println!("merge_requests: {:?}", merge_requests);

    let merge_request = gl.merge_requests(project_id)
        .single(merge_requests_ids[0])
        .list()
        .chain_err(|| "cannot get merge request")?;
    println!("merge_request: {:?}", merge_request);

    let merge_requests = gl.merge_requests(project_id)
        .iid(vec![merge_requests_iids[0]])
        .list()
        .chain_err(|| "cannot get merge request")?;
    println!("merge_requests: {:?}", merge_requests);

    // let merge_requests = gl.merge_requests(project_id).iid(merge_requests_iids).list().chain_err(|| "cannot get merge request")?;
    // println!("merge_requests: {:?}", merge_requests);

    Ok(())
}
