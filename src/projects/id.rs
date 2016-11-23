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
//! ```
//! GET /projects/:id
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id` | integer/string | yes | The ID or NAMESPACE/PROJECT_NAME of the project |
