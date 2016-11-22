// Inspired by http://python-gitlab.readthedocs.io/en/stable/

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

extern crate hyper;

pub mod gitlab;
pub mod groups;
pub mod projects;

pub use gitlab::Pagination;
pub use gitlab::GitLab;
pub use groups::{GroupListing, GroupListerOptionsOrderBy, GroupListerOptionsSort};
// pub use projects::Project;


trait BuildQuery {
    fn build_query(&self) -> String;  // FIXME: Return Result instead.
}



#[cfg(test)]
mod tests {
    use gitlab::GitLab;
    use hyper;

    // #[test]
    // fn unauthorized() {
    //     let gl = GitLab::new("http", "gitlab.com", 80, "XXXXXXXXXXXXX");
    //     println!("gl: {:?}", gl);
    //     assert_eq!(gl.attempt_connection().unwrap().status,
    //                hyper::status::StatusCode::Unauthorized);
    // }
}
