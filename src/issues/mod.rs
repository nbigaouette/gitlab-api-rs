//! List issues
//!
//! https://docs.gitlab.com/ce/api/issues.html#list-issues
//!
//! # List issues
//!
//! Get all issues created by the authenticated user.
//!
//! ```text
//! GET /issues
//! ```
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `state`   | string  | no    | Return all issues or just those that are `opened` or `closed`|
//! | `labels`  | string  | no    | Comma-separated list of label names, issues with any of the labels will be returned |
//! | `order_by`| string  | no    | Return requests ordered by `created_at` or `updated_at` fields. Default is `created_at` |
//! | `sort`    | string  | no    | Return requests sorted in `asc` or `desc` order. Default is `desc`  |
//!


use BuildQuery;

use serde_json;

use gitlab::GitLab;
use Issues;

pub mod single;


impl GitLab {
    pub fn issues(&self, listing: Listing) -> Result<Issues, serde_json::Error> {
        let query = listing.build_query();
        self.get(&query)
    }
}


#[derive(Debug, Copy, Clone)]
pub enum ListingState {
    Opened,
    Closed,
}


#[derive(Debug, Copy, Clone)]
pub enum ListingOrderBy {
    CreatedAt,
    UpdatedAt,
}


#[derive(Debug, Copy, Clone)]
pub enum ListingSort {
    Asc,
    Desc,
}



#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// State of issues to return.
    state: Option<ListingState>,
    /// Labels of issues to return.
    labels: Vec<String>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `ListingSort::Desc`.
    sort: Option<ListingSort>,
}


#[allow(dead_code)]
impl Listing {
    pub fn new() -> Listing {
        Default::default()
    }
    pub fn state(&mut self, state: ListingState) -> &mut Listing {
        self.state = Some(state);
        self
    }
    pub fn labels(&mut self, labels: Vec<String>) -> &mut Listing {
        self.labels = labels;
        self
    }
    pub fn order_by(&mut self, order_by: ListingOrderBy) -> &mut Listing {
        self.order_by = Some(order_by);
        self
    }
    fn sort(&mut self, sort: ListingSort) -> &mut Listing {
        self.sort = Some(sort);
        self
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {

        let mut query = String::from("issues");

        let amp_char = "&";
        let comma_char = ",";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?" only if at least one of the `Option` is `Some(_)` or if
        // strings contain something.
        query.push_str(match (&self.state,
                              self.labels.is_empty(),
                              &self.order_by,
                              &self.sort) {
            (&None, true, &None, &None) => "",
            _ => "?",
        });

        self.state.map(|state| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("state=");
            query.push_str(match state {
                ListingState::Opened => "opened",
                ListingState::Closed => "closed",
            });
        });

        if !self.labels.is_empty() {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("labels=");

            let mut array_split_char = &none_char;
            for label in &self.labels {
                query.push_str(array_split_char);
                query.push_str(&label.to_string());
                array_split_char = &comma_char;
            }
        }

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
                ListingSort::Asc => "asc",
                ListingSort::Desc => "desc",
            });
        });

        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;


    #[test]
    fn build_query_default() {
        let expected_string = "issues";
        let listing: Listing = Default::default();
        let query = listing.build_query();
        assert_eq!(query, expected_string);

        let expected_string = "issues";
        let listing = Listing::new();
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_state() {
        let expected_string = "issues?state=opened";
        let query = Listing::new().state(ListingState::Opened).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "issues?state=closed";
        let query = Listing::new().state(ListingState::Closed).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_skip_groups() {
        let expected_string = "issues?labels=label1,label2,label3";
        let query = Listing::new().labels(vec![String::from("label1"), String::from("label2"), String::from("label3")]).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let expected_string = "issues?order_by=created_at";
        let query = Listing::new().order_by(ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "issues?order_by=updated_at";
        let query = Listing::new().order_by(ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let expected_string = "issues?sort=asc";
        let query = Listing::new().sort(ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "issues?sort=desc";
        let query = Listing::new().sort(ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_multiple() {
        let expected_string = "issues?order_by=created_at&sort=asc";
        let query = Listing::new().sort(ListingSort::Asc).order_by(ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);
    }
}
