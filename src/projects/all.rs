//! List ALL projects
//!
//! https://docs.gitlab.com/ce/api/projects.html#list-all-projects
//!
//! # List ALL projects
//!
//! Get a list of all projects (admin only).
//!
//! ```text
//! GET /projects/all
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



use serde_json;
use serde_urlencoded;

use BuildQuery;
use Projects;
use projects::{AllProjectListerInternal, ListingOrderBy};


#[derive(Debug, Clone)]
pub struct ProjectsLister<'a> {
    gl: &'a ::GitLab,
    internal: AllProjectListerInternal,
}

impl<'a> ProjectsLister<'a> {
    pub fn new(gl: &'a ::GitLab) -> ProjectsLister {
        ProjectsLister {
            gl: gl,
            internal: AllProjectListerInternal {
                archived: None,
                visibility: None,
                order_by: None,
                sort: None,
                search: None,
            },
        }
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
        let mut query = String::from("projects/all");
        if !encoded.is_empty() {
            query.push_str("?");
            query.push_str(&encoded);
        }
        debug!("query: {}", query);

        query
    }
}


#[cfg(test)]
mod tests {
    use BuildQuery;
    use projects::ListingOrderBy;


    #[test]
    fn build_query_default() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/all";
        let projects_lister = gl.projects().all();
        let query = projects_lister.build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all";
        let query = gl.projects().all().build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_archived() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/all?archived=true";
        let query = gl.projects().all().archived(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?archived=false";
        let query = gl.projects().all().archived(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_visibility() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/all?visibility=public";
        let query = gl.projects().all().visibility(::ListingVisibility::Public).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?visibility=internal";
        let query = gl.projects().all().visibility(::ListingVisibility::Internal).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?visibility=private";
        let query = gl.projects().all().visibility(::ListingVisibility::Private).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/all?order_by=id";
        let query = gl.projects().all().order_by(ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=name";
        let query = gl.projects().all().order_by(ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=path";
        let query = gl.projects().all().order_by(ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=created_at";
        let query = gl.projects().all().order_by(ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=updated_at";
        let query = gl.projects().all().order_by(ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=last_activity_at";
        let query = gl.projects().all().order_by(ListingOrderBy::LastActivityAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/all?sort=asc";
        let query = gl.projects().all().sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?sort=desc";
        let query = gl.projects().all().sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_search() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/all?search=SearchPattern";
        let query = gl.projects().all().search(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_multiple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects/all?archived=false&sort=asc";
        let query = gl.projects().all().archived(false).sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);
    }
}
