//! List merge requests
//!
//! https://docs.gitlab.com/ce/api/merge_requests.html#list-merge-requests
//!
//! # List merge requests
//!
//! Get all merge requests for this project.
//! The `state` parameter can be used to get only merge requests with a given state (`opened`, `closed`, or `merged`) or all of them (`all`).
//! The pagination parameters `page` and `per_page` can be used to restrict the list of merge requests.
//!
//! ```text
//! GET /projects/ID/merge_requests
//! GET /projects/ID/merge_requests?state=opened
//! GET /projects/ID/merge_requests?state=all
//! GET /projects/ID/merge_requests?iid=42
//! GET /projects/ID/merge_requests?iid[]=42&iid[]=43
//! ```
//!
//! Parameters:
//!
//! - `id` (required) - The ID of a project
//! - `iid` (optional) - Return the request having the given `iid`
//! - `state` (optional) - Return `all` requests or just those that are `merged`, `opened` or `closed`
//! - `order_by` (optional) - Return requests ordered by `created_at` or `updated_at` fields. Default is `created_at`
//! - `sort` (optional) - Return requests sorted in `asc` or `desc` order. Default is `desc`
//!
//!


use serde_json;
// use serde_urlencoded;

use BuildQuery;

pub mod single;


#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/merge_requests/serde_types.rs"));

#[derive(Debug, Clone)]
pub struct MergeRequestsLister<'a> {
    gl: &'a ::GitLab,
    id: i64,
    internal: MergeRequestsListerInternal,
}


#[allow(dead_code)]
impl<'a> MergeRequestsLister<'a> {
    pub fn new(gl: &'a ::GitLab, id: i64) -> MergeRequestsLister {
        MergeRequestsLister {
            gl: gl,
            id: id,
            internal: MergeRequestsListerInternal {
                iid: None,
                state: None,
                order_by: None,
                sort: None,
            },
        }
    }


    pub fn single(self, merge_request_id: i64) -> single::MergeRequestLister<'a> {
        // assert_eq!(self, MergeRequestLister::new(self.gl));
        single::MergeRequestLister::new(self.gl, self.id, merge_request_id)
    }


    pub fn iid(&'a mut self, iid: Vec<i64>) -> &'a mut MergeRequestsLister {
        self.internal.iid = Some(iid);
        self
    }
    pub fn state(&'a mut self, state: State) -> &'a mut MergeRequestsLister {
        self.internal.state = Some(state);
        self
    }
    pub fn order_by(&'a mut self, order_by: ListingOrderBy) -> &'a mut MergeRequestsLister {
        self.internal.order_by = Some(order_by);
        self
    }
    fn sort(&'a mut self, sort: ::ListingSort) -> &'a mut MergeRequestsLister {
        self.internal.sort = Some(sort);
        self
    }


    /// Commit the lister: Query GitLab and return a list of projects.
    pub fn list(&self) -> MergeRequests {
        let query = self.build_query();
        debug!("query: {:?}", query);

        let merge_requests: Result<MergeRequests, serde_json::Error> = self.gl.get(&query);

        merge_requests.unwrap()
    }
}


impl<'a> BuildQuery for MergeRequestsLister<'a> {
    fn build_query(&self) -> String {

        // NOTE: Can't use `serde_urlencoded` since it cannot serialize a Vec<T>
        //       See https://github.com/nox/serde_urlencoded/issues/6
        // let encoded = serde_urlencoded::to_string(&self.internal).unwrap();

        let mut query = format!("projects/{}/merge_requests", self.id);

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?" only if at least one of the `Option` is `Some(_)` or if
        // strings contain something.
        query.push_str(match (&self.internal.iid, &self.internal.state, &self.internal.order_by, &self.internal.sort) {
            (&None, &None, &None, &None) => "",
            _ => "?",
        });

        self.internal.iid.as_ref().map(|iid| {
            query.push_str(split_char);
            split_char = &amp_char;

            if iid.len() == 1 {
                query.push_str("iid=");
                query.push_str(&iid[0].to_string());
            } else {
                let mut array_split_char = &none_char;
                for iid in iid {
                    query.push_str(array_split_char);
                    query.push_str("iid[]=");
                    query.push_str(&iid.to_string());
                    array_split_char = &amp_char;
                }
            }
        });

        self.internal.state.map(|state| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("state=");
            query.push_str(match state {
                State::Merged => "merged",
                State::Opened => "opened",
                State::Closed => "closed",
                State::All => "all",
            });
        });

        self.internal.order_by.map(|order_by| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            query.push_str(match order_by {
                ListingOrderBy::CreatedAt => "created_at",
                ListingOrderBy::UpdatedAt => "updated_at",
            });
        });

        self.internal.sort.map(|sort| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("sort=");
            query.push_str(match sort {
                ::ListingSort::Asc => "asc",
                ::ListingSort::Desc => "desc",
            });
        });

        query
    }
}


#[cfg(test)]
mod tests {
    use BuildQuery;


    const TEST_PROJECT_ID: i64 = 123;


    #[test]
    fn build_query_default() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/{}/merge_requests", TEST_PROJECT_ID);
        let lister = gl.merge_requests(TEST_PROJECT_ID);
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/{}/merge_requests", TEST_PROJECT_ID);
        let query = gl.merge_requests(TEST_PROJECT_ID).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_iid() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/{}/merge_requests?iid=456", TEST_PROJECT_ID);
        let query = gl.merge_requests(TEST_PROJECT_ID).iid(vec![456]).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/123/merge_requests?iid[]=456&iid[]=789");
        let query = gl.merge_requests(TEST_PROJECT_ID).iid(vec![456, 789]).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_state() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/{}/merge_requests?state=opened", TEST_PROJECT_ID);
        let query = gl.merge_requests(TEST_PROJECT_ID).state(::merge_requests::State::Opened).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/{}/merge_requests?state=closed", TEST_PROJECT_ID);
        let query = gl.merge_requests(TEST_PROJECT_ID).state(::merge_requests::State::Closed).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/{}/merge_requests?order_by=created_at", TEST_PROJECT_ID);
        let query = gl.merge_requests(TEST_PROJECT_ID).order_by(::merge_requests::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/{}/merge_requests?order_by=updated_at", TEST_PROJECT_ID);
        let query = gl.merge_requests(TEST_PROJECT_ID).order_by(::merge_requests::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/{}/merge_requests?sort=asc", TEST_PROJECT_ID);
        let query = gl.merge_requests(TEST_PROJECT_ID).sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/{}/merge_requests?sort=desc", TEST_PROJECT_ID);
        let query = gl.merge_requests(TEST_PROJECT_ID).sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_multiple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("projects/{}/merge_requests?iid[]=456&iid[]=789&order_by=created_at&sort=asc", TEST_PROJECT_ID);
        let query = gl.merge_requests(TEST_PROJECT_ID)
            .iid(vec![456, 789])
            .sort(::ListingSort::Asc)
            .order_by(::merge_requests::ListingOrderBy::CreatedAt)
            .build_query();
        assert_eq!(query, expected_string);
    }
}
