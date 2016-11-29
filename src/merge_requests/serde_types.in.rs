

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    /// Project id
    id: i64,
    /// Merge request's IID
    iid: Vec<i64>,
    /// State of the requests
    state: Option<MergeRequestState>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `ListingSort::Desc`.
    sort: Option<::ListingSort>,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListingOrderBy {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}
