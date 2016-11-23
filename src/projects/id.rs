//! Get project by id.
//!
//! https://docs.gitlab.com/ce/api/projects.html#get-single-project
//!
//! # Get single project
//!
//! Get a specific project, identified by project ID or NAMESPACE/PROJECT_NAME, which is owned by
//! the authenticated user.
//! If using namespaced projects call make sure that the NAMESPACE/PROJECT_NAME is URL-encoded,
//! eg. `/api/v3/projects/diaspora%2Fdiaspora` (where `/` is represented by `%2F`).
//!
//! ```text
//! GET /projects/ID
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id` | integer/string | yes | The ID or NAMESPACE/PROJECT_NAME of the project |


use BuildQuery;

use serde_json;

use gitlab::GitLab;
use Projects;


impl GitLab {
    pub fn projects_id(&self, listing: Listing) -> Result<Projects, serde_json::Error> {
        let query = listing.build_query();
        self.get(&query)
    }
}


#[derive(Debug, Clone)]
pub enum ListingId {
    Id(i64),
    NamespaceProject(String),
}


#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// The ID or NAMESPACE/PROJECT_NAME of the project
    id: Option<ListingId>,
}


impl Listing {
    pub fn new(id: ListingId) -> Listing {
        Listing { id: Some(id) }
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {

        let mut query = String::from("projects/");

        self.id.clone().map(|id| {
            query.push_str(&match id {
                ListingId::Id(id) => id.to_string(),
                ListingId::NamespaceProject(s) => s.replace("/", "%2F"),
            });
        });

        query
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;

    const TEST_PROJECT_ID: i64 = 123;
    const TEST_PROJECT_NAME: &'static str = "group/project";


    #[test]
    fn build_query_id() {
        let expected_string = format!("projects/{}", TEST_PROJECT_ID);
        let query = Listing::new(ListingId::Id(123)).build_query();
        assert_eq!(query, expected_string);


        let expected_string = format!("projects/{}",
                                      TEST_PROJECT_NAME.to_string().replace("/", "%2F"));
        let query = Listing::new(ListingId::NamespaceProject(TEST_PROJECT_NAME.to_string()))
            .build_query();
        assert_eq!(query, expected_string);
    }
}
