//! List groups
//!
//! https://docs.gitlab.com/ce/api/groups.html#list-groups
//!
//! # List groups
//!
//! Get a list of groups. (As user: my groups or all available, as admin: all groups).
//!
//! ```text
//! GET /groups
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `skip_groups` | array of integers | no | Skip the group IDs passes |
//! | `all_available` | boolean | no | Show all the groups you have access to |
//! | `search` | string | no | Return list of authorized groups matching the search criteria |
//! | `order_by` | string | no | Order groups by `name` or `path`. Default is `name` |
//! | `sort` | string | no | Order groups in `asc` or `desc` order. Default is `asc` |
//!
//! You can search for groups by name or path.
//!
//! **NOTE**: The _Search for group_ (from
//! https://docs.gitlab.com/ce/api/groups.html#search-for-group) is performed in this module.
//!
//!


// use serde_urlencoded;

use BuildQuery;
use Groups;

pub mod owned;
pub mod projects;
pub mod details;

use ::errors::*;


#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/groups/serde_types.rs"));


#[derive(Debug, Clone)]
pub struct GroupsLister<'a> {
    gl: &'a ::GitLab,
    internal: GroupsListerInternal,
}


impl<'a> GroupsLister<'a> {
    pub fn new(gl: &'a ::GitLab) -> GroupsLister {
        GroupsLister {
            gl: gl,
            internal: GroupsListerInternal {
                skip_groups: None,
                all_available: None,
                search: None,
                order_by: None,
                sort: None,
            },
        }
    }


    pub fn details(self, id: ListingId) -> details::GroupLister<'a> {
        // assert_eq!(self, GroupLister::new(self.gl));
        details::GroupLister::new(self.gl, id)
    }

    pub fn owned(self) -> owned::GroupsLister<'a> {
        // assert_eq!(self, GroupsLister::new(self.gl));
        owned::GroupsLister::new(self.gl)
    }

    pub fn projects(self, id: i64) -> projects::ProjectsLister<'a> {
        // assert_eq!(self, ProjectsLister::new(self.gl));
        projects::ProjectsLister::new(self.gl, id)
    }


    pub fn skip_groups(&'a mut self, skip_groups: Vec<i64>) -> &'a mut GroupsLister {
        self.internal.skip_groups = Some(skip_groups);
        self
    }

    pub fn all_available(&'a mut self, all_available: bool) -> &'a mut GroupsLister {
        self.internal.all_available = Some(all_available);
        self
    }

    pub fn search(&'a mut self, search: String) -> &'a mut GroupsLister {
        self.internal.search = Some(search);
        self
    }

    pub fn order_by(&'a mut self, order_by: ListingOrderBy) -> &'a mut GroupsLister {
        self.internal.order_by = Some(order_by);
        self
    }

    pub fn sort(&'a mut self, sort: ::ListingSort) -> &'a mut GroupsLister {
        self.internal.sort = Some(sort);
        self
    }


    /// Commit the lister: Query GitLab and return a list of groups.
    pub fn list(&self) -> Result<Groups> {
        let query = self.build_query();
        debug!("query: {:?}", query);

        self.gl.get(&query, None, None).chain_err(|| format!("cannot get query {}", query))
    }
}


impl<'a> BuildQuery for GroupsLister<'a> {
    fn build_query(&self) -> String {

        // NOTE: Can't use `serde_urlencoded` since it cannot serialize a Vec<T>
        //       See https://github.com/nox/serde_urlencoded/issues/6
        // let encoded = serde_urlencoded::to_string(&self.internal).unwrap();

        let mut query = String::from("groups");

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?", only if one of the `Option` is `Some(_)`
        query.push_str(match (&self.internal.skip_groups,
                              &self.internal.all_available,
                              &self.internal.search,
                              &self.internal.order_by,
                              &self.internal.sort) {
            (&None, &None, &None, &None, &None) => "",
            _ => "?",
        });

        self.internal.skip_groups.as_ref().map(|skip_groups| {
            query.push_str(split_char);
            split_char = &amp_char;

            let mut array_split_char = &none_char;
            for skip_group in skip_groups {
                query.push_str(array_split_char);
                query.push_str("skip_groups[]=");
                query.push_str(&skip_group.to_string());
                array_split_char = &amp_char;
            }
        });

        self.internal.all_available.map(|all_available| {
            query.push_str(split_char);
            split_char = &amp_char;

            if all_available {
                query.push_str("all_available=true")
            } else {
                query.push_str("all_available=false")
            }
        });

        self.internal.search.as_ref().map(|search| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("search=");
            query.push_str(search);
        });

        self.internal.order_by.map(|order_by| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            query.push_str(match order_by {
                ListingOrderBy::Name => "name",
                ListingOrderBy::Path => "path",
            });
        });

        self.internal.sort.map(|sort| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("sort=");
            query.push_str(match sort {
                ::ListingSort::Asc => "asc",
                ::ListingSort::Desc => "desc",
            });
        });

        query
    }
}


#[cfg(test)]
mod tests {
    use BuildQuery;


    #[test]
    fn groups_build_query_default() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups";
        let lister = gl.groups();
        let query = lister.build_query();
        assert_eq!(query, expected_string);

        let expected_string = "groups";
        let query = gl.groups().build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_skip_groups() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups?skip_groups[]=1&skip_groups[]=2&skip_groups[]=3";
        let query = gl.groups().skip_groups(vec![1, 2, 3]).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_all_available() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups?all_available=true";
        let query = gl.groups().all_available(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "groups?all_available=false";
        let query = gl.groups().all_available(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_search() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups?search=SearchPattern";
        let query = gl.groups().search(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_order_by_name() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups?order_by=name";
        let query = gl.groups().order_by(::groups::ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_order_by_path() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups?order_by=path";
        let query = gl.groups().order_by(::groups::ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_sort() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups?sort=asc";
        let query = gl.groups().sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "groups?sort=desc";
        let query = gl.groups().sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_search_order_by_path() {
        let gl = ::GitLab::new(&"localhost", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        // let gl: ::GitLab = Default::default();

        let expected_string = "groups?search=SearchPattern&order_by=path";
        let query = gl.groups()
            .order_by(::groups::ListingOrderBy::Path)
            .search(String::from("SearchPattern"))
            .build_query();
        assert_eq!(query, expected_string);
        let query = gl.groups()
            .search(String::from("SearchPattern"))
            .order_by(::groups::ListingOrderBy::Path)
            .build_query();
        assert_eq!(query, expected_string);
    }
}
