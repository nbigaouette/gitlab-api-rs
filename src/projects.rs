

// #[derive(Debug)]
// pub struct ProjectManager {
//     // scheme: String,
// }

// https://users.rust-lang.org/t/what-am-i-doing-wrong-go-program-is-12x-faster-than-rust/5692/13
// https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Project {
    pub id: u32,  // Right size?
    pub description: String,
    pub default_branch: String,
    // // tag_list: [],
    // public: bool,
    // archived: bool,
    // visibility_level: u32,  // Right size?
    // ssh_url_to_repo: String,
    // http_url_to_repo: String,
    // web_url: String,
    // // "owner": {
    // //     "name": String,
    // //     "username": String,
    // //     "id": u32,  // Right size?
    // //     "state": String,
    // //     "avatar_url": String,
    // //     "web_url": String"
    // // },
    // name: String,
    // name_with_namespace: String,
    // path: String,
    // path_with_namespace: String,
    // container_registry_enabled: bool,
    // issues_enabled: bool,
    // merge_requests_enabled: bool,
    // wiki_enabled: bool,
    // builds_enabled: bool,
    // snippets_enabled: bool,
    // created_at: String,  // Date instead?
    // last_activity_at: String,  // Date instead?
    // shared_runners_enabled: bool,
    // lfs_enabled: bool,
    // creator_id: u32,  // Right size?
    // // "namespace": {
    // //     "id": u32,  // Right size?
    // //     "name": String,
    // //     "path": String,
    // //     "owner_id": u32,  // Right size?
    // //     "created_at": String,  // Date instead?
    // //     "updated_at": String,  // Date instead?
    // //     "description": String,
    // //     "avatar": null,
    // //     "membership_lock": bool,
    // //     "share_with_group_lock": bool,
    // //     "visibility_level": u32,  // Right size?
    // //     "request_access_enabled": bool,
    // //     "ldap_sync_status": "ready",
    // //     "ldap_sync_error": null,
    // //     "ldap_sync_last_update_at": null,
    // //     "ldap_sync_last_successful_update_at": null,
    // //     "ldap_sync_last_sync_at": null,
    // //     "deleted_at": null,
    // //     "lfs_enabled": null,
    // //     "repository_size_limit": null
    // // },
    // // "forked_from_project": {
    // //     "id": u32,  // Right size?
    // //     "http_url_to_repo": String,
    // //     "web_url": String,
    // //     "name": String,
    // //     "name_with_namespace": String,
    // //     "path": String,
    // //     "path_with_namespace": String,
    // // },
    // avatar_url: null,
    // star_count: u32,  // Right size?
    // forks_count: u32,  // Right size?
    // open_issues_count: u32,  // Right size?
    // public_builds: bool,
    // // shared_with_groups: [],
    // only_allow_merge_if_build_succeeds: bool,
    // request_access_enabled: bool,
    // // only_allow_merge_if_all_discussions_are_resolved: null,
    // approvals_before_merge: u32,  // Right size?
    // // "permissions": {
    // //     "project_access": {
    // //         "access_level": u32,  // Right size?
    // //         "notification_level": u32,  // Right size?
    // //     },
    // //     "group_access": null
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Projects {
    pub p: Vec<Project>,
}

