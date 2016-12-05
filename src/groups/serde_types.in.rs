#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListingOrderBy {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "path")]
    Path,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct GroupsListerInternal {
    /// Skip the group IDs passes
    skip_groups: Option<Vec<i64>>,
    /// Show all the groups you have access to
    all_available: Option<bool>,
    /// Return list of authorized groups matching the search criteria
    search: Option<String>,
    /// Order groups by `name` or `path`. Default is `name`
    order_by: Option<ListingOrderBy>,
    /// Order groups in `asc` or `desc` order. Default is `asc`
    sort: Option<::ListingSort>,
}
