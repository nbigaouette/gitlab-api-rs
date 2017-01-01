#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use ::errors::*;


#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));


#[macro_use]
extern crate log;
extern crate hyper;

extern crate url;


pub mod gitlab;
pub mod groups;
pub mod projects;
pub mod issues;
pub mod merge_requests;

// Re-export those structs
pub use gitlab::GitLab;
// pub use projects::Project;
// Re-export those traits


trait BuildQuery {
    fn build_query(&self) -> String;
}

pub trait Lister<T> {
    fn list(&self) -> Result<T>;
    // fn list_paginated(&self, page: u16, per_page: u16) -> Result<T>;
    // FIXME: Remove default implementation, will have to implement this for other structs.
    //        See src/issues/mod.rs
    fn list_paginated(&self, _page: u16, _per_page: u16) -> Result<T> {
        self.list()
    }
}

pub trait GitLabItem {
    fn iid(&self) -> i64;
}


#[cfg(test)]
mod tests {
    // use gitlab::GitLab;
    // use hyper;

    // #[test]
    // fn unauthorized() {
    //     let gl = GitLab::new("http", "gitlab.com", 80, "XXXXXXXXXXXXX").unwrap();
    //     println!("gl: {:?}", gl);
    //     assert_eq!(gl.attempt_connection().unwrap().status,
    //                hyper::status::StatusCode::Unauthorized);
    // }
}
