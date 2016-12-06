//! Get project by id.
//!
//! https://docs.gitlab.com/ce/api/projects.html#get-single-project
//!
//! # Get single project
//!
//! Get a specific project, identified by project ID or `NAMESPACE/PROJECT_NAME`, which is owned by
//! the authenticated user.
//! If using namespaced projects call make sure that the `NAMESPACE/PROJECT_NAME` is URL-encoded,
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
//! | `id` | integer/string | yes | The ID or `NAMESPACE/PROJECT_NAME` of the project |


use BuildQuery;
use Project;
use projects::ListingId;

use ::errors::*;


#[derive(Debug, Clone)]
pub struct ProjectsLister<'a> {
    gl: &'a ::GitLab,
    id: ListingId,
}


impl<'a> ProjectsLister<'a> {
    pub fn new(gl: &'a ::GitLab, id: ListingId) -> ProjectsLister {
        ProjectsLister { gl: gl, id: id }
    }

    /// Commit the lister: Query GitLab and return a list of projects.
    pub fn list(&self) -> Result<Project> {
        // let query = serde_urlencoded::to_string(&self);
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query).chain_err(|| format!("cannot get query {}", query))
    }
}

impl<'a> BuildQuery for ProjectsLister<'a> {
    fn build_query(&self) -> String {
        let mut query = String::from("projects/");

        query.push_str(&match self.id {
            ListingId::Id(id) => id.to_string(),
            ListingId::NamespaceProject(ref s) => s.replace("/", "%2F"),
        });

        query
    }
}


#[cfg(test)]
mod tests {
    use projects::ListingId;
    use BuildQuery;

    const TEST_PROJECT_ID: i64 = 123;
    const TEST_PROJECT_NAME: &'static str = "group/project";


    #[test]
    fn build_query_id() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/{}", TEST_PROJECT_ID);
        let query = gl.projects()
            .id(ListingId::Id(TEST_PROJECT_ID))
            .build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/{}",
                                      TEST_PROJECT_NAME.to_string().replace("/", "%2F"));
        let query = gl.projects()
            .id(ListingId::NamespaceProject(TEST_PROJECT_NAME.to_string()))
            .build_query();
        assert_eq!(query, expected_string);
    }
}
