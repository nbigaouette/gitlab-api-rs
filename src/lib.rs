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

#[macro_use]
extern crate log;
extern crate hyper;

extern crate regex;
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


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListingSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListingVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "private")]
    Private,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum UserState {
    #[serde(rename = "active")]
    Active,

    #[serde(rename = "blocked")]
    Blocked,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum MilestoneState {
    #[serde(rename = "active")]
    Active,

    #[serde(rename = "closed")]
    Closed,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub revision: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub description: String,
    pub visibility_level: i64,
    pub lfs_enabled: bool,
    pub avatar_url: Option<String>,
    pub web_url: String,
    pub request_access_enabled: bool,
}

pub type Groups = Vec<Group>;


#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub id: i64,
    pub iid: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String,
    pub state: MilestoneState,
    pub created_at: String,  // FIXME: Use date type?
    pub updated_at: String,  // FIXME: Use date type?
    pub due_date: Option<String>  // FIXME: Use date type?
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub id: i64,
    pub state: UserState,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>
}



trait BuildQuery {
    fn build_query(&self) -> String;
}

pub trait Lister<T> {
    fn list(&self) -> Result<T>;
    fn list_paginated(&self, page: u16, per_page: u16) -> Result<T>;
}


#[cfg(test)]
mod tests {
    // use gitlab::GitLab;
    // use hyper;
    use serde_json;

    // #[test]
    // fn unauthorized() {
    //     let gl = GitLab::new("http", "gitlab.com", 80, "XXXXXXXXXXXXX").unwrap();
    //     println!("gl: {:?}", gl);
    //     assert_eq!(gl.attempt_connection().unwrap().status,
    //                hyper::status::StatusCode::Unauthorized);
    // }

    #[test]
    fn deserialize_project() {
        let json_reply = r##"[
            {
                "id": 517564,
                "description": "GitLab API library and client in Rust",
                "default_branch": "master",
                "tag_list": [],
                "public": false,
                "archived": false,
                "visibility_level": 0,
                "ssh_url_to_repo": "git@gitlab.com:nbigaouette1/gitlab-api-rs.git",
                "http_url_to_repo": "https://gitlab.com/nbigaouette1/gitlab-api-rs.git",
                "web_url": "https://gitlab.com/nbigaouette1/gitlab-api-rs",
                "owner": {
                    "name": "Nicolas Bigaouette",
                    "username": "nbigaouette1",
                    "id": 163821,
                    "state": "active",
                    "avatar_url": "https://secure.gravatar.com/avatar/3325e461df2fda8738f35a8bf4fd735e?s=80&d=identicon",
                    "web_url": "https://gitlab.com/nbigaouette1"
                },
                "name": "gitlab-api-rs",
                "name_with_namespace": "Nicolas Bigaouette / gitlab-api-rs",
                "path": "gitlab-api-rs",
                "path_with_namespace": "nbigaouette1/gitlab-api-rs",
                "container_registry_enabled": null,
                "issues_enabled": true,
                "merge_requests_enabled": true,
                "wiki_enabled": true,
                "builds_enabled": false,
                "snippets_enabled": false,
                "created_at": "2015-10-09T00:32:18.646Z",
                "last_activity_at": "2017-01-31T14:46:26.638Z",
                "shared_runners_enabled": true,
                "lfs_enabled": true,
                "creator_id": 163821,
                "namespace": {
                    "id": 193119,
                    "name": "nbigaouette1",
                    "path": "nbigaouette1",
                    "kind": "user"
                },
                "avatar_url": null,
                "star_count": 0,
                "forks_count": 0,
                "open_issues_count": 1,
                "public_builds": true,
                "shared_with_groups": [],
                "only_allow_merge_if_build_succeeds": false,
                "request_access_enabled": true,
                "only_allow_merge_if_all_discussions_are_resolved": null,
                "approvals_before_merge": 0
            }
        ]"##;

        let _: ::projects::Projects = serde_json::from_str(json_reply)
            .expect("JSON deserialization failed");
    }
}
