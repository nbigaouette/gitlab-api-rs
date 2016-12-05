#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListingOrderBy {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "path")]
    Path,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListingId {
    Id(i64),
    NamespaceProject(String),
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct ProjectsListerInternal {
    /// Limit by archived status.
    archived: Option<bool>,
    /// Limit by visibility
    visibility: Option<::ListingVisibility>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<::projects::ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
    /// Return list of authorized projects according to a search criteria.
    search: Option<String>,
    /// Return projects ordered by `ci_enabled` flag. Projects with enabled GitLab CI go first.
    ci_enabled_first: Option<bool>,
}
