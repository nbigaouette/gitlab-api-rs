
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
pub enum UserState {
    #[serde(rename = "active")]
    Active,

    #[serde(rename = "blocked")]
    Blocked,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum MilestoneState {
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
pub struct Milestone {
    pub id: i64,
    pub iid: i64,
    pub project_id: i64,
    pub title: String,
    pub description: String,
    pub state: MilestoneState,
    pub created_at: String,  // FIXME: Use date type?
    pub updated_at: String,  // FIXME: Use date type?
    pub due_date: Option<String>  // FIXME: Use date type?
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub id: i64,
    pub state: UserState,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>
}
