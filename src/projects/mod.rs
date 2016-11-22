
use BuildQuery;


// https://docs.gitlab.com/ce/api/projects.html

// Get a list of projects for which the authenticated user is a member.
// GET /projects
//
// Get a list of projects which the authenticated user can see.
// GET /projects/visible
//
// Get a list of projects which are owned by the authenticated user.
// GET /projects/owned
//
// Get a list of projects which are starred by the authenticated user.
// GET /projects/starred
//
// Get a list of all GitLab projects (admin only).
// GET /projects/all
//
// Get a specific project, identified by project `ID` or `NAMESPACE/PROJECT_NAME`, which is owned by the authenticated user. If using namespaced projects call make sure that the `NAMESPACE/PROJECT_NAME` is URL-encoded, eg. `/api/v3/projects/diaspora%2Fdiaspora` (where `/` is represented by `%2F`).
// GET /projects/:id
//
// Get the events for the specified project. Sorted from newest to oldest
// GET /projects/:id/events
//
// Creates a new project owned by the authenticated user.
// POST /projects
//
// Creates a new project owned by the specified user. Available only for admins.
// POST /projects/user/:user_id
//
// Updates an existing project
// PUT /projects/:id
//
// Forks a project into the user namespace of the authenticated user or the one provided.
// POST /projects/fork/:id
//
// Stars a given project. Returns status code `201` and the project on success and `304` if the project is already starred.
// POST /projects/:id/star
//
// Unstars a given project. Returns status code `200` and the project on success and `304` if the project is not starred.
// DELETE /projects/:id/star
//
// Archives the project if the user is either admin or the project owner of this project. This action is idempotent, thus archiving an already archived project will not change the project.
//
// Status code `201` with the project as body is given when successful, in case the user doesn't have the proper access rights, code `403` is returned. Status `404` is returned if the project doesn't exist, or is hidden to the user.
// POST /projects/:id/archive
//
//
// Unarchives the project if the user is either admin or the project owner of this project. This action is idempotent, thus unarchiving an non-archived project will not change the project.
//
// Status code `201` with the project as body is given when successful, in case the user doesn't have the proper access rights, code `403` is returned. Status `404` is returned if the project doesn't exist, or is hidden to the user.
// POST /projects/:id/unarchive
//
//
// Removes a project including all associated resources (issues, merge requests etc.)
// DELETE /projects/:id
//
//
// Uploads a file to the specified project to be used in an issue or merge request description, or a comment.
// POST /projects/:id/uploads
//
//
// Allow to share project with group.
// POST /projects/:id/share
//
//
// Get a list of project hooks.
// GET /projects/:id/hooks
//
//
// Get a specific hook for a project.
// GET /projects/:id/hooks/:hook_id
//
//
// Adds a hook to a specified project.
// POST /projects/:id/hooks
//
//
// Edits a hook for a specified project.
// PUT /projects/:id/hooks/:hook_id
//
//
// Removes a hook from a project. This is an idempotent method and can be called multiple times. Either the hook is available or not.
// DELETE /projects/:id/hooks/:hook_id
//
//
// Lists all branches of a project.
// GET /projects/:id/repository/branches
//
//
// A specific branch of a project.
// GET /projects/:id/repository/branches/:branch
//
//
// Protects a single branch of a project.
// PUT /projects/:id/repository/branches/:branch/protect
//
//
// Unprotects a single branch of a project.
// PUT /projects/:id/repository/branches/:branch/unprotect
//
//
// Create a forked from/to relation between existing projects.
// POST /projects/:id/fork/:forked_from_id
//
//
// Delete an existing forked from relationship
// DELETE /projects/:id/fork
//
//
// Search for projects by name which are accessible to the authenticated user.
// GET /projects/search/:query






// https://docs.gitlab.com/ce/api/projects.html#list-projects


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


#[derive(Debug, Copy, Clone)]
pub enum ListingSort {
    Asc,
    Desc,
}


#[derive(Default, Debug, Clone)]
pub struct Listing {
    /// Limit by archived status
    archived:  Option<bool>,
    /// Limit by visibility.
    visibility: Option<ListingVisibility>,
    /// Return requests ordered by. Default is `ListingOrderBy::CreatedAt`.
    order_by:  Option<ListingOrderBy>,
    /// Return requests sorted. Default is `ListingSort::Desc`.
    sort: Option<ListingSort>,
    /// Return list of authorized projects matching the search criteria.
    search:  String,
    /// Return only the ID, URL, name, and path of each project
    simple: Option<bool>,
}
