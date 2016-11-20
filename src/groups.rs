
// https://users.rust-lang.org/t/what-am-i-doing-wrong-go-program-is-12x-faster-than-rust/5692/13
// https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html
#[derive(Debug, RustcDecodable, RustcEncodable)]
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
