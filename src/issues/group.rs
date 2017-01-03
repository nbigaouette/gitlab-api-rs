//! List group issues
//!
//! https://docs.gitlab.com/ce/api/issues.html#list-group-issues
//!
//! # List group issues
//!
//! Get a list of a group's issues.
//!
//! ```text
//! GET /groups/ID/issues
//! GET /groups/ID/issues?state=opened
//! GET /groups/ID/issues?state=closed
//! GET /groups/ID/issues?labels=foo
//! GET /groups/ID/issues?labels=foo,bar
//! GET /groups/ID/issues?labels=foo,bar&state=opened
//! GET /groups/ID/issues?milestone=1.0.0
//! GET /groups/ID/issues?milestone=1.0.0&state=opened
//! ```
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id`      | integer | yes   | The ID of a group |
//! | `state`   | string  | no    | Return all issues or just those that are `opened` or `closed`|
//! | `labels`  | string  | no    | Comma-separated list of label names, issues must have all labels to be returned |
//! | `milestone` | string| no    | The milestone title |
//! | `order_by`| string  | no    | Return requests ordered by `created_at` or `updated_at` fields. Default is `created_at` |
//! | `sort`    | string  | no    | Return requests sorted in `asc` or `desc` order. Default is `desc`  |
//!


use serde_urlencoded;

use BuildQuery;
use Issues;
use issues::GroupIssuesListerInternal;

use ::errors::*;


#[derive(Debug, Clone)]
pub struct IssuesLister<'a> {
    gl: &'a ::GitLab,
    /// The ID of a group
    id: i64,
    internal: GroupIssuesListerInternal,
}


impl<'a> IssuesLister<'a> {
    pub fn new(gl: &'a ::GitLab, id: i64) -> IssuesLister {
        IssuesLister {
            gl: gl,
            id: id,
            internal: GroupIssuesListerInternal {
                state: None,
                labels: None,
                milestone: None,
                order_by: None,
                sort: None,
            },
        }
    }


    pub fn state(&'a mut self, state: ::issues::State) -> &'a mut IssuesLister {
        self.internal.state = Some(state);
        self
    }

    pub fn milestone(&'a mut self, milestone: String) -> &'a mut IssuesLister {
        self.internal.milestone = Some(milestone);
        self
    }

    pub fn labels(&'a mut self, labels: Vec<String>) -> &'a mut IssuesLister {
        self.internal.labels = Some(labels);
        self
    }

    pub fn order_by(&'a mut self, order_by: ::issues::ListingOrderBy) -> &'a mut IssuesLister {
        self.internal.order_by = Some(order_by);
        self
    }

    pub fn sort(&'a mut self, sort: ::ListingSort) -> &'a mut IssuesLister {
        self.internal.sort = Some(sort);
        self
    }


    /// Commit the lister: Query GitLab and return a list of issues.
    pub fn list(&self) -> Result<Issues> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, None, None).chain_err(|| format!("cannot get query {}", query))
    }
}



impl<'a> BuildQuery for IssuesLister<'a> {
    fn build_query(&self) -> String {

        // NOTE: Can't use `serde_urlencoded` since it cannot serialize a Vec<T>
        //       See https://github.com/nox/serde_urlencoded/issues/6
        // let encoded = serde_urlencoded::to_string(&self.internal).unwrap();

        let mut query = format!("groups/{}/issues", self.id);
        let amp_char = "&";
        let comma_char = ",";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?" only if at least one of the `Option` is `Some(_)` or if
        // strings contain something.
        query.push_str(match (&self.internal.state,
                              &self.internal.labels,
                              &self.internal.milestone,
                              &self.internal.order_by,
                              &self.internal.sort) {
            (&None, &None, &None, &None, &None) => "",
            _ => "?",
        });

        self.internal.state.map(|state| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("state=");
            query.push_str(match state {
                ::issues::State::Opened => "opened",
                ::issues::State::Closed => "closed",
            });
        });

        self.internal.labels.as_ref().map(|labels| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("labels=");

            let mut array_split_char = &none_char;
            for label in labels {
                query.push_str(array_split_char);
                query.push_str(&label.to_string());
                array_split_char = &comma_char;
            }
        });

        self.internal.milestone.as_ref().map(|milestone| {
            query.push_str(split_char);
            split_char = &amp_char;

            let params = &[("milestone", milestone)];
            query.push_str(&serde_urlencoded::to_string(&params).unwrap());
        });

        self.internal.order_by.map(|order_by| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            query.push_str(match order_by {
                ::issues::ListingOrderBy::CreatedAt => "created_at",
                ::issues::ListingOrderBy::UpdatedAt => "updated_at",
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
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}/issues", TEST_PROJECT_ID);

        let lister = gl.issues();
        let lister = lister.group(TEST_PROJECT_ID);
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let lister = gl.issues().group(TEST_PROJECT_ID);
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let query = gl.issues().group(TEST_PROJECT_ID).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_state() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups/123/issues?state=opened";
        let query = gl.issues().group(TEST_PROJECT_ID).state(::issues::State::Opened).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "groups/123/issues?state=closed";
        let query = gl.issues().group(TEST_PROJECT_ID).state(::issues::State::Closed).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_milestone() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups/123/issues?milestone=Test+Milestone";
        let query = gl.issues()
            .group(TEST_PROJECT_ID)
            .milestone("Test Milestone".to_string())
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_skip_groups() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups/123/issues?labels=label1,label2,label3";
        let query = gl.issues()
            .group(TEST_PROJECT_ID)
            .labels(vec![String::from("label1"), String::from("label2"), String::from("label3")])
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups/123/issues?order_by=created_at";
        let query = gl.issues()
            .group(TEST_PROJECT_ID)
            .order_by(::issues::ListingOrderBy::CreatedAt)
            .build_query();
        assert_eq!(query, expected_string);

        let expected_string = "groups/123/issues?order_by=updated_at";
        let query = gl.issues()
            .group(TEST_PROJECT_ID)
            .order_by(::issues::ListingOrderBy::UpdatedAt)
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups/123/issues?sort=asc";
        let query = gl.issues().group(TEST_PROJECT_ID).sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "groups/123/issues?sort=desc";
        let query = gl.issues().group(TEST_PROJECT_ID).sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_multiple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups/123/issues?order_by=created_at&sort=asc";
        let query = gl.issues()
            .group(TEST_PROJECT_ID)
            .sort(::ListingSort::Asc)
            .order_by(::issues::ListingOrderBy::CreatedAt)
            .build_query();
        assert_eq!(query, expected_string);
    }
}
