extern crate gitlab_api as gitlab;

use std::env;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate clap;


use gitlab::GitLab;
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

    let matches = clap::App::new("get_id_issue")
        .version("1.0")
        .author("Nicolas Bigaouette <nbigaouette@gmail.com>")
        .about("Get the id of a GitLab issue from namespace/project#iid.")
        .arg(clap::Arg::with_name("namespace")
            .help("The project's namespace (or group)")
            .long("namespace")
            .short("n")
            .takes_value(true)
            .required(true))
        .arg(clap::Arg::with_name("project")
            .help("The project's name")
            .long("project")
            .short("p")
            .takes_value(true)
            .required(true))
        .arg(clap::Arg::with_name("id")
            .help("The issue's id")
            .long("id")
            .short("i")
            .takes_value(true)
            .required(true))
        .get_matches();

    let project_namespace = matches.value_of("namespace").unwrap();
    let project_name = matches.value_of("project").unwrap();
    let issue_iid = value_t!(matches, "id", i64).unwrap_or_else(|e| e.exit());

    let issue = gl.get_issue(project_namespace, project_name, issue_iid)
        .chain_err(|| "cannot get issue")?;
    // println!("issue: {:?}", issue);

    println!("Id for {}/{}#{}: {}",
             project_namespace,
             project_name,
             issue_iid,
             issue.id);

    Ok(())
}
