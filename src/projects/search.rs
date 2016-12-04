//! Search for projects by name
//!
//! https://docs.gitlab.com/ce/api/projects.html#search-for-projects-by-name
//!
//! # Search for projects by name
//!
//! Search for projects by name which are accessible to the authenticated user.
//!
//! ```text
//! GET /projects/search/QUERY
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `query` | string | yes | A string contained in the project name |
//! | `order_by` | string | no | Return requests ordered by `id`, `name`, `created_at` or `last_activity_at` fields |
//! | `sort` | string | no | Return requests sorted in `asc` or `desc` order |


use serde_json;
use serde_urlencoded;

use BuildQuery;
use Projects;
use projects::{SearchProjectListerInternal, ListingOrderBy};


#[derive(Debug, Clone)]
pub struct ProjectsLister<'a> {
    gl: &'a ::GitLab,
    /// A string contained in the project name.
    query: String,
    internal: SearchProjectListerInternal,
}

impl<'a> ProjectsLister<'a> {
    pub fn new(gl: &'a ::GitLab, query: String) -> ProjectsLister {
        ProjectsLister {
            gl: gl,
            query: query,
            internal: SearchProjectListerInternal {
                order_by: None,
                sort: None,
            },
        }
    }

    pub fn order_by(&'a mut self, order_by: ListingOrderBy) -> &'a mut ProjectsLister {
        self.internal.order_by = Some(order_by);
        self
    }

    pub fn sort(&'a mut self, sort: ::ListingSort) -> &'a mut ProjectsLister {
        self.internal.sort = Some(sort);
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
        let mut query = format!("projects/search/{}", self.query);
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

    const TEST_SEARCH_QUERY: &'static str = "SearchPattern";


    #[test]
    fn build_query_default() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/search/{}", TEST_SEARCH_QUERY);

        let mut lister = gl.projects().search(TEST_SEARCH_QUERY.to_string());
        let query = lister.build_query();
        assert_eq!(query, expected_string);
    }


//     #[test]
//     fn build_query_order_by() {
//         let expected_string = format!("projects/search/{}?order_by=id", TEST_SEARCH_QUERY);
//         let query =
//             ProjectsLister::new(TEST_SEARCH_QUERY.to_string()).order_by(ListingOrderBy::Id).build_query();
//         assert_eq!(query, expected_string);
//
//         let expected_string = format!("projects/search/{}?order_by=name", TEST_SEARCH_QUERY);
//         let query = ProjectsLister::new(TEST_SEARCH_QUERY.to_string())
//             .order_by(ListingOrderBy::Name)
//             .build_query();
//         assert_eq!(query, expected_string);
//
//         let expected_string = format!("projects/search/{}?order_by=created_at", TEST_SEARCH_QUERY);
//         let query = ProjectsLister::new(TEST_SEARCH_QUERY.to_string())
//             .order_by(ListingOrderBy::CreatedAt)
//             .build_query();
//         assert_eq!(query, expected_string);
//
//         let expected_string = format!("projects/search/{}?order_by=last_activity_at",
//                                       TEST_SEARCH_QUERY);
//         let query = ProjectsLister::new(TEST_SEARCH_QUERY.to_string())
//             .order_by(ListingOrderBy::LastActivityAt)
//             .build_query();
//         assert_eq!(query, expected_string);
//     }
//
//
//     #[test]
//     fn build_query_sort() {
//         let expected_string = format!("projects/search/{}?sort=asc", TEST_SEARCH_QUERY);
//         let query =
//             ProjectsLister::new(TEST_SEARCH_QUERY.to_string()).sort(::ListingSort::Asc).build_query();
//         assert_eq!(query, expected_string);
//
//         let expected_string = format!("projects/search/{}?sort=desc", TEST_SEARCH_QUERY);
//         let query =
//             ProjectsLister::new(TEST_SEARCH_QUERY.to_string()).sort(::ListingSort::Desc).build_query();
//         assert_eq!(query, expected_string);
//     }
//
//
//     #[test]
//     fn groups_build_query_multiple() {
//         let expected_string = format!("projects/search/{}?order_by=created_at&sort=desc",
//                                       TEST_SEARCH_QUERY);
//         let query = ProjectsLister::new(TEST_SEARCH_QUERY.to_string())
//             .order_by(ListingOrderBy::CreatedAt)
//             .sort(::ListingSort::Desc)
//             .build_query();
//         assert_eq!(query, expected_string);
//     }
}
