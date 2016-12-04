
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
enum Status {
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
pub struct MergeRequestsListerInternal {
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
    id: i64,
    iid: i64,
    project_id: i64,
    title: String,
    description: String,
    state: State,
    created_at: String,  // FIXME: Use chrono crate
    updated_at: String,  // FIXME: Use chrono crate
    target_branch: String,
    source_branch: String,
    upvotes: i64,
    downvotes: i64,
    author: ::User,
    assignee: Option<::User>,
    source_project_id: i64,
    target_project_id: i64,
    labels: Vec<String>,
    work_in_progress: bool,
    milestone: Option<::Milestone>,
    merge_when_build_succeeds: bool,
    merge_status: Status,
    sha: String,
    merge_commit_sha: Option<String>,
    subscribed: bool,
    user_notes_count: i64,
    should_remove_source_branch: Option<bool>,
    force_remove_source_branch: Option<bool>,
    web_url: String
}

pub type MergeRequests = Vec<MergeRequest>;
