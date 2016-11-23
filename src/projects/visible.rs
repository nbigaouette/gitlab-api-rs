//! Get a list of projects which the authenticated user can see.
//!
//! https://docs.gitlab.com/ce/api/projects.html#list-projects
//!
//! # Get a list of projects which the authenticated user can see.
//!
//! ```text
//! GET /projects/visible
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
//! | `simple` | boolean | no | Return only the ID, URL, name, and path of each project |
//!
//!
