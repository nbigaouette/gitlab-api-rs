

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ProjectOwner {
    pub name: String,
    pub username: String,
    pub id: i64,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String,
}


#[derive(Debug, RustcDecodable, RustcEncodable)]
struct ProjectNamespaceAvatar {
    url: Option<String>,
}


#[derive(Debug, RustcDecodable, RustcEncodable)]
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


#[derive(Debug, RustcDecodable, RustcEncodable)]
struct ProjectForkedFrom {
    id: i64,
    http_url_to_repo: String,
    web_url: String,
    name: String,
    name_with_namespace: String,
    path: String,
    path_with_namespace: String,
}


#[derive(Debug, RustcDecodable, RustcEncodable)]
struct ProjectAccess {
    access_level: i64,
    notification_level: i64,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct ProjectPermissions {
    project_access: Option<ProjectAccess>,
    group_access: Option<ProjectAccess>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct ProjectSharedWithGroup {
    group_id: i64,
    group_name: String,
    group_access_level: i64,
}


// https://users.rust-lang.org/t/what-am-i-doing-wrong-go-program-is-12x-faster-than-rust/5692/13
// https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html
#[derive(Debug, RustcDecodable, RustcEncodable)]
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
    owner: Option<ProjectOwner>,  // FIXME: Why would a project not have an owner?
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
    public_builds: bool,
    shared_with_groups: Vec<ProjectSharedWithGroup>,
    only_allow_merge_if_build_succeeds: bool,
    request_access_enabled: bool,
    only_allow_merge_if_all_discussions_are_resolved: Option<bool>,  // FIXME: Is bool the proper type?
    approvals_before_merge: Option<i64>,
    permissions: ProjectPermissions,
}

pub type Projects = Vec<Project>;
