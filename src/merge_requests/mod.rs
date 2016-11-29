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


use BuildQuery;

use serde_json;
use serde_urlencoded;

use gitlab::GitLab;
use MergeRequests;

// Types from serde_types.in.rs
use MergeRequestState;

pub mod single;


// Include serializable types
#[cfg(feature = "serde_derive")]
include!("merge_requests/serde_types.in.rs");
#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/merge_requests/serde_types.rs"));


impl GitLab {
    pub fn merge_requests(&self, listing: Listing) -> Result<MergeRequests, serde_json::Error> {
        let query = listing.build_query();
        self.get(&query)
    }
}



#[allow(dead_code)]
impl Listing {
    pub fn new(id: i64) -> Listing {
        Listing {id: id, ..Default::default()}
    }
    pub fn iid(&mut self, iid: Vec<i64>) -> &mut Listing {
        self.iid = iid;
        self
    }
    pub fn state(&mut self, state: MergeRequestState) -> &mut Listing {
        self.state = Some(state);
        self
    }
    pub fn order_by(&mut self, order_by: ListingOrderBy) -> &mut Listing {
        self.order_by = Some(order_by);
        self
    }
    fn sort(&mut self, sort: ::ListingSort) -> &mut Listing {
        self.sort = Some(sort);
        self
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {

        let mut query = format!("projects/{}/merge_requests", self.id);

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?" only if at least one of the `Option` is `Some(_)` or if
        // strings contain something.
        query.push_str(match (self.iid.is_empty(),
                              &self.state,
                              &self.order_by,
                              &self.sort) {
            (true, &None, &None, &None) => "",
            _ => "?",
        });

        if !self.iid.is_empty() {
            query.push_str(split_char);
            split_char = &amp_char;

            if self.iid.len() == 1 {
                query.push_str("iid=");
                query.push_str(&self.iid[0].to_string());
            } else {
                let mut array_split_char = &none_char;
                for iid in &self.iid {
                    query.push_str(array_split_char);
                    query.push_str("iid[]=");
                    query.push_str(&iid.to_string());
                    array_split_char = &amp_char;
                }
            }
        }

        self.state.map(|state| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("state=");
            query.push_str(match state {
                MergeRequestState::Merged => "merged",
                MergeRequestState::Opened => "opened",
                MergeRequestState::Closed => "closed",
                MergeRequestState::All => "all",
            });
        });

        self.order_by.map(|order_by| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            query.push_str(match order_by {
                ListingOrderBy::CreatedAt => "created_at",
                ListingOrderBy::UpdatedAt => "updated_at",
            });
        });

        self.sort.map(|sort| {
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
    use super::*;
    use BuildQuery;
    use MergeRequestState;


    #[test]
    fn build_query_default() {
        let expected_string = "projects/123/merge_requests";
        let listing = Listing {id: 123, ..Default::default()};
        let query = listing.build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/123/merge_requests";
        let listing = Listing::new(123);
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_iid() {
        let expected_string = format!("projects/123/merge_requests?iid=456");
        let query = Listing::new(123).iid(vec![456]).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/123/merge_requests?iid[]=456&iid[]=789");
        let query = Listing::new(123).iid(vec![456,789]).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_state() {
        let expected_string = "projects/123/merge_requests?state=opened";
        let query = Listing::new(123).state(MergeRequestState::Opened).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/123/merge_requests?state=closed";
        let query = Listing::new(123).state(MergeRequestState::Closed).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let expected_string = "projects/123/merge_requests?order_by=created_at";
        let query = Listing::new(123).order_by(ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/123/merge_requests?order_by=updated_at";
        let query = Listing::new(123).order_by(ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let expected_string = "projects/123/merge_requests?sort=asc";
        let query = Listing::new(123).sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/123/merge_requests?sort=desc";
        let query = Listing::new(123).sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_multiple() {
        let expected_string = "projects/123/merge_requests?iid[]=456&iid[]=789&order_by=created_at&sort=asc";
        let query = Listing::new(123).iid(vec![456,789]).sort(::ListingSort::Asc).order_by(ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);
    }
}
