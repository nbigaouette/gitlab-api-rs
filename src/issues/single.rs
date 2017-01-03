//! Single issue
//!
//! https://docs.gitlab.com/ce/api/issues.html#single-issue
//!
//! # Single issue
//!
//! Get a single project issue.
//!
//! ```text
//! GET /projects/ID/issues/ISSUE_ID
//! ```
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id`      | integer | yes   | The ID of a project |
//! | `issue_id`| integer | yes   | The ID of a project's issue |
//!
//!


use BuildQuery;
use Issue;

use ::errors::*;


#[derive(Debug, Clone)]
pub struct IssueLister<'a> {
    gl: &'a ::GitLab,
    /// The ID of a project
    id: i64,
    /// The ID of a project's issue
    issue_id: i64,
}


impl<'a> IssueLister<'a> {
    pub fn new(gl: &'a ::GitLab, id: i64, issue_id: i64) -> IssueLister {
        IssueLister {
            gl: gl,
            id: id,
            issue_id: issue_id,
        }
    }

    /// Commit the lister: Query GitLab and return a list of projects.
    pub fn list(&self) -> Result<Issue> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, None, None).chain_err(|| format!("cannot get query {}", query))
    }
}


impl<'a> BuildQuery for IssueLister<'a> {
    fn build_query(&self) -> String {
        format!("projects/{}/issues/{}", self.id, self.issue_id)
    }
}


#[cfg(test)]
mod tests {
    use BuildQuery;

    const TEST_PROJECT_ID: i64 = 123;
    const TEST_ISSUE_ID: i64 = 456;


    #[test]
    fn build_query_default() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/{}/issues/{}", TEST_PROJECT_ID, TEST_ISSUE_ID);

        let lister = gl.issues();
        let lister = lister.single(TEST_PROJECT_ID, TEST_ISSUE_ID);
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let lister = gl.issues().single(TEST_PROJECT_ID, TEST_ISSUE_ID);
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let query = gl.issues().single(TEST_PROJECT_ID, TEST_ISSUE_ID).build_query();
        assert_eq!(query, expected_string);
    }
}
