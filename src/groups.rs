
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
enum GroupListerOptionsOrderBy {
    Name,
    Path,
}

// impl Default for GroupListerOptionsOrderBy {
//     fn default() -> GroupListerOptionsOrderBy { GroupListerOptionsOrderBy::Name }
// }

#[derive(Debug, Copy, Clone)]
enum GroupListerOptionsSort {
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
struct GroupListerOptions {
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
    options: GroupListerOptions,
}


impl BuildQuery for GroupListing {
    fn build_query(&self) -> String {
        let options = &self.options;

        let mut query = String::from("groups");

        // Append a "?", only if one of the `Option` is `Some(_)`
        query.push_str(match (&options.skip_groups,
                              &options.all_available,
                              &options.search,
                              &options.order_by,
                              &options.sort) {
            (&None, &None, &None, &None, &None) => "",
            _ => "?",
        });

        options.all_available.map(|all_available| {
            if all_available {
                query.push_str("all_available=true")
            } else {
                query.push_str("all_available=false")
            }
        });

        options.order_by.map(|order_by| {
            query.push_str("order_by=");
            append_group_lister_options_order_by(order_by, &mut query);
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
fn test_group_lister_build_query() {
    // use BuildQuery;
    // use Groups;

    let expected_string = "groups";
    let listing: GroupListing = Default::default();
    let query = listing.build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups";
    let listing = GroupListing { options: Default::default() };
    let query = listing.build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups?order_by=name";
    let listing = GroupListing {
        options: GroupListerOptions { order_by: Some(GroupListerOptionsOrderBy::Name), ..Default::default() },
    };
    println!("listing: {:?}", listing);
    let query = listing.build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups?order_by=path";
    let listing = GroupListing {
        options: GroupListerOptions { order_by: Some(GroupListerOptionsOrderBy::Path), ..Default::default() },
    };
    println!("listing: {:?}", listing);
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}
