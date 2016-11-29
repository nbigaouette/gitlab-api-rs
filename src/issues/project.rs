//! List project issues
//!
//! https://docs.gitlab.com/ce/api/issues.html#list-project-issues
//!
//! # List project issues
//!
//! Get a list of a project's issues.
//!
//! ```text
//! GET /projects/ID/issues
//! GET /projects/ID/issues?state=opened
//! GET /projects/ID/issues?state=closed
//! GET /projects/ID/issues?labels=foo
//! GET /projects/ID/issues?labels=foo,bar
//! GET /projects/ID/issues?labels=foo,bar&state=opened
//! GET /projects/ID/issues?milestone=1.0.0
//! GET /projects/ID/issues?milestone=1.0.0&state=opened
//! GET /projects/ID/issues?iid=42
//! ```
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id`      | integer | yes   | The ID of a project |
//! | `iid`     | integer | no    | Return the issue having the given `iid` |
//! | `state`   | string  | no    | Return all issues or just those that are `opened` or `closed`|
//! | `labels`  | string  | no    | Comma-separated list of label names, issues with any of the labels will be returned |
//! | `milestone` | string| no    | The milestone title |
//! | `order_by`| string  | no    | Return requests ordered by `created_at` or `updated_at` fields. Default is `created_at` |
//! | `sort`    | string  | no    | Return requests sorted in `asc` or `desc` order. Default is `desc`  |
//!


use BuildQuery;

use serde_json;

use gitlab::GitLab;
use Issues;


impl GitLab {
    pub fn project_issues(&self, listing: Listing) -> Result<Issues, serde_json::Error> {
        let query = listing.build_query();
        self.get(&query)
    }
}


#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// The ID of a project
    id: i64,
    /// The exact IID of an issue
    iid: Option<i64>,
    /// State of issues to return.
    state: Option<::issues::ListingState>,
    /// Labels of issues to return.
    labels: Vec<String>,
    /// The milestone title
    milestone: String,
    /// Return requests ordered by. Default is `::issues::ListingOrderBy::CreatedAt`.
    order_by: Option<::issues::ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
}


#[allow(dead_code)]
impl Listing {
    pub fn new(id: i64) -> Listing {
        Listing {id: id, ..Default::default()}
    }
    pub fn iid(&mut self, iid: i64) -> &mut Listing {
        self.iid = Some(iid);
        self
    }
    pub fn state(&mut self, state: ::issues::ListingState) -> &mut Listing {
        self.state = Some(state);
        self
    }
    pub fn labels(&mut self, labels: Vec<String>) -> &mut Listing {
        self.labels = labels;
        self
    }
    pub fn milestone(&mut self, milestone: String) -> &mut Listing {
        self.milestone = milestone;
        self
    }
    pub fn order_by(&mut self, order_by: ::issues::ListingOrderBy) -> &mut Listing {
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
        let mut query = format!("projects/{}/issues", self.id);
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
                ::issues::ListingState::Opened => "opened",
                ::issues::ListingState::Closed => "closed",
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

        if !self.milestone.is_empty() {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("milestone=");
            query.push_str(&self.milestone);
        }

        self.order_by.map(|order_by| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            query.push_str(match order_by {
                ::issues::ListingOrderBy::CreatedAt => "created_at",
                ::issues::ListingOrderBy::UpdatedAt => "updated_at",
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

        self.iid.map(|iid| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("iid=");
            let iid_string = format!("{}", iid);
            query.push_str(&iid_string);
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
        let expected_string = "projects/123/issues";
        let listing = Listing::new(123);
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_state() {
        let expected_string = "projects/123/issues?state=opened";
        let query = Listing::new(123).state(::issues::ListingState::Opened).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/123/issues?state=closed";
        let query = Listing::new(123).state(::issues::ListingState::Closed).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_skip_groups() {
        let expected_string = "projects/123/issues?labels=label1,label2,label3";
        let query = Listing::new(123).labels(vec![String::from("label1"), String::from("label2"), String::from("label3")]).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let expected_string = "projects/123/issues?order_by=created_at";
        let query = Listing::new(123).order_by(::issues::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/123/issues?order_by=updated_at";
        let query = Listing::new(123).order_by(::issues::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let expected_string = "projects/123/issues?sort=asc";
        let query = Listing::new(123).sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/123/issues?sort=desc";
        let query = Listing::new(123).sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_multiple() {
        let expected_string = "projects/123/issues?order_by=created_at&sort=asc";
        let query = Listing::new(123).sort(::ListingSort::Asc).order_by(::issues::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);
    }
}
