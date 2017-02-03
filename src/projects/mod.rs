//! List projects
//!
//! https://docs.gitlab.com/ce/api/projects.html
//!
//! # List projects
//!
//! Get a list of projects for which the authenticated user is a member.
//!
//! ```text
//! GET /projects
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `archived` | boolean | no | Limit by archived status |
//! | `visibility` | string | no | Limit by visibility `public`, `internal`, or `private` |
//! | `order_by` | string | no | Return projects ordered by `id`, `name`, `path`, `created_at`, `updated_at`, or `last_activity_at` fields. Default is `created_at` |
//! | `sort` | string | no | Return projects sorted in `asc` or `desc` order. Default is `desc` |
//! | `search` | string | no | Return list of authorized projects matching the search criteria |
//! | `simple` | boolean | no | Return only the ID, URL, name, and path of each project |
//!


use serde_urlencoded;

use BuildQuery;
use Lister;


pub mod all;
pub mod id_branches;
pub mod id_branch;
pub mod id_events;
pub mod id_hooks_id;
pub mod id_hooks;
pub mod id;
pub mod owned;
pub mod search;
pub mod starred;
pub mod visible;

use ::errors::*;


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


#[derive(Debug, Clone)]
pub struct ProjectsLister<'a> {
    gl: &'a ::GitLab,
    internal: ::projects::ProjectListerInternal,
}


impl<'a> Lister<::projects::Projects> for ProjectsLister<'a> {
    /// Commit the lister: Query GitLab and return a list of projects.
    fn list(&self) -> Result<::projects::Projects> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, None, None).chain_err(|| format!("cannot get query {}", query))
    }

    /// Commit the lister: Query GitLab and return a list of issues.
    fn list_paginated(&self, page: u16, per_page: u16) -> Result<::projects::Projects> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, page, per_page).chain_err(|| format!("cannot get query {}", query))
    }
}


impl<'a> ProjectsLister<'a> {
    pub fn new(gl: &'a ::GitLab) -> ProjectsLister {
        ProjectsLister {
            gl: gl,
            internal: ::projects::ProjectListerInternal {
                archived: None,
                visibility: None,
                order_by: None,
                sort: None,
                search: None,
                simple: None,
            },
        }
    }

    pub fn all(self) -> all::ProjectsLister<'a> {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        all::ProjectsLister::new(self.gl)
    }

    pub fn owned(self) -> owned::ProjectsLister<'a> {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        owned::ProjectsLister::new(self.gl)
    }

    pub fn search(self, query: String) -> search::ProjectsLister<'a> {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        search::ProjectsLister::new(self.gl, query)
    }

    pub fn id(self, id: ListingId) -> id::ProjectsLister<'a> {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        id::ProjectsLister::new(self.gl, id)
    }



    pub fn archived(&'a mut self, archived: bool) -> &'a mut ProjectsLister {
        self.internal.archived = Some(archived);
        self
    }

    pub fn visibility(&'a mut self, visibility: ::ListingVisibility) -> &'a mut ProjectsLister {
        self.internal.visibility = Some(visibility);
        self
    }

    pub fn order_by(&'a mut self, order_by: ListingOrderBy) -> &'a mut ProjectsLister {
        self.internal.order_by = Some(order_by);
        self
    }

    pub fn sort(&'a mut self, sort: ::ListingSort) -> &'a mut ProjectsLister {
        self.internal.sort = Some(sort);
        self
    }

    pub fn search_pattern(&'a mut self, search: String) -> &'a mut ProjectsLister {
        self.internal.search = Some(search);
        self
    }

    pub fn simple(&'a mut self, simple: bool) -> &'a mut ProjectsLister {
        self.internal.simple = Some(simple);
        self
    }
}

impl<'a> BuildQuery for ProjectsLister<'a> {
    fn build_query(&self) -> String {

        let encoded = serde_urlencoded::to_string(&self.internal).unwrap();
        let mut query = String::from("projects");
        if !encoded.is_empty() {
            query.push_str("?");
            query.push_str(&encoded);
        }

        query
    }
}


impl<'a> Project {
    /// Return a lister for the project's issues
    pub fn issues(&'a self, gl: &'a ::GitLab) -> ::issues::project::IssuesLister {
        ::issues::project::IssuesLister::new(gl, self.id)
    }

    /// Return a lister for the project's merge requests
    pub fn merge_requests(&'a self, gl: &'a ::GitLab) -> ::merge_requests::MergeRequestsLister {
        ::merge_requests::MergeRequestsLister::new(gl, self.id)
    }
}



#[cfg(test)]
mod tests {
    use BuildQuery;


    #[test]
    fn build_query_default() {
        let expected_string = "projects";

        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let projects_lister = gl.projects();
        let query = projects_lister.build_query();
        assert_eq!(query, expected_string);

        let query = gl.projects().build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_archived() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?archived=true";

        let mut projects_lister = gl.projects();
        let query = projects_lister.archived(true).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().archived(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?archived=false";

        let mut projects_lister = gl.projects();
        let query = projects_lister.archived(false).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().archived(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_visibility() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?visibility=public";
        let mut projects_lister = gl.projects();
        let query = projects_lister.visibility(::ListingVisibility::Public).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().visibility(::ListingVisibility::Public).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?visibility=internal";
        let mut projects_lister = gl.projects();
        let query = projects_lister.visibility(::ListingVisibility::Internal).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().visibility(::ListingVisibility::Internal).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?visibility=private";
        let mut projects_lister = gl.projects();
        let query = projects_lister.visibility(::ListingVisibility::Private).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().visibility(::ListingVisibility::Private).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?order_by=id";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=name";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=path";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=created_at";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=updated_at";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().order_by(::projects::ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?order_by=last_activity_at";
        let mut projects_lister = gl.projects();
        let query = projects_lister.order_by(::projects::ListingOrderBy::LastActivityAt)
            .build_query();
        assert_eq!(query, expected_string);
        let query =
            gl.projects().order_by(::projects::ListingOrderBy::LastActivityAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?sort=asc";
        let mut projects_lister = gl.projects();
        let query = projects_lister.sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?sort=desc";
        let mut projects_lister = gl.projects();
        let query = projects_lister.sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_search() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?search=SearchPattern";
        let mut projects_lister = gl.projects();
        let query = projects_lister.search_pattern(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().search_pattern(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_simple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?simple=true";
        let mut projects_lister = gl.projects();
        let query = projects_lister.simple(true).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().simple(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?simple=false";
        let mut projects_lister = gl.projects();
        let query = projects_lister.simple(false).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().simple(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_multiple() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "projects?archived=true&simple=true";
        let mut projects_lister = gl.projects();
        let query = projects_lister.archived(true).simple(true).build_query();
        assert_eq!(query, expected_string);
        let query = gl.projects().archived(true).simple(true).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn project_to_issues() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        let project_id = 123;
        let project = ::projects::Project { id: project_id, ..Default::default() };
        let issues_lister = format!("{:?}", project.issues(&gl));
        let default_issues_lister = format!("{:?}",
                                            ::issues::project::IssuesLister::new(&gl, project_id));
        assert_eq!(issues_lister, default_issues_lister);
    }


    #[test]
    fn project_to_merge_requests() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        let project_id = 123;
        let project = ::projects::Project { id: project_id, ..Default::default() };
        let merge_requests_lister = format!("{:?}", project.merge_requests(&gl));
        let default_merge_requests_lister = format!("{:?}",
                    ::merge_requests::MergeRequestsLister::new(&gl, project_id));
        assert_eq!(merge_requests_lister, default_merge_requests_lister);
    }
}
