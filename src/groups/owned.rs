//! List owned groups
//!
//! https://docs.gitlab.com/ce/api/groups.html#list-owned-groups
//!
//! # List owned groups
//!
//! Get a list of groups which are owned by the authenticated user.
//!
//! ```text
//! GET /groups/owned
//! ```


use serde_json;

use BuildQuery;
use Groups;


#[derive(Debug, Clone)]
pub struct GroupsLister<'a> {
    gl: &'a ::GitLab,
}


impl<'a> GroupsLister<'a> {
    pub fn new(gl: &'a ::GitLab) -> GroupsLister {
        GroupsLister { gl: gl }
    }

    /// Commit the lister: Query GitLab and return a list of groups.
    pub fn list(&self) -> Groups {
        let query = self.build_query();
        debug!("query: {:?}", query);

        let groups: Result<Groups, serde_json::Error> = self.gl.get(&query);

        groups.unwrap()
    }
}


impl<'a> BuildQuery for GroupsLister<'a> {
    fn build_query(&self) -> String {
        String::from("groups/owned")
    }
}


#[cfg(test)]
mod tests {
    use BuildQuery;

    #[test]
    fn build_query_default_split0() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups/owned";

        let lister = gl.groups();
        let lister = lister.owned();
        let query = lister.build_query();
        assert_eq!(query, expected_string);
    }

    #[test]
    fn build_query_default_split1() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups/owned";

        let lister = gl.groups().owned();
        let query = lister.build_query();
        assert_eq!(query, expected_string);
    }

    #[test]
    fn build_query_default() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX");
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups/owned";

        let query = gl.groups().owned().build_query();
        assert_eq!(query, expected_string);
    }
}
