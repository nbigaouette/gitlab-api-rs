//! List a group's projects
//!
//! https://docs.gitlab.com/ce/api/groups.html#list-a-group-s-projects
//!
//! # List a group's projects
//!
//! Get a list of projects in this group.
//!
//! ```text
//! GET /groups/ID/projects
//! ```
//!
//! Parameters:
//!
//! - `archived` (optional) - if passed, limit by archived status
//! - `visibility` (optional) - if passed, limit by visibility `public`, `internal`, `private`
//! - `order_by` (optional) - Return requests ordered by `id`, `name`, `path`, `created_at`,
//!     `updated_at` or `last_activity_at` fields. Default is `created_at`
//! - `sort` (optional) - Return requests sorted in `asc` or `desc` order. Default is `desc`
//! - `search` (optional) - Return list of authorized projects according to a search criteria
//! - `ci_enabled_first` - Return projects ordered by ci_enabled flag. Projects with enabled
//!     GitLab CI go first
//!
//!


use BuildQuery;


// FIXME: Use a type for the project id.

#[derive(Debug, Copy, Clone)]
pub enum ListingVisibility {
    Public,
    Internal,
    Private,
}


#[derive(Debug, Copy, Clone)]
pub enum ListingOrderBy {
    Id,
    Name,
    Path,
    CreatedAt,
    UpdatedAt,
    LastActivityAt,
}


#[derive(Debug, Copy, Clone)]
pub enum ListingSort {
    Asc,
    Desc,
}


#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// Group Id.
    id: i64,
    /// Limit by archived status.
    archived: Option<bool>,
    /// Limit by visibility
    visibility: Option<ListingVisibility>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `ListingSort::Desc`.
    sort: Option<ListingSort>,
    /// Return list of authorized projects according to a search criteria.
    search: String,
    /// Return projects ordered by `ci_enabled` flag. Projects with enabled GitLab CI go first.
    ci_enabled_first: Option<bool>,
}


impl Listing {
    pub fn new(id: i64) -> Listing {
        Listing { id: id, ..Default::default() }
    }
    pub fn archived(&mut self, archived: bool) -> &mut Listing {
        self.archived = Some(archived);
        self
    }
    pub fn visibility(&mut self, visibility: ListingVisibility) -> &mut Listing {
        self.visibility = Some(visibility);
        self
    }
    pub fn order_by(&mut self, order_by: ListingOrderBy) -> &mut Listing {
        self.order_by = Some(order_by);
        self
    }
    pub fn sort(&mut self, sort: ListingSort) -> &mut Listing {
        self.sort = Some(sort);
        self
    }
    pub fn search(&mut self, search: String) -> &mut Listing {
        self.search = search;
        self
    }
    pub fn ci_enabled_first(&mut self, ci_enabled_first: bool) -> &mut Listing {
        self.ci_enabled_first = Some(ci_enabled_first);
        self
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {
        let mut query = format!("groups/{}/projects", self.id);

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?", only if one of the `Option` is `Some(_)`
        query.push_str(match (&self.archived,
                              &self.visibility,
                              &self.order_by,
                              &self.sort,
                              self.search.is_empty(),
                              &self.ci_enabled_first) {
            (&None, &None, &None, &None, true, &None) => "",
            _ => "?",
        });

        self.archived.map(|archived| {
            query.push_str(split_char);
            split_char = &amp_char;

            if archived {
                query.push_str("archived=true")
            } else {
                query.push_str("archived=false")
            }
        });

        self.visibility.map(|visibility| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("visibility=");
            query.push_str(match visibility {
                ListingVisibility::Public => "public",
                ListingVisibility::Internal => "internal",
                ListingVisibility::Private => "private",
            });
        });

        self.order_by.map(|order_by| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            query.push_str(match order_by {
                ListingOrderBy::Id => "id",
                ListingOrderBy::Name => "name",
                ListingOrderBy::Path => "path",
                ListingOrderBy::CreatedAt => "created_at",
                ListingOrderBy::UpdatedAt => "updated_at",
                ListingOrderBy::LastActivityAt => "last_activity_at",
            });
        });

        self.sort.map(|sort| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("sort=");
            query.push_str(match sort {
                ListingSort::Asc => "asc",
                ListingSort::Desc => "desc",
            });
        });

        if !self.search.is_empty() {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("search=");
            query.push_str(&self.search);
        }

        self.ci_enabled_first.map(|ci_enabled_first| {
            query.push_str(split_char);
            split_char = &amp_char;

            if ci_enabled_first {
                query.push_str("ci_enabled_first=true")
            } else {
                query.push_str("ci_enabled_first=false")
            }
        });

        query
    }
}

// GET /groups/:id/projects


#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;

    const TEST_PROJECT_ID: i64 = 123;


    #[test]
    fn groups_build_query_default() {
        let expected_string = format!("groups/{}/projects", TEST_PROJECT_ID);
        let listing = Listing::new(TEST_PROJECT_ID);
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_archived() {
        let expected_string = format!("groups/{}/projects?archived=true", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone()).archived(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?archived=false", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone()).archived(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_visibility() {
        let expected_string = format!("groups/{}/projects?visibility=public", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone())
            .visibility(ListingVisibility::Public)
            .build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?visibility=internal", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone())
            .visibility(ListingVisibility::Internal)
            .build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?visibility=private", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone())
            .visibility(ListingVisibility::Private)
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_order_by() {
        let expected_string = format!("groups/{}/projects?order_by=id", TEST_PROJECT_ID);
        let query =
            Listing::new(TEST_PROJECT_ID.clone()).order_by(ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=name", TEST_PROJECT_ID);
        let query =
            Listing::new(TEST_PROJECT_ID.clone()).order_by(ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=path", TEST_PROJECT_ID);
        let query =
            Listing::new(TEST_PROJECT_ID.clone()).order_by(ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=created_at", TEST_PROJECT_ID);
        let query =
            Listing::new(TEST_PROJECT_ID.clone()).order_by(ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=updated_at", TEST_PROJECT_ID);
        let query =
            Listing::new(TEST_PROJECT_ID.clone()).order_by(ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?order_by=last_activity_at",
                                      TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone())
            .order_by(ListingOrderBy::LastActivityAt)
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_sort() {
        let expected_string = format!("groups/{}/projects?sort=asc", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone()).sort(ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?sort=desc", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone()).sort(ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_search() {
        let expected_string = format!("groups/{}/projects?search=SearchPattern", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone())
            .search(String::from("SearchPattern"))
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_ci_enabled_first() {
        let expected_string = format!("groups/{}/projects?ci_enabled_first=true", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone()).ci_enabled_first(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("groups/{}/projects?ci_enabled_first=false", TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone()).ci_enabled_first(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_multiple() {
        let expected_string = format!("groups/{}/projects?archived=true&ci_enabled_first=true",
                                      TEST_PROJECT_ID);
        let query = Listing::new(TEST_PROJECT_ID.clone())
            .archived(true)
            .ci_enabled_first(true)
            .build_query();
        assert_eq!(query, expected_string);
    }
}
