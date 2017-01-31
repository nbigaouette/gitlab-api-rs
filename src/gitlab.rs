
use std::io::Read;  // Trait providing read_to_string()
use std;

use url;
use hyper;
use serde;
use serde_json;
use regex;


// use Groups;
use Lister;

use ::errors::*;


pub const API_VERSION: u16 = 3;




pub struct GitLab {
    url: url::Url,
    private_token: String,
    client: hyper::Client,
}


// Explicitly implement Debug trait for GitLab so we can hide the token.
impl std::fmt::Debug for GitLab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "GitLab {{ scheme: {}, domain: {}, port: {}, private_token: XXXXXXXXXXXXXXXXXXXX }}",
               self.url.scheme(),
               self.url.domain().unwrap_or("bad hostname provided"),
               self.url
                   .port()
                   .map(|port_u16| port_u16.to_string())
                   .unwrap_or("no port provided".to_string()))
    }
}

fn validate_url(scheme: &str, domain: &str, port: u16) -> Result<url::Url> {

    match domain.find('.') {
        None => {
            // pass
        }
        Some(index) => {
            if index == 0 {
                bail!(format!("invalid domain: '{}' cannot start with a dot", domain));
            }
        }
    };

    if domain.ends_with('.') {
        bail!(format!("invalid domain: '{}' cannot end with a dot", domain));
    }

    let url_string = format!("{}://{}/api/v{}/", scheme, domain, API_VERSION);
    let mut url = url::Url::parse(&url_string)
        .chain_err(|| format!("failure to parse URL '{}'", url_string))?;
    url.set_port(Some(port)).expect("bad port provided");

    {
        let url_host = url.host_str();
        if url_host.is_none() {
            bail!("failure to get URL's hostname");
        }
        if url_host.unwrap() != domain {
            bail!(format!("invalid hostname '{}'", domain));
        }
    }

    Ok(url)
}

impl GitLab {
    pub fn _new(scheme: &str, domain: &str, port: u16, private_token: &str) -> Result<GitLab> {
        if private_token.len() != 20 {
            bail!(format!("private token should be a 20 characters string (not {})",
                          private_token.len()));
        }

        let url: url::Url = validate_url(scheme, domain, port).chain_err(|| "invalid URL")?;

        Ok(GitLab {
            url: url,
            private_token: private_token.to_string(),
            client: match std::env::var("HTTP_PROXY") {
                Ok(proxy) => {
                    let proxy: Vec<&str> = proxy.trim_left_matches("http://").split(':').collect();
                    let hostname = proxy[0].to_string();
                    let port = proxy[1].parse()
                        .chain_err(|| format!("failure to set port to {}", proxy[1]))?;

                    hyper::Client::with_http_proxy(hostname, port)
                }
                Err(_) => hyper::Client::new(),
            },
        })
    }

    pub fn new_insecure(domain: &str, private_token: &str) -> Result<GitLab> {
        warn!("Using insecure http:// protocol: Token will be sent in clear!");
        GitLab::_new("http", domain, 80, private_token)
    }

    pub fn new(domain: &str, private_token: &str) -> Result<GitLab> {
        GitLab::_new("https", domain, 443, private_token)
    }

    pub fn port(mut self, port: u16) -> Self {
        self.url.set_port(Some(port)).unwrap();
        self
    }

    pub fn scheme(mut self, scheme: &str) -> Self {
        self.url.set_scheme(scheme).unwrap();
        self
    }

    /// Build a URL used to access GitLab instance, including some parameters.
    ///
    /// # Examples
    ///
    /// Example from GitLab: https://docs.gitlab.com/ce/api/#basic-usage
    ///
    /// ```
    /// use gitlab_api::GitLab;
    ///
    /// let expected_url = "https://gitlab.example.com\
    ///                     /api/v3/groups?order_by=path&private_token=XXXXXXXXXXXXXXXXXXXX";
    ///
    /// let gl = GitLab::new("gitlab.example.com", "XXXXXXXXXXXXXXXXXXXX").unwrap();
    ///
    /// assert_eq!(gl.build_url("groups?order_by=path").unwrap(), expected_url);
    /// ```
    pub fn build_url(&self, query: &str) -> Result<String> {
        let mut new_url = self.url
            .clone()
            .join(query)
            .chain_err(|| {
                format!("Failure to join query '{}' to url {}",
                        query,
                        self.url.as_str())
            })?;
        new_url.query_pairs_mut().append_pair("private_token", &self.private_token);

        Ok(new_url.into_string())
    }

    // pub fn attempt_connection(&self) -> Result<hyper::client::Response, hyper::Error> {
    //     let url = self.build_url("version");
    //     // Close connections after each GET.
    //     self.client.get(&url).header(hyper::header::Connection::close()).send()
    // }


    /// Perform an HTTP GET to the GitLab server from a specific query.
    ///
    /// The `query` is simply the string part appearing in the GET URL.
    /// For example, for a `GET https://www.example.com/projects/:id/merge_requests?iid=42`, the
    /// `query` is `projects/:id/merge_requests?iid=42`.
    ///
    /// The method _can_ be paginated if `page` and `per_page` is provided.
    ///
    /// Notes:
    ///
    /// * This method is meant to be used internally;
    /// * Until all `BuildQuery::build_query()`s use `serde_urlencoded`, the `query` paramter will
    ///   have to remain a string.
    ///
    /// Returns a specific GitLab type, wrapped in a `Result`.
    pub fn get<T, U>(&self, query: &str, page: U, per_page: U) -> Result<T>
        where T: serde::Deserialize,
              U: Into<Option<u16>>
    {
        let mut url = self.build_url(query)
            .chain_err(|| format!("failure to build url for query '{}'", query))?;
        info!("url: {:?}", remove_gitlab_token_from_url(&url));

        // Add pagination information if requested.
        page.into().map(|page| url.push_str(&format!("&page={}", page)));
        per_page.into().map(|per_page| url.push_str(&format!("&per_page={}", per_page)));

        // Close connections after each GET.
        let mut res: hyper::client::Response = self.client
            .get(&url)
            .header(hyper::header::Connection::close())
            .send()
            .chain_err(|| format!("cannot send request '{}' to {:?}", query, self))?;
        info!("res.status: {:?}", res.status);
        // The headers might leak the token, don't print them.
        // debug!("res.headers: {:?}", res.headers);
        // Hide the url's token in logger
        debug!("res.url: {}", remove_gitlab_token_from_url(res.url.as_str()));

        let mut body = String::new();
        res.read_to_string(&mut body).chain_err(|| "cannot read response body")?;
        debug!("body:\n{}", body);

        if res.status != hyper::status::StatusCode::Ok {
            bail!(format!("status code '{}', not '200 OK'", res.status));
        }

        serde_json::from_str(body.as_str())
            .chain_err(|| format!("cannot build Rust struct from JSON data: {}", body))
    }

    pub fn version(&self) -> Result<::Version> {
        self.get("version", None, None).chain_err(|| "cannot query 'version'")
    }

    pub fn groups(&self) -> ::groups::GroupsLister {
        ::groups::GroupsLister::new(self)
    }

    pub fn projects(&self) -> ::projects::ProjectsLister {
        ::projects::ProjectsLister::new(self)
    }

    pub fn issues(&self) -> ::issues::IssuesLister {
        ::issues::IssuesLister::new(self)
    }

    pub fn merge_requests(&self, project_id: i64) -> ::merge_requests::MergeRequestsLister {
        ::merge_requests::MergeRequestsLister::new(self, project_id)
    }

    // pub fn groups(&mut self, listing: ::groups::Listing) -> Result<Groups, serde_json::Error> {
    //     let query = listing.build_query();
    //     // self.get(&query)
    //     unimplemented!();
    // }

    // pub fn owned_groups(&mut self) -> Result<Groups, serde_json::Error> {
    //     let query = ::groups::owned_groups::Listing::new().build_query();
    //     info!("query: {:?}", query);
    //     self.get(&query)
    // }


    /// Search for a (generic) GitLab item, iterating over all found match to get the proper one.
    ///
    /// This allows getting, for example, a specific issue from a specific project. The GitLab API
    /// does not make this easy to do in a generic way, so we need to perform the search in a loop
    /// until the proper item is found and returned.
    fn get_paginated_from_project<T, F, G, L>(&self, item_search_closure: F, iter_find_closure: G) -> Result<T>
        where F: Fn() -> L,
              G: Fn(&<std::vec::IntoIter<T> as IntoIterator>::Item) -> bool,
              L: Lister<Vec<T>>
    {
        // Explicitly set the pagination information so we can iterate over the pages.
        let mut pagination_page = 1;
        let pagination_per_page = 20;

        let mut found: Option<T>;

        // Query GitLab inside the page loop
        loop {
            // Query GitLab, specifying the pagination information. Use a closure, passed as
            // argument, to make this operation generic.
            // To list project's issues:
            // let found_items = self.issues().project(id).list_paginated(...).chain_err(...)?;
            // To list project's merge requests:
            // let found_items = self.merge_requests(id).list_paginated(...).chain_err(..)?;
            // To get matching projects:
            // let found_items = self.projects().search(name).list_paginated(...).chain_err(...)?;

            let found_items = item_search_closure().list_paginated(pagination_page, pagination_per_page)
                .chain_err(|| "cannot get item in GitLab::get_paginated_from_project()")?;

            let nb_found = found_items.len();

            // Find the right item in the vector, if any. Use the second closure passed as argument
            // as the closure used in `find()`.
            found = found_items.into_iter().find(&iter_find_closure);

            // Break if we find something.
            if found.is_some() {
                break;
            }

            // Also break if the number of found items is less than the maximum allowed per page.
            // In that case, there is no more match and we need to stop the loop (we did not find
            // anything).
            if nb_found < pagination_per_page as usize {
                break;
            }

            // Bump to the next page
            pagination_page += 1;
        }

        // Return the found item
        match found {
            None => bail!("not found!"),
            Some(item) => Ok(item),
        }
    }

    /// Get a specific "namespace/name" project.
    /// NOTE: We can't search for "namespace/name", so we search for "name", and refine the match
    ///       on the namespace. This means the operation could be slow as multiple query to the
    ///       GitLab server might be required to find the right item.
    pub fn get_project(&self, namespace: &str, name: &str) -> Result<::projects::Project> {

        // Closure to search for the item, possibly returning multiple match on multiple pages.
        let query_gitlab_closure = || self.projects().search(name.to_string());
        // Closure to find the right item in the found list on the page.
        let iter_find_closure = |ref project: &::projects::Project| project.namespace.name == namespace && project.name == name;

        self.get_paginated_from_project(query_gitlab_closure, iter_find_closure)
    }

    /// Get a project issue from a its project's `namespace` and `name` and the issue's `iid`.
    ///
    /// Since GitLab uses unique `id`s in its API and _not_ `iid`s, we will need to list issues
    /// (grouped by pages of 20) until we find the proper issue matching the `id` requested.
    ///
    /// **Note**: A `iid` is the issue number as seen by normal user, for example appearing on
    /// a GitLab URL. This `iid` can be used to reference an issue (in other issues, in commit
    /// messages, etc.) by prepending a pound sign to it, for example `#3`. An `id`, instead, is
    /// GitLab's internal and unique id associated with the issue.
    ///
    /// Because we need to search (and thus query the GitLab server possibly multiple times), this
    /// _can_ be a slow operation if there is many issues in the project.
    pub fn get_issue(&self, namespace: &str, name: &str, iid: i64) -> Result<::issues::Issue> {
        // We first need to find the specific project.
        let project = self.get_project(namespace, name)
            .chain_err(|| format!("cannot get project '{}/{}'", namespace, name))?;

        // Closure to search for the item, possibly returning multiple match on multiple pages.
        let query_gitlab_closure = || self.issues().project(project.id);
        // Closure to find the right item in the found list on the page.
        let iter_find_closure = |ref issue: &::issues::Issue| issue.iid == iid;

        self.get_paginated_from_project(query_gitlab_closure, iter_find_closure)
    }

    /// Get a project merge request from a its project's `namespace` and `name` and the issue's `iid`.
    ///
    /// Since GitLab uses unique `id`s in its API and _not_ `iid`s, we will need to list issues
    /// (grouped by pages of 20) until we find the proper issue matching the `id` requested.
    ///
    /// **Note**: A `iid` is the issue number as seen by normal user, for example appearing on
    /// a GitLab URL. This `iid` can be used to reference an issue (in other issues, in commit
    /// messages, etc.) by prepending a pound sign to it, for example `#3`. An `id`, instead, is
    /// GitLab's internal and unique id associated with the issue.
    ///
    /// Because we need to search (and thus query the GitLab server possibly multiple times), this
    /// _can_ be a slow operation if there is many issues in the project.
    pub fn get_merge_request(&self,
                             namespace: &str,
                             name: &str,
                             iid: i64)
                             -> Result<::merge_requests::MergeRequest> {

        // We first need to find the specific project.
        let project = self.get_project(namespace, name)
            .chain_err(|| format!("cannot get project '{}/{}'", namespace, name))?;

        // Closure to search for the item, possibly returning multiple match on multiple pages.
        let query_gitlab_closure = || self.merge_requests(project.id);
        // Closure to find the right item in the found list on the page.
        let iter_find_closure = |ref issue: &::merge_requests::MergeRequest| issue.iid == iid;

        self.get_paginated_from_project(query_gitlab_closure, iter_find_closure)
    }
}

/// Remove the private token from a URL string, replacing it with `${GITLAB_TOKEN}`.
/// This allows setting the environment variable `${GITLAB_TOKEN}` and still be able
/// to copy-paste a printed URL.
fn remove_gitlab_token_from_url(url: &str) -> String {
    let re = regex::Regex::new(r"private_token=\w{20}").unwrap();
    re.replace_all(url, "private_token=$${GITLAB_TOKEN}").into()
}


#[cfg(test)]
mod tests {
    use std::fmt;
    use gitlab::*;
    use errors::*;

    fn verify_ok<T>(result: &Result<T>) {
        if let &Err(ref e) = result {
            println!("error: {}", e);

            for e in e.iter().skip(1) {
                println!("caused by: {}", e);
            }

            // The backtrace is not always generated. Try to run this example
            // with `RUST_BACKTRACE=1`.
            if let Some(backtrace) = e.backtrace() {
                println!("backtrace: {:?}", backtrace);
            }
        }
        assert!(result.is_ok());
    }

    fn verify_err<T>(result: &Result<T>)
        where T: fmt::Debug
    {
        match result {
            &Err(_) => {
                // pass
            }
            &Ok(ref t) => {
                panic!(format!("Expected an Err(), got an Ok(t), with t: {:?}", t));
            }
        }
    }

    #[test]
    fn impl_debug_for_gitlab() {
        let gl = GitLab::new("gitlab.com", "XXXXXXXXXXXXXXXXXXXX").unwrap();

        let debug = format!("{:?}", gl);
        assert_eq!("GitLab { scheme: https, domain: gitlab.com, port: no port provided, \
                    private_token: XXXXXXXXXXXXXXXXXXXX }",
                   debug);

        let gl = gl.scheme("http").port(80);
        let debug = format!("{:?}", gl);
        assert_eq!("GitLab { scheme: http, domain: gitlab.com, port: no port provided, \
                    private_token: XXXXXXXXXXXXXXXXXXXX }",
                   debug);

        let gl = gl.port(81);
        let debug = format!("{:?}", gl);
        assert_eq!("GitLab { scheme: http, domain: gitlab.com, port: 81, private_token: \
                    XXXXXXXXXXXXXXXXXXXX }",
                   debug);
    }

    #[test]
    fn gitlab_listers_groups() {
        let gl = GitLab::new("gitlab.com", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        let groups_lister = gl.groups();
        let debug = format!("{:?}", groups_lister);
        assert_eq!("GroupsLister { gl: GitLab { scheme: https, domain: gitlab.com, port: no \
                    port provided, private_token: XXXXXXXXXXXXXXXXXXXX }, \
                    internal: GroupsListerInternal { skip_groups: None, all_available: None, \
                    search: None, order_by: None, sort: None } }",
                   debug);
    }

    #[test]
    fn gitlab_listers_projects() {
        let gl = GitLab::new("gitlab.com", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        let projects_lister = gl.projects();
        let debug = format!("{:?}", projects_lister);
        assert_eq!("ProjectsLister { gl: GitLab { scheme: https, domain: gitlab.com, port: no \
                    port provided, private_token: XXXXXXXXXXXXXXXXXXXX }, \
                    internal: ProjectListerInternal { archived: None, visibility: None, \
                    order_by: None, sort: None, search: None, simple: None } }",
                   debug);
    }

    #[test]
    fn gitlab_listers_issues() {
        let gl = GitLab::new("gitlab.com", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        let issues_lister = gl.issues();
        let debug = format!("{:?}", issues_lister);
        assert_eq!("IssuesLister { gl: GitLab { scheme: https, domain: gitlab.com, port: no port \
                    provided, private_token: XXXXXXXXXXXXXXXXXXXX }, internal: \
                    IssuesListerInternal { state: None, labels: None, order_by: None, sort: None \
                    } }",
                   debug);
    }

    #[test]
    fn new_valid() {
        let gl = GitLab::new("gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_ok(&gl);

        let gl = GitLab::new_insecure("gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_ok(&gl);

        let gl = GitLab::new("localhost", "XXXXXXXXXXXXXXXXXXXX");
        verify_ok(&gl);

        let gl = GitLab::new_insecure("localhost", "XXXXXXXXXXXXXXXXXXXX");
        verify_ok(&gl);
    }

    #[test]
    fn new_invalid_url_1() {
        let gl = GitLab::new("", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new_insecure("", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn new_invalid_url_2() {
        let gl = GitLab::new("gitla/b.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new_insecure("gitla/b.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn new_invalid_url_3() {
        let gl = GitLab::new("/gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new_insecure("/gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn new_invalid_url_4() {
        let gl = GitLab::new("http:/gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new_insecure("http:/gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn new_invalid_url_5() {
        let gl = GitLab::new("http:///gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new_insecure("http:///gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn new_invalid_url_6() {
        let gl = GitLab::new(".gitlab", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new_insecure(".gitlab", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn new_invalid_url_7() {
        let gl = GitLab::new(".gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new_insecure(".gitlab.com", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn new_invalid_url_8() {
        let gl = GitLab::new("gitlab.", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new_insecure("gitlab.", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn new_invalid_url_10() {
        let gl = GitLab::new("gitlab.com.", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new_insecure("gitlab.com.", "XXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn new_invalid_token() {
        let gl = GitLab::new("gitlab.com", "");
        verify_err(&gl);

        let gl = GitLab::new("gitlab.com", "X");
        verify_err(&gl);

        let gl = GitLab::new("gitlab.com", "XXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);

        let gl = GitLab::new("gitlab.com", "XXXXXXXXXXXXXXXXXXXXX");
        verify_err(&gl);
    }

    #[test]
    fn build_url_doc() {
        let expected_url = "https://gitlab.example.com\
                            /api/v3/groups?order_by=path&private_token=XXXXXXXXXXXXXXXXXXXX";
        let gl = GitLab::new("gitlab.example.com", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        let url = gl.build_url("groups?order_by=path").unwrap();
        assert_eq!(url, expected_url);
    }
}
