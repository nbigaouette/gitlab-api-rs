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

use serde_json;

use gitlab::GitLab;
use Issue;


impl GitLab {
    pub fn issue(&self, listing: Listing) -> Result<Issue, serde_json::Error> {
        let query = listing.build_query();
        self.get(&query)
    }
}


#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// The ID of a project
    id: i64,
    /// The ID of a project's issue
    issue_id: i64,
}


#[allow(dead_code)]
impl Listing {
    pub fn new(id: i64, issue_id: i64) -> Listing {
        Listing {id: id, issue_id: issue_id}
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {
        format!("projects/{}/issues/{}", self.id, self.issue_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;


    #[test]
    fn build_query_default() {
        let expected_string = "projects/123/issues/456";
        let listing = Listing {id: 123, issue_id: 456};
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }
}
