//! Search for projects by name
//!
//! https://docs.gitlab.com/ce/api/projects.html#search-for-projects-by-name
//!
//! # Search for projects by name
//!
//! Search for projects by name which are accessible to the authenticated user.
//!
//! ```
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
