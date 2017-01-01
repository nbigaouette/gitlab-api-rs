extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;

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

    let mut gl =
        gitlab::GitLab::new(&hostname, &token).chain_err(|| "failure to create GitLab instance")?;
    // let mut gl = gitlab::GitLab::new(&hostname, &token).chain_err(|| "failure to create GitLab instance")?.scheme("http").port(80);
    // gl = gl.scheme("http").port(80);

    println!("gl: {:?}", gl);

    let groups = gl.groups().list().chain_err(|| "cannot get groups")?;
    println!("groups: {:?}", groups);

    let groups = gl.groups()
        .details(gitlab::groups::ListingId::Id(21))
        .list()
        .chain_err(|| "cannot get groups")?;
    println!("groups: {:?}", groups);

    // let groups = gl.groups(gitlab::groups::Listing::new().skip_groups(vec![1, 2, 3]).clone());
    // println!("groups: {:?}", groups);
    //
    let owned_groups = gl.groups().owned().list().chain_err(|| "cannot get groups")?;
    println!("owned_groups: {:?}", owned_groups);

    let projects_groups = gl.groups().projects(21).list().chain_err(|| "cannot get groups")?;
    println!("projects_groups: {:?}", projects_groups);

    Ok(())
}
