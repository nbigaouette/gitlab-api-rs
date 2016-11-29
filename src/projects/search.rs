//! Search for projects by name
//!
//! https://docs.gitlab.com/ce/api/projects.html#search-for-projects-by-name
//!
//! # Search for projects by name
//!
//! Search for projects by name which are accessible to the authenticated user.
//!
//! ```text
//! GET /projects/search/QUERY
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `query` | string | yes | A string contained in the project name |
//! | `order_by` | string | no | Return requests ordered by `id`, `name`, `created_at` or `last_activity_at` fields |
//! | `sort` | string | no | Return requests sorted in `asc` or `desc` order |


use BuildQuery;

use serde_json;

use gitlab::GitLab;
use Projects;


impl GitLab {
    pub fn projects_search(&self, listing: Listing) -> Result<Projects, serde_json::Error> {
        let query = listing.build_query();
        self.get(&query)
    }
}


#[derive(Debug, Copy, Clone)]
pub enum ListingOrderBy {
    Id,
    Name,
    // Path,
    CreatedAt,
    // UpdatedAt,
    LastActivityAt,
}


#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// A string contained in the project name.
    query: String,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
}


#[allow(dead_code)]
impl Listing {
    pub fn new(query: String) -> Listing {
        Listing { query: query, ..Default::default() }
    }
    pub fn order_by(&mut self, order_by: ListingOrderBy) -> &mut Listing {
        self.order_by = Some(order_by);
        self
    }
    fn sort(&mut self, sort: ::ListingSort) -> &mut Listing {
        self.sort = Some(sort);
        self
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {

        let mut query = format!("projects/search/{}", self.query);

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?" only if at least one of the `Option` is `Some(_)` or if
        // strings contain something.
        query.push_str(match (&self.order_by, &self.sort) {
            (&None, &None) => "",
            _ => "?",
        });

        self.order_by.map(|order_by| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("order_by=");
            query.push_str(match order_by {
                ListingOrderBy::Id => "id",
                ListingOrderBy::Name => "name",
                // ListingOrderBy::Path => "path",
                ListingOrderBy::CreatedAt => "created_at",
                // ListingOrderBy::UpdatedAt => "updated_at",
                ListingOrderBy::LastActivityAt => "last_activity_at",
            });
        });

        self.sort.map(|sort| {
            query.push_str(split_char);
            // split_char = &amp_char;

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
    use super::*;
    use BuildQuery;

    const TEST_SEARCH_QUERY: &'static str = "SearchPattern";


    #[test]
    fn build_query_default() {
        let expected_string = format!("projects/search/{}", TEST_SEARCH_QUERY);
        let listing = Listing::new(TEST_SEARCH_QUERY.to_string());
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let expected_string = format!("projects/search/{}?order_by=id", TEST_SEARCH_QUERY);
        let query =
            Listing::new(TEST_SEARCH_QUERY.to_string()).order_by(ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/search/{}?order_by=name", TEST_SEARCH_QUERY);
        let query = Listing::new(TEST_SEARCH_QUERY.to_string())
            .order_by(ListingOrderBy::Name)
            .build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/search/{}?order_by=created_at", TEST_SEARCH_QUERY);
        let query = Listing::new(TEST_SEARCH_QUERY.to_string())
            .order_by(ListingOrderBy::CreatedAt)
            .build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/search/{}?order_by=last_activity_at",
                                      TEST_SEARCH_QUERY);
        let query = Listing::new(TEST_SEARCH_QUERY.to_string())
            .order_by(ListingOrderBy::LastActivityAt)
            .build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let expected_string = format!("projects/search/{}?sort=asc", TEST_SEARCH_QUERY);
        let query =
            Listing::new(TEST_SEARCH_QUERY.to_string()).sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = format!("projects/search/{}?sort=desc", TEST_SEARCH_QUERY);
        let query =
            Listing::new(TEST_SEARCH_QUERY.to_string()).sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_multiple() {
        let expected_string = format!("projects/search/{}?order_by=created_at&sort=desc",
                                      TEST_SEARCH_QUERY);
        let query = Listing::new(TEST_SEARCH_QUERY.to_string())
            .order_by(ListingOrderBy::CreatedAt)
            .sort(::ListingSort::Desc)
            .build_query();
        assert_eq!(query, expected_string);
    }
}
