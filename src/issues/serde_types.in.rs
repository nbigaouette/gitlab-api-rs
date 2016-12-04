

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
