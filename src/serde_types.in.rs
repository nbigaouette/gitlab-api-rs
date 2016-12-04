
// https://users.rust-lang.org/t/what-am-i-doing-wrong-go-program-is-12x-faster-than-rust/5692/13
// https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html
// https://serde.rs/codegen-hybrid.html


// FIXME: Use https://github.com/nox/serde_urlencoded
// FIXME: Make sure that all structs above Issue are not using `String`s instead of `Enum`s.
// FIXME: Harmonize the different state enums (e.g. IssueState, MergeRequestState, Authors)
// FIXME: Move all enums here (ListingOrderBy, ListingSort, etc.)
// FIXME: Use a type for sha1
// FIXME: Use chrono crate for dates
// FIXME: Use unsigned integers where it makes sense (id, iid, etc.)
// FIXME: Verify all `match` in push_str() in build_query(): They should contain all members.
// FIXME: Get rid of build_query(), use serde's Serialize instead.
// FIXME: Write nicer wrappers, getting rid of Listing.



#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListingSort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListingVisibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "private")]
    Private,
}


#[derive(Debug, Serialize, Deserialize)]
enum IssueState {
    #[serde(rename = "opened")]
    Opened,

    #[serde(rename = "closed")]
    Closed,
}


#[derive(Debug, Serialize, Deserialize)]
enum UserState {
    #[serde(rename = "active")]
    Active,

    #[serde(rename = "blocked")]
    Blocked,
}


#[derive(Debug, Serialize, Deserialize)]
enum MilestoneState {
    #[serde(rename = "active")]
    Active,

    #[serde(rename = "closed")]
    Closed,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub revision: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub description: String,
    pub visibility_level: i64,
    pub lfs_enabled: bool,
    pub avatar_url: Option<String>,
    pub web_url: String,
    pub request_access_enabled: bool,
}

pub type Groups = Vec<Group>;








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
struct ProjectNamespaceAvatar {
    url: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
struct ProjectNamespace {
    id: i64,
    name: String,
    path: String,
    owner_id: Option<i64>,  // FIXME: Why would a project not have this?
    created_at: String,  // FIXME: Date instead?
    updated_at: String,  // FIXME: Date instead?
    description: String,
    avatar: Option<ProjectNamespaceAvatar>,
    membership_lock: Option<bool>,
    share_with_group_lock: bool,
    visibility_level: i64,
    request_access_enabled: bool,
    ldap_sync_status: Option<String>,
    ldap_sync_error: Option<String>,  // FIXME: Is String the proper type?
    ldap_sync_last_update_at: Option<String>,  // FIXME: Is String the proper type?
    ldap_sync_last_successful_update_at: Option<String>,  // FIXME: Is String the proper type?
    ldap_sync_last_sync_at: Option<String>,  // FIXME: Is String the proper type?
    deleted_at: Option<String>,  // FIXME: Is String the proper type?
    lfs_enabled: Option<String>,  // FIXME: Is String the proper type?
    repository_size_limit: Option<String>  // FIXME: Is String the proper type?
}


#[derive(Debug, Serialize, Deserialize)]
struct ProjectForkedFrom {
    id: i64,
    http_url_to_repo: String,
    web_url: String,
    name: String,
    name_with_namespace: String,
    path: String,
    path_with_namespace: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct ProjectAccess {
    access_level: i64,
    notification_level: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectPermissions {
    project_access: Option<ProjectAccess>,
    group_access: Option<ProjectAccess>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectSharedWithGroup {
    group_id: i64,
    group_name: String,
    group_access_level: i64,
}


// https://users.rust-lang.org/t/what-am-i-doing-wrong-go-program-is-12x-faster-than-rust/5692/13
// https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    id: i64,
    description: String,
    default_branch: Option<String>,
    tag_list: Vec<String>,
    public: bool,
    archived: bool,
    visibility_level: i64,
    ssh_url_to_repo: String,
    http_url_to_repo: String,
    web_url: String,
    // owner: Option<ProjectOwner>,  // FIXME: Why would a project not have an owner?
    name: String,
    name_with_namespace: String,
    path: String,
    path_with_namespace: String,
    container_registry_enabled: Option<bool>,
    issues_enabled: bool,
    merge_requests_enabled: bool,
    wiki_enabled: bool,
    builds_enabled: bool,
    snippets_enabled: bool,
    created_at: String,  // FIXME: Date instead?
    last_activity_at: String,  // FIXME: Date instead?
    shared_runners_enabled: bool,
    lfs_enabled: bool,
    creator_id: i64,
    namespace: ProjectNamespace,
    forked_from_project: Option<ProjectForkedFrom>,
    avatar_url: Option<String>,
    star_count: i64,
    forks_count: i64,
    open_issues_count: i64,
    runners_token: Option<String>,
    public_builds: bool,
    shared_with_groups: Vec<ProjectSharedWithGroup>,
    only_allow_merge_if_build_succeeds: bool,
    request_access_enabled: bool,
    only_allow_merge_if_all_discussions_are_resolved: Option<bool>,  // FIXME: Is bool the proper type?
    approvals_before_merge: Option<i64>,
    permissions: ProjectPermissions,
}

pub type Projects = Vec<Project>;



#[derive(Debug, Serialize, Deserialize)]
struct Milestone {
    id: i64,
    iid: i64,
    project_id: i64,
    title: String,
    description: String,
    state: MilestoneState,
    created_at: String,  // FIXME: Use date type?
    updated_at: String,  // FIXME: Use date type?
    due_date: Option<String>  // FIXME: Use date type?
}


#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    username: String,
    id: i64,
    state: UserState,
    avatar_url: Option<String>,
    web_url: Option<String>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    id: i64,
    iid: i64,
    project_id: i64,
    title: String,
    description: String,
    state: IssueState,
    created_at: String,  // FIXME: Use date type?
    updated_at: String,  // FIXME: Use date type?
    labels: Vec<String>,
    milestone: Option<Milestone>,
    assignee: Option<User>,
    author: User,
    subscribed: bool,
    user_notes_count: i64,
    upvotes: i64,
    downvotes: i64,
    due_date: Option<String>,  // FIXME: Use date type?
    confidential: bool,
    web_url: Option<String>
}


pub type Issues = Vec<Issue>;
