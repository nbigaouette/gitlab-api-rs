
use BuildQuery;
use Groups;

// https://docs.gitlab.com/ce/api/groups.html
// List groups:                 GET /groups
// List owned groups:           GET /groups/owned
// List a group's projects:     GET /groups/:id/projects
// Details of a group:          GET /groups/:id
// New group:                   POST /groups
// Transfer project to group:   POST  /groups/:id/projects/:project_id
// Update group:                PUT /groups/:id
// Remove group:                DELETE /groups/:id
// Search for group:            GET /groups?search=foobar

// https://docs.gitlab.com/ce/api/groups.html#list-groups


#[derive(Debug, Copy, Clone)]
pub enum GroupListerOptionsOrderBy {
    Name,
    Path,
}

// impl Default for GroupListerOptionsOrderBy {
//     fn default() -> GroupListerOptionsOrderBy { GroupListerOptionsOrderBy::Name }
// }

#[derive(Debug, Copy, Clone)]
pub enum GroupListerOptionsSort {
    Asc,
    Desc,
}

// impl Default for GroupListerOptionsSort {
//     fn default() -> GroupListerOptionsSort { GroupListerOptionsSort::Asc }
// }


fn append_group_lister_options_order_by(order_by: GroupListerOptionsOrderBy, s: &mut String) {
    s.push_str(match order_by {
        GroupListerOptionsOrderBy::Name => "name",
        GroupListerOptionsOrderBy::Path => "path",
    });
}


fn append_group_lister_options_sort(order_by: GroupListerOptionsSort, s: &mut String) {
    s.push_str(match order_by {
        GroupListerOptionsSort::Asc => "asc",
        GroupListerOptionsSort::Desc => "desc",
    });
}

/// https://docs.gitlab.com/ce/api/groups.html#list-groups
#[derive(Default, Debug)]
pub struct GroupListerOptions {
    /// Skip the group IDs passes
    pub skip_groups: Option<Vec<i64>>,
    /// Show all the groups you have access to
    pub all_available: Option<bool>,
    /// Return list of authorized groups matching the search criteria
    pub search: Option<String>,
    /// Order groups by `name` or `path`. Default is `name`
    pub order_by: Option<GroupListerOptionsOrderBy>,
    /// Order groups in `asc` or `desc` order. Default is `asc`
    pub sort: Option<GroupListerOptionsSort>,
}


#[derive(Default, Debug)]
pub struct GroupListing {
    pub options: GroupListerOptions,
}


impl BuildQuery for GroupListing {
    fn build_query(&self) -> String {

        let options = &self.options;

        let mut query = String::from("groups");

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?", only if one of the `Option` is `Some(_)`
        query.push_str(match (&options.skip_groups,
                              &options.all_available,
                              &options.search,
                              &options.order_by,
                              &options.sort) {
            (&None, &None, &None, &None, &None) => "",
            _ => "?",
        });

        options.skip_groups.as_ref().map(|skip_groups| {
            if !skip_groups.is_empty() {
                query.push_str(&split_char);
                split_char = &amp_char;

                let mut array_split_char = &none_char;
                for &skip_group in skip_groups {
                    query.push_str(array_split_char);
                    query.push_str("skip_groups[]=");
                    query.push_str(&skip_group.to_string());
                    array_split_char = &amp_char;
                }
            }
        });

        options.all_available.map(|all_available| {
            query.push_str(&split_char);
            split_char = &amp_char;

            if all_available {
                query.push_str("all_available=true")
            } else {
                query.push_str("all_available=false")
            }
        });

        options.search.as_ref().map(|search| {
            query.push_str(&split_char);
            split_char = &amp_char;

            query.push_str("search=");
            query.push_str(search);
        });

        options.order_by.map(|order_by| {
            query.push_str(&split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            append_group_lister_options_order_by(order_by, &mut query);
        });

        options.sort.map(|sort| {
            query.push_str(&split_char);
            split_char = &amp_char;

            query.push_str("sort=");
            append_group_lister_options_sort(sort, &mut query);
        });

        query
    }
}


impl GroupListing {
    fn list(&self) -> Groups {
        let groups: Groups = vec![];

        groups
    }
}


#[test]
fn test_append_group_lister_options_order_by() {
    let expected_string = "Initialnamepathpathname";

    let mut s = String::from("Initial");

    append_group_lister_options_order_by(GroupListerOptionsOrderBy::Name, &mut s);
    append_group_lister_options_order_by(GroupListerOptionsOrderBy::Path, &mut s);
    append_group_lister_options_order_by(GroupListerOptionsOrderBy::Path, &mut s);
    append_group_lister_options_order_by(GroupListerOptionsOrderBy::Name, &mut s);

    assert_eq!(s, expected_string);
}


#[test]
fn test_append_group_lister_options_sort() {
    let expected_string = "Initialascdescascdesc";

    let mut s = String::from("Initial");

    append_group_lister_options_sort(GroupListerOptionsSort::Asc, &mut s);
    append_group_lister_options_sort(GroupListerOptionsSort::Desc, &mut s);
    append_group_lister_options_sort(GroupListerOptionsSort::Asc, &mut s);
    append_group_lister_options_sort(GroupListerOptionsSort::Desc, &mut s);

    assert_eq!(s, expected_string);
}


#[test]
fn groups_build_query_default() {
    let expected_string = "groups";
    let listing: GroupListing = Default::default();
    let query = listing.build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups";
    let listing = GroupListing { options: Default::default() };
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_skip_groups() {
    let expected_string = "groups?skip_groups[]=1&skip_groups[]=2&skip_groups[]=3";
    let listing = GroupListing {
        options: GroupListerOptions {
            skip_groups: Some(vec![1,2,3]),
            ..Default::default()
        },
    };
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_all_available() {
    let expected_string = "groups?all_available=true";
    let listing = GroupListing {
        options: GroupListerOptions {
            all_available: Some(true),
            ..Default::default()
        },
    };
    let query = listing.build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups?all_available=false";
    let listing = GroupListing {
        options: GroupListerOptions {
            all_available: Some(false),
            ..Default::default()
        },
    };
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_search() {
    let expected_string = "groups?search=SearchPattern";
    let listing = GroupListing {
        options: GroupListerOptions {
            search: Some(String::from("SearchPattern")),
            ..Default::default()
        },
    };
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_order_by_name() {
    let expected_string = "groups?order_by=name";
    let listing = GroupListing {
        options: GroupListerOptions {
            order_by: Some(GroupListerOptionsOrderBy::Name),
            ..Default::default()
        },
    };
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_order_by_path() {
    let expected_string = "groups?order_by=path";
    let listing = GroupListing {
        options: GroupListerOptions {
            order_by: Some(GroupListerOptionsOrderBy::Path),
            ..Default::default()
        },
    };
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_sort() {
    let expected_string = "groups?sort=asc";
    let listing = GroupListing {
        options: GroupListerOptions {
            sort: Some(GroupListerOptionsSort::Asc),
            ..Default::default()
        },
    };
    let query = listing.build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups?sort=desc";
    let listing = GroupListing {
        options: GroupListerOptions {
            sort: Some(GroupListerOptionsSort::Desc),
            ..Default::default()
        },
    };
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}



#[test]
fn groups_build_query_search_order_by_path() {
    let expected_string = "groups?search=SearchPattern&order_by=path";
    let listing = GroupListing {
        options: GroupListerOptions {
            order_by: Some(GroupListerOptionsOrderBy::Path),
            search: Some(String::from("SearchPattern")),
            ..Default::default()
        },
    };
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}
