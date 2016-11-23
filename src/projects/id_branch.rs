//! Single branch
//!
//! https://docs.gitlab.com/ce/api/projects.html#single-branch
//!
//! # Single branch
//!
//! A specific branch of a project.
//!
//! ```
//! GET /projects/ID/repository/branches/BRANCH
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id` | integer/string | yes | The ID of the project or NAMESPACE/PROJECT_NAME |
//! | `branch` | string | yes | The name of the branch |
//! | `developers_can_push` | boolean | no | Flag if developers can push to the branch |
//! | `developers_can_merge` | boolean | no | Flag if developers can merge to the branch |
