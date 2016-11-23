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

// FIXME: Use a type for the project id.

#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// Group Id.
    id: i64,
}


impl Listing {
    pub fn new(id: i64) -> Listing {
        Listing { id: id }
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {
        format!("groups/{}", self.id)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;

    const TEST_PROJECT_ID: i64 = 123;


    #[test]
    fn groups_build_query_default() {
        let expected_string = format!("groups/{}", TEST_PROJECT_ID);
        let listing = Listing::new(TEST_PROJECT_ID);
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }
}
