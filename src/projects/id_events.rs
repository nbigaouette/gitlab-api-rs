//! Get project events
//!
//! https://docs.gitlab.com/ce/api/projects.html#get-project-events
//!
//! # Get project events
//!
//! Get the events for the specified project.
//! Sorted from newest to oldest
//!
//! ```
//! GET /projects/ID/events
//! ```
//!
//! Parameters:
//!
//! | Attribute | Type | Required | Description |
//! | --------- | ---- | -------- | ----------- |
//! | `id` | integer/string | yes | The ID or NAMESPACE/PROJECT_NAME of the project |
//!
//!
