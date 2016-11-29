//! List ALL projects
//!
//! https://docs.gitlab.com/ce/api/projects.html#list-all-projects
//!
//! # List ALL projects
//!
//! Get a list of all GitLab projects (admin only).
//!
//! ```text
//! GET /projects/all
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `archived` | boolean | no | Limit by archived status |
//! | `visibility` | string | no | Limit by visibility `public`, `internal`, or `private` |
//! | `order_by` | string | no | Return projects ordered by `id`, `name`, `path`, `created_at`, `updated_at`, or `last_activity_at` fields. Default is `created_at` |
//! | `sort` | string | no | Return projects sorted in `asc` or `desc` order. Default is `desc` |
//! | `search` | string | no | Return list of authorized projects matching the search criteria |


use BuildQuery;

use serde_json;

use gitlab::GitLab;
use Projects;


impl GitLab {
    pub fn projects_all(&self, listing: Listing) -> Result<Projects, serde_json::Error> {
        let query = listing.build_query();
        self.get(&query)
    }
}


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


#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// Limit by archived status
    archived: Option<bool>,
    /// Limit by visibility.
    visibility: Option<ListingVisibility>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by: Option<ListingOrderBy>,
    /// Return requests sorted. Default is `::ListingSort::Desc`.
    sort: Option<::ListingSort>,
    /// Return list of authorized projects matching the search criteria.
    search: String,
}


#[allow(dead_code)]
impl Listing {
    pub fn new() -> Listing {
        Default::default()
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
    fn sort(&mut self, sort: ::ListingSort) -> &mut Listing {
        self.sort = Some(sort);
        self
    }
    pub fn search(&mut self, search: String) -> &mut Listing {
        self.search = search;
        self
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {

        let mut query = String::from("projects/all");

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?" only if at least one of the `Option` is `Some(_)` or if
        // strings contain something.
        query.push_str(match (&self.archived,
                              &self.visibility,
                              &self.order_by,
                              &self.sort,
                              self.search.is_empty()) {
            (&None, &None, &None, &None, true) => "",
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
                ::ListingSort::Asc => "asc",
                ::ListingSort::Desc => "desc",
            });
        });

        if !self.search.is_empty() {
            query.push_str(split_char);
            // split_char = &amp_char;

            query.push_str("search=");
            query.push_str(&self.search);
        }

        query
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;


    #[test]
    fn build_query_default() {
        let expected_string = "projects/all";
        let listing: Listing = Default::default();
        let query = listing.build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all";
        let listing = Listing::new();
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_archived() {
        let expected_string = "projects/all?archived=true";
        let query = Listing::new().archived(true).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?archived=false";
        let query = Listing::new().archived(false).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_visibility() {
        let expected_string = "projects/all?visibility=public";
        let query = Listing::new().visibility(ListingVisibility::Public).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?visibility=internal";
        let query = Listing::new().visibility(ListingVisibility::Internal).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?visibility=private";
        let query = Listing::new().visibility(ListingVisibility::Private).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_order_by() {
        let expected_string = "projects/all?order_by=id";
        let query = Listing::new().order_by(ListingOrderBy::Id).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=name";
        let query = Listing::new().order_by(ListingOrderBy::Name).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=path";
        let query = Listing::new().order_by(ListingOrderBy::Path).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=created_at";
        let query = Listing::new().order_by(ListingOrderBy::CreatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=updated_at";
        let query = Listing::new().order_by(ListingOrderBy::UpdatedAt).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?order_by=last_activity_at";
        let query = Listing::new().order_by(ListingOrderBy::LastActivityAt).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_sort() {
        let expected_string = "projects/all?sort=asc";
        let query = Listing::new().sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects/all?sort=desc";
        let query = Listing::new().sort(::ListingSort::Desc).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn build_query_search() {
        let expected_string = "projects/all?search=SearchPattern";
        let query = Listing::new().search(String::from("SearchPattern")).build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn groups_build_query_multiple() {
        let expected_string = "projects/all?archived=false&sort=asc";
        let query = Listing::new().archived(false).sort(::ListingSort::Asc).build_query();
        assert_eq!(query, expected_string);
    }
}
