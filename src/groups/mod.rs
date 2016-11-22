
use BuildQuery;

pub mod owned_groups;


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
pub enum ListingOrderBy {
    Name,
    Path,
}


#[derive(Debug, Copy, Clone)]
pub enum ListingSort {
    Asc,
    Desc,
}


fn append_group_lister_options_order_by(order_by: ListingOrderBy, s: &mut String) {
    s.push_str(match order_by {
        ListingOrderBy::Name => "name",
        ListingOrderBy::Path => "path",
    });
}


fn append_group_lister_options_sort(order_by: ListingSort, s: &mut String) {
    s.push_str(match order_by {
        ListingSort::Asc => "asc",
        ListingSort::Desc => "desc",
    });
}


/// https://docs.gitlab.com/ce/api/groups.html#list-groups
#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// Skip the group IDs passes
    skip_groups: Vec<i64>,
    /// Show all the groups you have access to
    all_available: Option<bool>,
    /// Return list of authorized groups matching the search criteria
    search: String,
    /// Order groups by `name` or `path`. Default is `name`
    order_by: Option<ListingOrderBy>,
    /// Order groups in `asc` or `desc` order. Default is `asc`
    sort: Option<ListingSort>,
}


impl Listing {
    pub fn new() -> Listing {
        Default::default()
    }
    pub fn skip_groups(&mut self, skip_groups: Vec<i64>) -> &mut Listing {
        self.skip_groups = skip_groups;
        self
    }
    pub fn all_available(&mut self, all_available: bool) -> &mut Listing {
        self.all_available = Some(all_available);
        self
    }
    pub fn search(&mut self, search: String) -> &mut Listing {
        self.search = search;
        self
    }
    pub fn order_by(&mut self, order_by: ListingOrderBy) -> &mut Listing {
        self.order_by = Some(order_by);
        self
    }
    fn sort(&mut self, sort: ListingSort) -> &mut Listing {
        self.sort = Some(sort);
        self
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {

        let mut query = String::from("groups");

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?", only if one of the `Option` is `Some(_)`
        query.push_str(match (self.skip_groups.is_empty(),
                              &self.all_available,
                              self.search.is_empty(),
                              &self.order_by,
                              &self.sort) {
            (true, &None, true, &None, &None) => "",
            _ => "?",
        });

        if !self.skip_groups.is_empty() {
            query.push_str(split_char);
            split_char = &amp_char;

            let mut array_split_char = &none_char;
            for skip_group in &self.skip_groups {
                query.push_str(array_split_char);
                query.push_str("skip_groups[]=");
                query.push_str(&skip_group.to_string());
                array_split_char = &amp_char;
            }
        }

        self.all_available.map(|all_available| {
            query.push_str(split_char);
            split_char = &amp_char;

            if all_available {
                query.push_str("all_available=true")
            } else {
                query.push_str("all_available=false")
            }
        });

        if !self.search.is_empty() {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("search=");
            query.push_str(&self.search);
        }

        self.order_by.map(|order_by| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            append_group_lister_options_order_by(order_by, &mut query);
        });

        self.sort.map(|sort| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("sort=");
            append_group_lister_options_sort(sort, &mut query);
        });

        query
    }
}


#[test]
fn test_append_group_lister_options_order_by() {
    let expected_string = "Initialnamepathpathname";

    let mut s = String::from("Initial");

    append_group_lister_options_order_by(ListingOrderBy::Name, &mut s);
    append_group_lister_options_order_by(ListingOrderBy::Path, &mut s);
    append_group_lister_options_order_by(ListingOrderBy::Path, &mut s);
    append_group_lister_options_order_by(ListingOrderBy::Name, &mut s);

    assert_eq!(s, expected_string);
}


#[test]
fn test_append_group_lister_options_sort() {
    let expected_string = "Initialascdescascdesc";

    let mut s = String::from("Initial");

    append_group_lister_options_sort(ListingSort::Asc, &mut s);
    append_group_lister_options_sort(ListingSort::Desc, &mut s);
    append_group_lister_options_sort(ListingSort::Asc, &mut s);
    append_group_lister_options_sort(ListingSort::Desc, &mut s);

    assert_eq!(s, expected_string);
}


#[test]
fn groups_build_query_default() {
    let expected_string = "groups";
    let listing: Listing = Default::default();
    let query = listing.build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups";
    let listing = Listing::new();
    let query = listing.build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_skip_groups() {
    let expected_string = "groups?skip_groups[]=1&skip_groups[]=2&skip_groups[]=3";
    let query = Listing::new().skip_groups(vec![1, 2, 3]).build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_all_available() {
    let expected_string = "groups?all_available=true";
    let query = Listing::new().all_available(true).build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups?all_available=false";
    let query = Listing::new().all_available(false).build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_search() {
    let expected_string = "groups?search=SearchPattern";
    let query = Listing::new().search(String::from("SearchPattern")).build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_order_by_name() {
    let expected_string = "groups?order_by=name";
    let query = Listing::new().order_by(ListingOrderBy::Name).build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_order_by_path() {
    let expected_string = "groups?order_by=path";
    let query = Listing::new().order_by(ListingOrderBy::Path).build_query();
    assert_eq!(query, expected_string);
}


#[test]
fn groups_build_query_sort() {
    let expected_string = "groups?sort=asc";
    let query = Listing::new().sort(ListingSort::Asc).build_query();
    assert_eq!(query, expected_string);

    let expected_string = "groups?sort=desc";
    let query = Listing::new().sort(ListingSort::Desc).build_query();
    assert_eq!(query, expected_string);
}



#[test]
fn groups_build_query_search_order_by_path() {
    let expected_string = "groups?search=SearchPattern&order_by=path";
    let query = Listing::new()
        .order_by(ListingOrderBy::Path)
        .search(String::from("SearchPattern"))
        .build_query();
    assert_eq!(query, expected_string);
    let query = Listing::new()
        .search(String::from("SearchPattern"))
        .order_by(ListingOrderBy::Path)
        .build_query();
    assert_eq!(query, expected_string);
}
