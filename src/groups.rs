
use Groups;

// https://docs.gitlab.com/ce/api/groups.html
// List groups:                 GET /groups
// List owned groups:           GET /groups/owned
// List a group's projects:     GET /groups/:id/projects
// Details of a group:          GET /groups/:id
// New group:                   POST /groups
// Transfer project to group:   POST  /groups/:id/projects/:project_id
// Update group:                PUT /groups/:id
// Remove group:                DELETE /groups/:id
// Search for group:            GET /groups?search=foobar


pub struct GroupListing {

}


impl GroupListing {
    fn list(&self) -> Groups {
        let groups: Groups = vec![];

        groups
    }
}
