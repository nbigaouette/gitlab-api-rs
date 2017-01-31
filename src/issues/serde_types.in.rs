

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
