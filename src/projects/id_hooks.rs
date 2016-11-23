//! List project hooks
//!
//! https://docs.gitlab.com/ce/api/projects.html#list-project-hooks
//!
//! # List project hooks
//!
//! Get a list of project hooks.
//!
//! ```
//! GET /projects/ID/hooks
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id` | integer/string | yes | The ID of the project or NAMESPACE/PROJECT_NAME |
