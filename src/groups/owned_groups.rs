//! List owned groups
//!
//! https://docs.gitlab.com/ce/api/groups.html#list-owned-groups
//!
//! # List owned groups
//!
//! Get a list of groups which are owned by the authenticated user.
//!
//! ```
//! GET /groups/owned
//! ```


use BuildQuery;


#[derive(Default, Debug, Clone)]
pub struct Listing {
}

impl Listing {
    pub fn new() -> Listing {
        Default::default()
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {
        String::from("groups/owned")
    }
}


#[test]
fn owned_groups_build_query_default() {
    let expected_string = "groups/owned";
    let listing: Listing = Default::default();
    let query = listing.build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups/owned";
    let listing = Listing::new();
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}
