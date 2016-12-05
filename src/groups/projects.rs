//! List a group's projects
//!
//! https://docs.gitlab.com/ce/api/groups.html#list-a-group-s-projects
//!
//! # List a group's projects
//!
//! Get a list of projects in this group.
//!
//! ```text
//! GET /groups/ID/projects
//! ```
//!
//! Parameters:
//!
//! - `archived` (optional) - if passed, limit by archived status
//! - `visibility` (optional) - if passed, limit by visibility `public`, `internal`, `private`
//! - `order_by` (optional) - Return requests ordered by `id`, `name`, `path`, `created_at`,
//!     `updated_at` or `last_activity_at` fields. Default is `created_at`
//! - `sort` (optional) - Return requests sorted in `asc` or `desc` order. Default is `desc`
//! - `search` (optional) - Return list of authorized projects according to a search criteria
//! - `ci_enabled_first` - Return projects ordered by ci_enabled flag. Projects with enabled
//!     GitLab CI go first
//!
//!


use serde_json;
use serde_urlencoded;

use BuildQuery;
use Projects;

use groups::ProjectsListerInternal;


#[derive(Debug, Clone)]
pub struct ProjectsLister<'a> {
    gl: &'a ::GitLab,
    id: i64,
    internal: ProjectsListerInternal,
}


impl<'a> ProjectsLister<'a> {
    pub fn new(gl: &'a ::GitLab, id: i64) -> ProjectsLister {
        ProjectsLister {
            gl: gl,
            id: id,
            internal: ProjectsListerInternal {
                archived: None,
                visibility: None,
                order_by: None,
                sort: None,
                search: None,
                ci_enabled_first: None,
            }
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

    pub fn order_by(&'a mut self, order_by: ::projects::ListingOrderBy) -> &'a mut ProjectsLister {
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

    pub fn ci_enabled_first(&'a mut self, ci_enabled_first: bool) -> &'a mut ProjectsLister {
        self.internal.ci_enabled_first = Some(ci_enabled_first);
        self
    }


    /// Commit the lister: Query GitLab and return a list of projects.
    pub fn list(&self) -> Projects {
        let query = self.build_query();
        debug!("query: {:?}", query);

        let projects: Result<Projects, serde_json::Error> = self.gl.get(&query);

        projects.unwrap()
    }
}


impl<'a> BuildQuery for ProjectsLister<'a> {
    fn build_query(&self) -> String {
        let encoded = serde_urlencoded::to_string(&self.internal).unwrap();

        let mut query = format!("groups/{}/projects", self.id);
        if !encoded.is_empty() {
            query.push_str("?");
            query.push_str(&encoded);
        }

        query
    }
}


#[cfg(test)]
mod tests {
    use BuildQuery;

    const TEST_PROJECT_ID: i64 = 123;


    #[test]
    fn build_query_default_split0() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects", TEST_PROJECT_ID);

        let lister = gl.groups();
        let lister = lister.projects(TEST_PROJECT_ID);
        let query = lister.build_query();
        assert_eq!(query, expected_string);
    }

    #[test]
    fn build_query_default_split1() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects", TEST_PROJECT_ID);

        let lister = gl.groups().projects(TEST_PROJECT_ID);
        let query = lister.build_query();
        assert_eq!(query, expected_string);
    }

    #[test]
    fn build_query_default() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects", TEST_PROJECT_ID);

        let query = gl.groups().projects(TEST_PROJECT_ID).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_archived() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects?archived=true", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID).archived(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?archived=false", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID).archived(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_visibility() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects?visibility=public", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID)
            .visibility(::ListingVisibility::Public)
            .build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?visibility=internal", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID)
            .visibility(::ListingVisibility::Internal)
            .build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?visibility=private", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID)
            .visibility(::ListingVisibility::Private)
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects?order_by=id", TEST_PROJECT_ID);
        let query =
            gl.groups().projects(TEST_PROJECT_ID).order_by(::projects::ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=name", TEST_PROJECT_ID);
        let query =
            gl.groups().projects(TEST_PROJECT_ID).order_by(::projects::ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=path", TEST_PROJECT_ID);
        let query =
            gl.groups().projects(TEST_PROJECT_ID).order_by(::projects::ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=created_at", TEST_PROJECT_ID);
        let query =
            gl.groups().projects(TEST_PROJECT_ID).order_by(::projects::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=updated_at", TEST_PROJECT_ID);
        let query =
            gl.groups().projects(TEST_PROJECT_ID).order_by(::projects::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=last_activity_at",
                                      TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID)
            .order_by(::projects::ListingOrderBy::LastActivityAt)
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects?sort=asc", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID).sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?sort=desc", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID).sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_search() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects?search=SearchPattern", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID)
            .search(String::from("SearchPattern"))
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_ci_enabled_first() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects?ci_enabled_first=true", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID).ci_enabled_first(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?ci_enabled_first=false", TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID).ci_enabled_first(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_multiple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/projects?archived=true&ci_enabled_first=true",
                                      TEST_PROJECT_ID);
        let query = gl.groups().projects(TEST_PROJECT_ID)
            .archived(true)
            .ci_enabled_first(true)
            .build_query();
        assert_eq!(query, expected_string);
    }
}
