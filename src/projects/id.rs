//! Get project by id.
//!
//! https://docs.gitlab.com/ce/api/projects.html#get-single-project
//!
//! # Get single project
//!
//! Get a specific project, identified by project ID or NAMESPACE/PROJECT_NAME, which is owned by
//! the authenticated user.
//! If using namespaced projects call make sure that the NAMESPACE/PROJECT_NAME is URL-encoded,
//! eg. `/api/v3/projects/diaspora%2Fdiaspora` (where `/` is represented by `%2F`).
//!
//! ```text
//! GET /projects/ID
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id` | integer/string | yes | The ID or NAMESPACE/PROJECT_NAME of the project |


use BuildQuery;



#[derive(Debug, Clone)]
pub enum ListingId {
    Id(i64),
    NamespaceProject(String),
}


#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// The ID or NAMESPACE/PROJECT_NAME of the project
    id:  Option<ListingId>,
}


impl Listing {
    pub fn new() -> Listing {
        Default::default()
    }
    pub fn id(&mut self, id: ListingId) -> &mut Listing {
        self.id = Some(id);
        self
    }
}


impl BuildQuery for Listing {
    fn build_query(&self) -> String {

        let mut query = String::from("projects");

        let amp_char = "&";
        let none_char = "";
        let mut split_char = &none_char;

        // Append a "?" only if at least one of the `Option` is `Some(_)` or if
        // strings contain something.
        query.push_str(match &self.id {
            &None => "",
            _ => "?",
        });

        self.id.clone().map(|id| {
            query.push_str(split_char);
            split_char = &amp_char;

            query.push_str("id=");
            query.push_str(&match id {
                ListingId::Id(id) => id.to_string(),
                ListingId::NamespaceProject(s) => s.replace("/", "%2F"),
            });
        });

        query
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use BuildQuery;


    #[test]
    fn projects_build_query_default() {
        let expected_string = "projects";
        let listing: Listing = Default::default();
        let query = listing.build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects";
        let listing = Listing::new();
        let query = listing.build_query();
        assert_eq!(query, expected_string);
    }


    #[test]
    fn projects_build_query_id() {
        let expected_string = "projects?id=123";
        let query = Listing::new().id(ListingId::Id(123)).build_query();
        assert_eq!(query, expected_string);

        let expected_string = "projects?id=group%2Fproject";
        let query = Listing::new().id(ListingId::NamespaceProject("group/project".to_string())).build_query();
        assert_eq!(query, expected_string);
    }
}
