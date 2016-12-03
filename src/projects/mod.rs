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


use serde_json;
use serde_urlencoded;

use BuildQuery;
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



#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/projects/serde_types.rs"));


#[derive(Debug, Clone)]
pub struct ProjectsLister<'a> {
    gl: &'a ::GitLab,
    internal: ProjectListerInternal,
}


impl<'a> ProjectsLister<'a> {
    pub fn new(gl: &'a ::GitLab) -> ProjectsLister {
        ProjectsLister {
            gl: gl,
            internal: ProjectListerInternal {
                archived: None,
                visibility: None,
                order_by: None,
                sort: None,
                search: None,
                simple: None,
            },
        }
    }

    pub fn owned(&self) -> owned::ProjectsLister {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        owned::ProjectsLister::new(self.gl)
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

    pub fn search(&'a mut self, search: String) -> &'a mut ProjectsLister {
        self.internal.search = Some(search);
        self
    }

    pub fn simple(&'a mut self, simple: bool) -> &'a mut ProjectsLister {
        self.internal.simple = Some(simple);
        self
    }

    /// Commit the lister: Query GitLab and return a list of projects.
    pub fn list(&self) -> Projects {
        // let query = serde_urlencoded::to_string(&self);
        let query = self.build_query();
        debug!("query: {:?}", query);

        let projects: Result<Projects, serde_json::Error> = self.gl.get(&query);

        projects.unwrap()
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
