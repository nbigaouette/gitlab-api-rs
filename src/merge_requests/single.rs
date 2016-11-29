//! Get single MR
//!
//! https://docs.gitlab.com/ce/api/merge_requests.html#get-single-mr
//!
//! # Get single MR
//!
//! Shows information about a single merge request.
//!
//! ```text
//! GET /projects/ID/merge_requests/MERGE_REQUEST_ID
//! ```
//!
//! Parameters:
//!
//! - `id` (required) - The ID of a project
//! - `merge_request_id` (required) - The ID of MR
//!
//!


use BuildQuery;

use serde_json;

use gitlab::GitLab;
use MergeRequest;


impl GitLab {
    pub fn merge_request(&self, listing: Listing) -> Result<MergeRequest, serde_json::Error> {
        let query = listing.build_query();
        self.get(&query)
    }
}


#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// Project id
    id: i64,
    /// Merge request's ID
    mr_id: i64,
}


impl Listing {
    pub fn new(id: i64, mr_id: i64) -> Listing {
        Listing {id: id, mr_id: mr_id}
    }
}

impl BuildQuery for Listing {
    fn build_query(&self) -> String {
        format!("projects/{}/merge_requests/{}", self.id, self.mr_id)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;


    #[test]
    fn build_query_default() {
        let expected_string = "projects/123/merge_requests/456";
        let listing = Listing {id: 123, mr_id: 456};
        let query = listing.build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/123/merge_requests/456";
        let listing = Listing::new(123, 456);
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }
}
