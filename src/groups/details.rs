//! Details of a group
//!
//! https://docs.gitlab.com/ce/api/groups.html#details-of-a-group
//!
//! # Details of a group
//!
//! Get all details of a group.
//!
//! ```text
//! GET /groups/ID
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id` | integer/string | yes | The ID or path of a group |
//!


use BuildQuery;
use Group;

use ::errors::*;


#[derive(Debug, Clone)]
pub struct GroupLister<'a> {
    gl: &'a ::GitLab,
    /// The ID of a project
    id: ::groups::ListingId,
}


impl<'a> GroupLister<'a> {
    pub fn new(gl: &'a ::GitLab, id: ::groups::ListingId) -> GroupLister {
        GroupLister { gl: gl, id: id }
    }

    /// Commit the lister: Query GitLab and return a group.
    pub fn list(&self) -> Result<Group> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query).chain_err(|| format!("cannot get query {}", query))
    }
}


impl<'a> BuildQuery for GroupLister<'a> {
    fn build_query(&self) -> String {
        let mut query = String::from("groups/");

        query.push_str(&match self.id {
            ::groups::ListingId::Id(id) => id.to_string(),
            ::groups::ListingId::NamespaceProject(ref s) => s.replace("/", "%2F"),
        });

        query
    }
}


#[cfg(test)]
mod tests {
    use BuildQuery;

    const TEST_GROUP_ID_I64: i64 = 123;
    const TEST_GROUP_ID_STRING: &'static str = "group/project";


    #[test]
    fn build_query_default_i64() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}", TEST_GROUP_ID_I64);

        let lister = gl.groups();
        let lister = lister.details(::groups::ListingId::Id(TEST_GROUP_ID_I64));
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let lister = gl.groups().details(::groups::ListingId::Id(TEST_GROUP_ID_I64));
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let query = gl.groups().details(::groups::ListingId::Id(TEST_GROUP_ID_I64)).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_default_str() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = format!("groups/{}", TEST_GROUP_ID_STRING.replace("/", "%2F"));

        let lister = gl.groups();
        let lister =
            lister.details(::groups::ListingId::NamespaceProject(TEST_GROUP_ID_STRING.to_string()));
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let lister = gl.groups()
            .details(::groups::ListingId::NamespaceProject(TEST_GROUP_ID_STRING.to_string()));
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let query = gl.groups()
            .details(::groups::ListingId::NamespaceProject(TEST_GROUP_ID_STRING.to_string()))
            .build_query();
        assert_eq!(query, expected_string);
    }
}
