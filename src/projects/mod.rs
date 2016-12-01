//! List projects
//!
//! https://docs.gitlab.com/ce/api/projects.html
//!
//! # List projects
//!
//! Get a list of projects for which the authenticated user is a member.
//!
//! ```text
//! GET /projects
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `archived` | boolean | no | Limit by archived status |
//! | `visibility` | string | no | Limit by visibility `public`, `internal`, or `private` |
//! | `order_by` | string | no | Return projects ordered by `id`, `name`, `path`, `created_at`, `updated_at`, or `last_activity_at` fields. Default is `created_at` |
//! | `sort` | string | no | Return projects sorted in `asc` or `desc` order. Default is `desc` |
//! | `search` | string | no | Return list of authorized projects matching the search criteria |
//! | `simple` | boolean | no | Return only the ID, URL, name, and path of each project |
//!


use std::io::Read;  // Trait providing read_to_string()
use BuildQuery;

use hyper;
use serde_json;
use serde_urlencoded;

use gitlab::GitLab;
use Project;
use Projects;


pub mod all;
pub mod id_branches;
pub mod id_branch;
pub mod id_events;
pub mod id_hooks_id;
pub mod id_hooks;
pub mod id;
pub mod owned;
pub mod search;
pub mod starred;
pub mod visible;


// TODO:
//
// Creates a new project owned by the authenticated user.
// POST /projects
//
// Creates a new project owned by the specified user. Available only for admins.
// POST /projects/user/:user_id
//
// Updates an existing project
// PUT /projects/:id
//
// Forks a project into the user namespace of the authenticated user or the one provided.
// POST /projects/fork/:id
//
// Stars a given project. Returns status code `201` and the project on success and `304` if the
// project is already starred.
// POST /projects/:id/star
//
// Unstars a given project. Returns status code `200` and the project on success and `304` if the
// project is not starred.
// DELETE /projects/:id/star
//
// Archives the project if the user is either admin or the project owner of this project. This
// action is idempotent, thus archiving an already archived project will not change the project.
//
// Status code `201` with the project as body is given when successful, in case the user doesn't
// have the proper access rights, code `403` is returned. Status `404` is returned if the project
// doesn't exist, or is hidden to the user.
// POST /projects/:id/archive
//
//
// Unarchives the project if the user is either admin or the project owner of this project. This
// action is idempotent, thus unarchiving an non-archived project will not change the project.
//
// Status code `201` with the project as body is given when successful, in case the user doesn't
// have the proper access rights, code `403` is returned. Status `404` is returned if the project
// doesn't exist, or is hidden to the user.
// POST /projects/:id/unarchive
//
//
// Removes a project including all associated resources (issues, merge requests etc.)
// DELETE /projects/:id
//
//
// Uploads a file to the specified project to be used in an issue or merge request description, or
// a comment.
// POST /projects/:id/uploads
//
//
// Allow to share project with group.
// POST /projects/:id/share
//
//
// Adds a hook to a specified project.
// POST /projects/:id/hooks
//
//
// Edits a hook for a specified project.
// PUT /projects/:id/hooks/:hook_id
//
//
// Removes a hook from a project. This is an idempotent method and can be called multiple times.
// Either the hook is available or not.
// DELETE /projects/:id/hooks/:hook_id
//
//
// Protects a single branch of a project.
// PUT /projects/:id/repository/branches/:branch/protect
//
//
// Unprotects a single branch of a project.
// PUT /projects/:id/repository/branches/:branch/unprotect
//
//
// Create a forked from/to relation between existing projects.
// POST /projects/:id/fork/:forked_from_id
//
//
// Delete an existing forked from relationship
// DELETE /projects/:id/fork
//


// impl GitLab {
//     pub fn projects_list(&self, listing: Listing) -> Result<Projects, serde_json::Error> {
//         let query = listing.build_query();
//         self.get(&query)
//     }
// }


// https://docs.gitlab.com/ce/api/projects.html#list-projects


#[derive(Debug, Copy, Clone)]
pub enum ListingOrderBy {
    Id,
    Name,
    Path,
    CreatedAt,
    UpdatedAt,
    LastActivityAt,
}


custom_derive!{
    #[derive(Debug, Clone, Builder)]
    pub struct ProjectsLister<'a> {
        gl: &'a ::GitLab,
        /// Limit by archived status
        archived: Option<bool>,
        /// Limit by visibility.
        visibility: Option<::ListingVisibility>,
        /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
        order_by: Option<ListingOrderBy>,
        /// Return requests sorted. Default is `::ListingSort::Desc`.
        sort: Option<::ListingSort>,
        /// Return list of authorized projects matching the search criteria.
        search: String,
        /// Return only the ID, URL, name, and path of each project
        simple: Option<bool>,
    }
}

impl<'a> ProjectsLister<'a> {

    pub fn new(gl: &'a ::GitLab) -> ProjectsLister {
        ProjectsLister {
            gl: gl,
            archived: None,
            visibility: None,
            order_by: None,
            sort: None,
            search: "".into(),
            simple: None,
        }
    }

    /// Commit the lister: Query GitLab and return a list of projects.
    pub fn list(&self) -> Projects {
        // let query = serde_urlencoded::to_string(&self);
        let query = self.build_query();
        debug!("query: {:?}", query);

        let client = self.gl.http_proxy().map_or_else(|| {
            hyper::Client::new()
        }, |proxy| {
            let proxy: Vec<&str> = proxy.trim_left_matches("http://").split(':').collect();
            let hostname = proxy[0].to_string();
            let port = proxy[1];

            info!("Using HTTP proxy {} on port {}", hostname, port);

            hyper::Client::with_http_proxy(hostname, port.parse().unwrap())
        });
        debug!("client: {:?}", client);

        let url = self.gl.build_url(&query);
        debug!("url: {:?}", url);

        // FIXME: Properly handle the error. Will require defining our own errors...

        // Close connections after each GET.
        let mut response = client.get(&url)
                            .header(hyper::header::Connection::close())
                            .send()
                            .unwrap();

        debug!("response: {:?}", response);

        info!("response.status: {:?}", response.status);
        debug!("response.headers: {:?}", response.headers);
        debug!("response.url: {:?}", response.url);

        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();
        debug!("body:\n{:?}", body);

        assert_eq!(response.status, hyper::status::StatusCode::Ok);

        serde_json::from_str(body.as_str()).unwrap()
    }
}

impl<'a> BuildQuery for ProjectsLister<'a> {
    fn build_query(&self) -> String {

        let mut query = String::from("projects");

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?" only if at least one of the `Option` is `Some(_)` or if
        // strings contain something.
        query.push_str(match (&self.archived,
                              &self.visibility,
                              &self.order_by,
                              &self.sort,
                              self.search.is_empty(),
                              &self.simple) {
            (&None, &None, &None, &None, true, &None) => "",
            _ => "?",
        });

        self.archived.map(|archived| {
            query.push_str(split_char);
            split_char = &amp_char;

            if archived {
                query.push_str("archived=true")
            } else {
                query.push_str("archived=false")
            }
        });

        self.visibility.map(|visibility| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("visibility=");
            query.push_str(match visibility {
                ::ListingVisibility::Public => "public",
                ::ListingVisibility::Internal => "internal",
                ::ListingVisibility::Private => "private",
            });
        });

        self.order_by.map(|order_by| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            query.push_str(match order_by {
                ListingOrderBy::Id => "id",
                ListingOrderBy::Name => "name",
                ListingOrderBy::Path => "path",
                ListingOrderBy::CreatedAt => "created_at",
                ListingOrderBy::UpdatedAt => "updated_at",
                ListingOrderBy::LastActivityAt => "last_activity_at",
            });
        });

        self.sort.map(|sort| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("sort=");
            query.push_str(match sort {
                ::ListingSort::Asc => "asc",
                ::ListingSort::Desc => "desc",
            });
        });

        if !self.search.is_empty() {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("search=");
            query.push_str(&self.search);
        }

        self.simple.map(|simple| {
            query.push_str(split_char);
            split_char = &amp_char;

            if simple {
                query.push_str("simple=true")
            } else {
                query.push_str("simple=false")
            }
        });

        query
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;


    #[test]
    fn build_query_default() {
        let expected_string = "projects";

        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let projects_lister = gl.projects();
        let query = projects_lister.build_query();
        assert_eq!(query, expected_string);

        let query = gl.projects().build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_archived() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?archived=true";

        let mut projects_lister = gl.projects();
        let query = projects_lister.archived(true).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().archived(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?archived=false";

        let mut projects_lister = gl.projects();
        let query = projects_lister.archived(false).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().archived(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_visibility() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?visibility=public";
        let mut projects_lister = gl.projects();
        let query = projects_lister.visibility(::ListingVisibility::Public).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().visibility(::ListingVisibility::Public).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?visibility=internal";
        let mut projects_lister = gl.projects();
        let query = projects_lister.visibility(::ListingVisibility::Internal).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().visibility(::ListingVisibility::Internal).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?visibility=private";
        let mut projects_lister = gl.projects();
        let query = projects_lister.visibility(::ListingVisibility::Private).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().visibility(::ListingVisibility::Private).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?order_by=id";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=name";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=path";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=created_at";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=updated_at";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=last_activity_at";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(ListingOrderBy::LastActivityAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(ListingOrderBy::LastActivityAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?sort=asc";
        let mut projects_lister = gl.projects();
        let query = projects_lister.sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?sort=desc";
        let mut projects_lister = gl.projects();
        let query = projects_lister.sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_search() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?search=SearchPattern";
        let mut projects_lister = gl.projects();
        let query = projects_lister.search(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().search(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_simple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?simple=true";
        let mut projects_lister = gl.projects();
        let query = projects_lister.simple(true).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().simple(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?simple=false";
        let mut projects_lister = gl.projects();
        let query = projects_lister.simple(false).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().simple(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_multiple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?archived=true&simple=true";
        let mut projects_lister = gl.projects();
        let query = projects_lister.archived(true).simple(true).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().archived(true).simple(true).build_query();
        assert_eq!(query, expected_string);
    }
}
