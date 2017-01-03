
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "merged")]
    Merged,
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "can_be_merged")]
    CanBeMerged,
    #[serde(rename = "cannot_be_merged")]
    CannotBeMerged,
    #[serde(rename = "unchecked")]
    Unchecked,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListingOrderBy {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}




#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct MergeRequestsListerInternal {
    /// Merge request's IID
    iid: Option<Vec<i64>>,
    /// State of the requests
    state: Option<State>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `ListingSort::Desc`.
    sort: Option<::ListingSort>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct MergeRequest {
    pub id: i64,
    pub iid: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String,
    pub state: State,
    pub created_at: String,  // FIXME: Use chrono crate
    pub updated_at: String,  // FIXME: Use chrono crate
    pub target_branch: String,
    pub source_branch: String,
    pub upvotes: i64,
    pub downvotes: i64,
    pub author: ::User,
    pub assignee: Option<::User>,
    pub source_project_id: i64,
    pub target_project_id: i64,
    pub labels: Vec<String>,
    pub work_in_progress: bool,
    pub milestone: Option<::Milestone>,
    pub merge_when_build_succeeds: bool,
    pub merge_status: Status,
    pub sha: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub subscribed: bool,
    pub user_notes_count: i64,
    pub should_remove_source_branch: Option<bool>,
    pub force_remove_source_branch: Option<bool>,
    pub web_url: String
}

pub type MergeRequests = Vec<MergeRequest>;
