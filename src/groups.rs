
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
pub struct GroupLister {
    // pagination: Pagination,

}


enum GroupListerOptionsOrderBy {
    Name,
    Path,
}

impl Default for GroupListerOptionsOrderBy {
    fn default() -> GroupListerOptionsOrderBy { GroupListerOptionsOrderBy::Name }
}

enum GroupListerOptionsSort {
    Asc,
    Desc,
}

impl Default for GroupListerOptionsSort {
    fn default() -> GroupListerOptionsSort { GroupListerOptionsSort::Asc }
}


fn append_group_lister_options_order_by(order_by: GroupListerOptionsOrderBy, s: &mut String) {
    s.push_str(
        match order_by {
            GroupListerOptionsOrderBy::Name => "name",
            GroupListerOptionsOrderBy::Path => "path",
        }
    );
}


fn append_group_lister_options_sort(order_by: GroupListerOptionsSort, s: &mut String) {
    s.push_str(
        match order_by {
            GroupListerOptionsSort::Asc => "asc",
            GroupListerOptionsSort::Desc => "desc",
        }
    );
}

/// https://docs.gitlab.com/ce/api/groups.html#list-groups
#[derive(Default)]
struct GroupListerOptions {
    /// Skip the group IDs passes
    skip_groups: Option<Vec<i64>>,
    /// Show all the groups you have access to
    all_available: Option<bool>,
    /// Return list of authorized groups matching the search criteria
    search: Option<String>,
    /// Order groups by `name` or `path`. Default is `name`
    order_by: GroupListerOptionsOrderBy,
    /// Order groups in `asc` or `desc` order. Default is `asc`
    sort: GroupListerOptionsSort,

}


// #[derive(Default)]
pub struct GroupListing {
    options: GroupListerOptions,
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
