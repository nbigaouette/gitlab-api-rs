
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ListingOrderBy {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
    #[serde(rename = "last_activity_at")]
    LastActivityAt,
}


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SearchListingOrderBy {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    // #[serde(rename = "path")]
    // Path,
    #[serde(rename = "created_at")]
    CreatedAt,
    // #[serde(rename = "updated_at")]
    // UpdatedAt,
    #[serde(rename = "last_activity_at")]
    LastActivityAt,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectListerInternal {
    /// Limit by archived status
    archived: Option<bool>,
    /// Limit by visibility.
    visibility: Option<::ListingVisibility>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
    /// Return list of authorized projects matching the search criteria.
    search: Option<String>,
    /// Return only the ID, URL, name, and path of each project
    simple: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct OwnedProjectListerInternal {
    /// Limit by archived status
    archived: Option<bool>,
    /// Limit by visibility.
    visibility: Option<::ListingVisibility>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
    /// Return list of authorized projects matching the search criteria.
    search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchProjectListerInternal {
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListingId {
    Id(i64),
    NamespaceProject(String),
}


type AllProjectListerInternal = OwnedProjectListerInternal;
