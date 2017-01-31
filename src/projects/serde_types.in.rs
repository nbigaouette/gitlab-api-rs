
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


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectOwner {
    pub name: String,
    pub username: String,
    pub id: i64,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNamespaceAvatar {
    pub url: Option<String>,
}


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ProjectNamespace {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub owner_id: Option<i64>,  // FIXME: Why would a project not have this?
    pub created_at: Option<String>,  // FIXME: Date instead?
    pub updated_at: Option<String>,  // FIXME: Date instead?
    pub description: Option<String>,
    pub avatar: Option<ProjectNamespaceAvatar>,
    pub membership_lock: Option<bool>,
    pub share_with_group_lock: Option<bool>,
    pub visibility_level: Option<i64>,
    pub request_access_enabled: Option<bool>,
    pub ldap_sync_status: Option<String>,
    pub ldap_sync_error: Option<String>,  // FIXME: Is String the proper type?
    pub ldap_sync_last_update_at: Option<String>,  // FIXME: Is String the proper type?
    pub ldap_sync_last_successful_update_at: Option<String>,  // FIXME: Is String the proper type?
    pub ldap_sync_last_sync_at: Option<String>,  // FIXME: Is String the proper type?
    pub deleted_at: Option<String>,  // FIXME: Is String the proper type?
    pub lfs_enabled: Option<String>,  // FIXME: Is String the proper type?
    pub repository_size_limit: Option<String>  // FIXME: Is String the proper type?
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectForkedFrom {
    pub id: i64,
    pub http_url_to_repo: String,
    pub web_url: String,
    pub name: String,
    pub name_with_namespace: String,
    pub path: String,
    pub path_with_namespace: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectAccess {
    pub access_level: i64,
    pub notification_level: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectPermissions {
    pub project_access: Option<ProjectAccess>,
    pub group_access: Option<ProjectAccess>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSharedWithGroup {
    pub group_id: i64,
    pub group_name: String,
    pub group_access_level: i64,
}


// https://users.rust-lang.org/t/what-am-i-doing-wrong-go-program-is-12x-faster-than-rust/5692/13
// https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub description: String,
    pub default_branch: Option<String>,
    pub tag_list: Vec<String>,
    pub public: bool,
    pub archived: bool,
    pub visibility_level: i64,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub web_url: String,
    // owner: Option<ProjectOwner>,  // FIXME: Why would a project not have an owner?
    pub name: String,
    pub name_with_namespace: String,
    pub path: String,
    pub path_with_namespace: String,
    pub container_registry_enabled: Option<bool>,
    pub issues_enabled: Option<bool>,
    pub merge_requests_enabled: Option<bool>,
    pub wiki_enabled: Option<bool>,
    pub builds_enabled: Option<bool>,
    pub snippets_enabled: Option<bool>,
    pub created_at: String,  // FIXME: Date instead?
    pub last_activity_at: String,  // FIXME: Date instead?
    pub shared_runners_enabled: bool,
    pub lfs_enabled: bool,
    pub creator_id: i64,
    pub namespace: ProjectNamespace,
    pub forked_from_project: Option<ProjectForkedFrom>,
    pub avatar_url: Option<String>,
    pub star_count: i64,
    pub forks_count: i64,
    pub open_issues_count: Option<i64>,
    pub runners_token: Option<String>,
    pub public_builds: bool,
    pub shared_with_groups: Vec<ProjectSharedWithGroup>,
    pub only_allow_merge_if_build_succeeds: bool,
    pub request_access_enabled: bool,
    pub only_allow_merge_if_all_discussions_are_resolved: Option<bool>,  // FIXME: Is bool the proper type?
    pub approvals_before_merge: Option<i64>,
    pub permissions: Option<ProjectPermissions>,
}

pub type Projects = Vec<Project>;
