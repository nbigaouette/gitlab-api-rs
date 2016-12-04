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


use serde_json;
// use serde_urlencoded;

use BuildQuery;

use merge_requests::MergeRequest;


#[derive(Debug, Clone)]
pub struct MergeRequestLister<'a> {
    gl: &'a ::GitLab,
    id: i64,
    mr_id: i64,
}


#[allow(dead_code)]
impl<'a> MergeRequestLister<'a> {
    pub fn new(gl: &'a ::GitLab, id: i64, mr_id: i64) -> MergeRequestLister {
        MergeRequestLister {
            gl: gl,
            id: id,
            mr_id: mr_id,
        }
    }

    /// Commit the lister: Query GitLab and return a list of merge requests.
    pub fn list(&self) -> MergeRequest {
        let query = self.build_query();
        debug!("query: {:?}", query);

        let merge_request: Result<MergeRequest, serde_json::Error> = self.gl.get(&query);

        merge_request.unwrap()
    }
}


impl<'a> BuildQuery for MergeRequestLister<'a> {
    fn build_query(&self) -> String {
        format!("projects/{}/merge_requests/{}", self.id, self.mr_id)
    }
}


#[cfg(test)]
mod tests {
    use BuildQuery;

    const TEST_PROJECT_ID: i64 = 123;
    const TEST_MR_ID: i64 = 456;

    #[test]
    fn build_query_default() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/{}/merge_requests/{}", TEST_PROJECT_ID, TEST_MR_ID);

        let lister = gl.merge_requests(TEST_PROJECT_ID);
        let lister = lister.single(TEST_MR_ID);
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let lister = gl.merge_requests(TEST_PROJECT_ID).single(TEST_MR_ID);
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let query = gl.merge_requests(TEST_PROJECT_ID).single(TEST_MR_ID).build_query();
        assert_eq!(query, expected_string);
    }
}
