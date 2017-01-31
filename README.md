# gitlab-api-rs

Rust wrapper to the GitLab API.

[![Crates.io](https://img.shields.io/crates/v/gitlab-api.svg)](https://crates.io/crates/gitlab-api)
[![Build Status](https://travis-ci.org/nbigaouette/gitlab-api-rs.svg?branch=master)](https://travis-ci.org/nbigaouette/gitlab-api-rs)
[![Documentation](https://docs.rs/gitlab-api/badge.svg)](https://docs.rs/gitlab-api)
[![License](https://img.shields.io/crates/l/gitlab-api.svg)](#licensing)
[![Coverage Status](https://coveralls.io/repos/github/nbigaouette/gitlab-api-rs/badge.svg?branch=master)](https://coveralls.io/github/nbigaouette/gitlab-api-rs?branch=master)
[![Codecov](https://img.shields.io/codecov/c/github/nbigaouette/gitlab-api-rs/master.svg?style=flat)](https://codecov.io/github/nbigaouette/gitlab-api-rs?branch=master)




## Synopsis

[GitLab](https://about.gitlab.com/) is an amazing tool. For most of the tasks, the web UI is more than enough but for some tasks nothing beats scripting them. The [GitLab API](https://docs.gitlab.com/ce/api/) is there to allow scripting actions on the GitLab server.

The excellent [python-gitlab](https://github.com/gpocentek/python-gitlab) allows to use the API from Python, but when playing with it I find myself missing [Rust](https://www.rust-lang.org/)'s static typing. Hence this implementation in Rust.

The (v3) API is quite long, so the parts I need will be implemented first.


## What Works

* Read-only listing:
    * Groups;
    * Issues;
    * Merge Requests;
    * Projects (admin all, user's, specific id, owned, search);


## What Doesn't Work

* Any _write_ commands (`POST`, `PUT`, etc.)
* Any _Enterprise Edition_-specific features.
* API elements using arrays.

    For example [listing merge requests, filtering  with multiple `iid`s](https://docs.gitlab.com/ce/api/merge_requests.html#list-merge-requests):

    ```
    GET /projects/:id/merge_requests?iid[]=42&iid[]=43
    ```
* Some projects listing:
    * branch;
    * branches;
    * events;
    * hook;
    * hooks;
    * starred;
    * visible;


## Usage


```
[dependencies]
gitlab-api = "0.4.0"
```

This crate uses a builder pattern to add filters to a query. Once the query is built, `list()` will commit it by contacting the GitLab server and performing the request.

```
extern crate gitlab_api as gitlab;

fn main() {
    let gl = gitlab::GitLab::new(&"gitlab.com", &"GITLAB_TOKEN_XXXXXXX").unwrap();

    // Get GitLab's version.
    let gitlab_version = gl.version().unwrap();
    println!("gitlab_version: {:?}", gitlab_version);


    // Low level methods

    // Get projects, owned by authenticated user and which are archived.
    let projects = gl.projects().owned().archived(true).list().unwrap();
    println!("projects: {:?}", projects);

    // Get groups owned by authenticated user.
    let owned_groups = gl.groups().owned().list().unwrap();
    println!("owned_groups: {:?}", owned_groups);

    // Get closed issues.
    let closed_issues = gl.issues().state(gitlab::issues::State::Closed).list().unwrap();
    println!("closed_issues: {:?}", closed_issues);


    // Higher level methods

    // Get a specific project
    let project = gl.get_project("nbigaouette1", "gitlab-api-rs").chain_err(|| "cannot get project")?;

    // Get a specific issue
    let issue = gl.get_issue("nbigaouette1", "gitlab-api-rs", 1).chain_err(|| "cannot get issue")?;

    // Get a specific merge request
    let merge_request = gl.get_merge_request("nbigaouette1", "gitlab-api-rs", 1).chain_err(|| "cannot get merge_request")?;
}
```

**NOTES**:
* Crate uses `https` by default. Use `GitLab::new_insecure()` to use `http` (or `port()` and `sheme()` setters on `GitLab` struct).
* Sending your token in clear over `http` is dangerous!
* See [examples/list_projects.rs] for an example of how to load the token (and the hostname) from an environment variable.
* See the `examples` directory for many more examples on how to use this crate.


## Dependencies

Thanks `cargo-graph` for the graph!

![Dependencies](gitlab-api-rs.png)


## Licensing

gitlab-api-rs is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
