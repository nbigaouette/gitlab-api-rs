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


// use serde_urlencoded;

use BuildQuery;
use Lister;

pub mod group;
pub mod project;
pub mod single;

use ::errors::*;


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "closed")]
    Closed,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListingOrderBy {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct IssuesListerInternal {
    /// State of issues to return.
    state: Option<State>,
    /// Labels of issues to return.
    labels: Option<Vec<String>>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct GroupIssuesListerInternal {
    /// State of issues to return.
    state: Option<State>,
    /// Labels of issues to return.
    labels: Option<Vec<String>>,
    /// The milestone title
    milestone: Option<String>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
}


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct ProjectsIssuesListerInternal {
    iid: Option<i64>,
    /// State of issues to return.
    state: Option<State>,
    /// Labels of issues to return.
    labels: Option<Vec<String>>,
    /// The milestone title
    milestone: Option<String>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum IssueState {
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "reopened")]
    Reopened,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub iid: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String,
    pub state: IssueState,
    pub created_at: String,  // FIXME: Use date type?
    pub updated_at: String,  // FIXME: Use date type?
    pub labels: Vec<String>,
    pub milestone: Option<::Milestone>,
    pub assignee: Option<::User>,
    pub author: ::User,
    pub subscribed: bool,
    pub user_notes_count: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub due_date: Option<String>,  // FIXME: Use date type?
    pub confidential: bool,
    pub web_url: Option<String>
}


pub type Issues = Vec<Issue>;


#[derive(Debug, Clone)]
pub struct IssuesLister<'a> {
    gl: &'a ::GitLab,
    internal: IssuesListerInternal,
}


impl<'a> Lister<Issues> for IssuesLister<'a> {
    /// Commit the lister: Query GitLab and return a list of issues.
    fn list(&self) -> Result<Issues> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, None, None).chain_err(|| format!("cannot get query {}", query))
    }

    /// Commit the lister: Query GitLab and return a list of issues.
    fn list_paginated(&self, page: u16, per_page: u16) -> Result<Issues> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, page, per_page).chain_err(|| format!("cannot get query {}", query))
    }
}


impl<'a> IssuesLister<'a> {
    pub fn new(gl: &'a ::GitLab) -> IssuesLister {
        IssuesLister {
            gl: gl,
            internal: IssuesListerInternal {
                state: None,
                labels: None,
                order_by: None,
                sort: None,
            },
        }
    }


    pub fn group(self, id: i64) -> group::IssuesLister<'a> {
        // assert_eq!(self, IssuesLister::new(self.gl));
        group::IssuesLister::new(self.gl, id)
    }

    pub fn project(self, id: i64) -> project::IssuesLister<'a> {
        // assert_eq!(self, IssuesLister::new(self.gl));
        project::IssuesLister::new(self.gl, id)
    }

    pub fn single(self, id: i64, issue_id: i64) -> single::IssueLister<'a> {
        // assert_eq!(self, IssuesLister::new(self.gl));
        single::IssueLister::new(self.gl, id, issue_id)
    }


    pub fn state(&'a mut self, state: State) -> &'a mut IssuesLister {
        self.internal.state = Some(state);
        self
    }

    pub fn labels(&'a mut self, labels: Vec<String>) -> &'a mut IssuesLister {
        self.internal.labels = Some(labels);
        self
    }

    pub fn order_by(&'a mut self, order_by: ListingOrderBy) -> &'a mut IssuesLister {
        self.internal.order_by = Some(order_by);
        self
    }

    pub fn sort(&'a mut self, sort: ::ListingSort) -> &'a mut IssuesLister {
        self.internal.sort = Some(sort);
        self
    }


    // /// Commit the lister: Query GitLab and return a list of issues.
    // pub fn list(&self) -> Result<Issues> {
    //     let query = self.build_query();
    //     debug!("query: {:?}", query);
    //
    //     self.gl.get(&query, None, None).chain_err(|| format!("cannot get query {}", query))
    // }
}


impl<'a> BuildQuery for IssuesLister<'a> {
    fn build_query(&self) -> String {

        // NOTE: Can't use `serde_urlencoded` since it cannot serialize a Vec<T>
        //       See https://github.com/nox/serde_urlencoded/issues/6
        // let encoded = serde_urlencoded::to_string(&self.internal).unwrap();

        let mut query = String::from("issues");

        let amp_char = "&";
        let comma_char = ",";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?" only if at least one of the `Option` is `Some(_)` or if
        // strings contain something.
        query.push_str(match (&self.internal.state,
                              &self.internal.labels,
                              &self.internal.order_by,
                              &self.internal.sort) {
            (&None, &None, &None, &None) => "",
            _ => "?",
        });

        self.internal.state.map(|state| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("state=");
            query.push_str(match state {
                State::Opened => "opened",
                State::Closed => "closed",
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


    #[test]
    fn build_query_default() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "issues";
        let lister = gl.issues();
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let expected_string = "issues";
        let query = gl.issues().build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_state() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "issues?state=opened";
        let query = gl.issues().state(::issues::State::Opened).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "issues?state=closed";
        let query = gl.issues().state(::issues::State::Closed).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_skip_groups() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "issues?labels=label1,label2,label3";
        let query = gl.issues()
            .labels(vec![String::from("label1"), String::from("label2"), String::from("label3")])
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "issues?order_by=created_at";
        let query = gl.issues().order_by(::issues::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "issues?order_by=updated_at";
        let query = gl.issues().order_by(::issues::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "issues?sort=asc";
        let query = gl.issues().sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "issues?sort=desc";
        let query = gl.issues().sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_multiple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "issues?order_by=created_at&sort=asc";
        let query = gl.issues()
            .sort(::ListingSort::Asc)
            .order_by(::issues::ListingOrderBy::CreatedAt)
            .build_query();
        assert_eq!(query, expected_string);
    }
}
