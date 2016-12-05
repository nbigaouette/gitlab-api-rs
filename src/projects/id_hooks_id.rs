//! Get project hook
//!
//! https://docs.gitlab.com/ce/api/projects.html#get-project-hook
//!
//! # Get project hook
//!
//! Get a specific hook for a project.
//!
//! ```text
//! GET /projects/ID/hooks/HOOK_ID
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id` | integer/string | yes | The ID of the project or `NAMESPACE/PROJECT_NAME` |
//! | `hook_id` | integer | yes | The ID of a project hook |
//!
//!
