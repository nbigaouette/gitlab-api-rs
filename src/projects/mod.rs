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


use serde_urlencoded;

use BuildQuery;
use Lister;


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

use ::errors::*;



#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/projects/serde_types.rs"));


#[derive(Debug, Clone)]
pub struct ProjectsLister<'a> {
    gl: &'a ::GitLab,
    internal: ::projects::ProjectListerInternal,
}


impl<'a> Lister<::projects::Projects> for ProjectsLister<'a> {
    /// Commit the lister: Query GitLab and return a list of projects.
    fn list(&self) -> Result<::projects::Projects> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, None, None).chain_err(|| format!("cannot get query {}", query))
    }

    /// Commit the lister: Query GitLab and return a list of issues.
    fn list_paginated(&self, page: u16, per_page: u16) -> Result<::projects::Projects> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, page, per_page).chain_err(|| format!("cannot get query {}", query))
    }
}


impl<'a> ProjectsLister<'a> {
    pub fn new(gl: &'a ::GitLab) -> ProjectsLister {
        ProjectsLister {
            gl: gl,
            internal: ::projects::ProjectListerInternal {
                archived: None,
                visibility: None,
                order_by: None,
                sort: None,
                search: None,
                simple: None,
            },
        }
    }

    pub fn all(self) -> all::ProjectsLister<'a> {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        all::ProjectsLister::new(self.gl)
    }

    pub fn owned(self) -> owned::ProjectsLister<'a> {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        owned::ProjectsLister::new(self.gl)
    }

    pub fn search(self, query: String) -> search::ProjectsLister<'a> {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        search::ProjectsLister::new(self.gl, query)
    }

    pub fn id(self, id: ListingId) -> id::ProjectsLister<'a> {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        id::ProjectsLister::new(self.gl, id)
    }



    pub fn archived(&'a mut self, archived: bool) -> &'a mut ProjectsLister {
        self.internal.archived = Some(archived);
        self
    }

    pub fn visibility(&'a mut self, visibility: ::ListingVisibility) -> &'a mut ProjectsLister {
        self.internal.visibility = Some(visibility);
        self
    }

    pub fn order_by(&'a mut self, order_by: ListingOrderBy) -> &'a mut ProjectsLister {
        self.internal.order_by = Some(order_by);
        self
    }

    pub fn sort(&'a mut self, sort: ::ListingSort) -> &'a mut ProjectsLister {
        self.internal.sort = Some(sort);
        self
    }

    pub fn search_pattern(&'a mut self, search: String) -> &'a mut ProjectsLister {
        self.internal.search = Some(search);
        self
    }

    pub fn simple(&'a mut self, simple: bool) -> &'a mut ProjectsLister {
        self.internal.simple = Some(simple);
        self
    }
}

impl<'a> BuildQuery for ProjectsLister<'a> {
    fn build_query(&self) -> String {

        let encoded = serde_urlencoded::to_string(&self.internal).unwrap();
        let mut query = String::from("projects");
        if !encoded.is_empty() {
            query.push_str("?");
            query.push_str(&encoded);
        }

        query
    }
}


impl<'a> Project {
    /// Return a lister for the project's issues
    pub fn issues(&'a self, gl: &'a ::GitLab) -> ::issues::project::IssuesLister {
        ::issues::project::IssuesLister::new(gl, self.id)
    }

    /// Return a lister for the project's merge requests
    pub fn merge_requests(&'a self, gl: &'a ::GitLab) -> ::merge_requests::MergeRequestsLister {
        ::merge_requests::MergeRequestsLister::new(gl, self.id)
    }
}



#[cfg(test)]
mod tests {
    use BuildQuery;


    #[test]
    fn build_query_default() {
        let expected_string = "projects";

        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let projects_lister = gl.projects();
        let query = projects_lister.build_query();
        assert_eq!(query, expected_string);

        let query = gl.projects().build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_archived() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
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
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
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
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?order_by=id";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=name";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=path";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=created_at";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=updated_at";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=last_activity_at";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::LastActivityAt)
            .build_query();
        assert_eq!(query, expected_string);
        let query =
            gl.projects().order_by(::projects::ListingOrderBy::LastActivityAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
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
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?search=SearchPattern";
        let mut projects_lister = gl.projects();
        let query = projects_lister.search_pattern(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().search_pattern(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_simple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
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
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?archived=true&simple=true";
        let mut projects_lister = gl.projects();
        let query = projects_lister.archived(true).simple(true).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().archived(true).simple(true).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn project_to_issues() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        let project_id = 123;
        let project = ::projects::Project { id: project_id, ..Default::default() };
        let issues_lister = format!("{:?}", project.issues(&gl));
        let default_issues_lister = format!("{:?}",
                                            ::issues::project::IssuesLister::new(&gl, project_id));
        assert_eq!(issues_lister, default_issues_lister);
    }


    #[test]
    fn project_to_merge_requests() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        let project_id = 123;
        let project = ::projects::Project { id: project_id, ..Default::default() };
        let merge_requests_lister = format!("{:?}", project.merge_requests(&gl));
        let default_merge_requests_lister = format!("{:?}",
                    ::merge_requests::MergeRequestsLister::new(&gl, project_id));
        assert_eq!(merge_requests_lister, default_merge_requests_lister);
    }
}
