
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ListingOrderBy {
    Id,
    Name,
    Path,
    CreatedAt,
    UpdatedAt,
    LastActivityAt,
}

macro_attr! {
    #[derive(Debug, Clone, Serialize, Deserialize, Builder!)]
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
        search: String,
        /// Return only the ID, URL, name, and path of each project
        simple: Option<bool>,
    }
}
