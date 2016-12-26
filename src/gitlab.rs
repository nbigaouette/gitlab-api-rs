
use std::fmt;
use std::io::Read;  // Trait providing read_to_string()
use std::env;

use url;
use hyper;
use serde;
use serde_json;


// use Groups;

use ::errors::*;


pub const API_VERSION: u16 = 3;




#[derive(Default, Clone, Copy, Debug)]
pub struct Pagination {
    pub page: u16,
    pub per_page: u16,
}

pub struct GitLab {
    url: url::Url,
    private_token: String,
    pagination: Option<Pagination>,
    client: hyper::Client,
}


// Explicitly implement Debug trait for GitLab so we can hide the token.
impl fmt::Debug for GitLab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "GitLab {{ scheme: {}, domain: {}, port: {}, private_token: XXXXXXXXXXXXXXXXXXXX, \
                pagination: {:?} }}",
               self.url.scheme(),
               self.url.domain().unwrap_or("bad hostname provided"),
               self.url
                   .port()
                   .map(|port_u16| port_u16.to_string())
                   .unwrap_or("no port provided".to_string()),
               self.pagination)
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
            pagination: None,
            client: match env::var("HTTP_PROXY") {
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
        self.pagination.as_ref().map(|pagination| {
            new_url.query_pairs_mut().append_pair("page", &pagination.page.to_string());
            new_url.query_pairs_mut().append_pair("per_page", &pagination.per_page.to_string());
        });

        Ok(new_url.into_string())
    }

    // pub fn attempt_connection(&self) -> Result<hyper::client::Response, hyper::Error> {
    //     let url = self.build_url("version");
    //     // Close connections after each GET.
    //     self.client.get(&url).header(hyper::header::Connection::close()).send()
    // }


    /// Set pagination information
    ///
    /// # Examples
    ///
    /// ```
    /// use gitlab_api::{GitLab, Pagination};
    ///
    /// let expected_url = "https://gitlab.example.com\
    ///                     /api/v3/groups?order_by=path&\
    ///                     private_token=XXXXXXXXXXXXXXXXXXXX&page=2&per_page=5";
    ///
    /// let mut gl = GitLab::new("gitlab.example.com", "XXXXXXXXXXXXXXXXXXXX").unwrap();
    /// gl.set_pagination(Pagination {page: 2, per_page: 5});
    /// assert_eq!(gl.build_url("groups?order_by=path").unwrap(), expected_url);
    /// ```
    pub fn set_pagination(&mut self, pagination: Pagination) {
        self.pagination = Some(pagination);
    }

    pub fn get<T>(&self, query: &str) -> Result<T>
        where T: serde::Deserialize
    {
        let url = self.build_url(query)
            .chain_err(|| format!("failure to build url for query '{}'", query))?;
        info!("url: {:?}", url);

        // Close connections after each GET.
        let mut res: hyper::client::Response = self.client
            .get(&url)
            .header(hyper::header::Connection::close())
            .send()
            .chain_err(|| format!("cannot send request '{}' to {:?}", query, self))?;
        info!("res.status: {:?}", res.status);
        debug!("res.headers: {:?}", res.headers);
        debug!("res.url: {:?}", res.url);

        let mut body = String::new();
        res.read_to_string(&mut body).chain_err(|| "cannot read response body")?;
        debug!("body:\n{:?}", body);

        if res.status != hyper::status::StatusCode::Ok {
            bail!(format!("status code '{}', not '200 OK'", res.status));
        }

        serde_json::from_str(body.as_str())
            .chain_err(|| format!("cannot build Rust struct from JSON data: {}", body))
    }

    pub fn version(&self) -> Result<::Version> {
        self.get("version").chain_err(|| "cannot query 'version'")
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

    // Higher level methods

    pub fn get_project(&mut self, namespace: &str, name: &str) -> Result<::Project> {
        // We can't search for "namespace/name", so we search for "name", and loop on the result
        // until we find the proper "namespace/name".
        // NOTE: Since our search match could contain many results and they will be paginated,
        //       we need two loops: one on content of a page, one for the pages.

        // Store the initial pagination so we can restore it later
        let initial_pagination = self.pagination.clone();

        // Set a default value for the pagination if it's None
        self.pagination = self.pagination.or(Some(Pagination {page: 1, per_page: 20}));

        let mut found_project: Option<::Project> = None;

        // Query GitLab inside the page loop
        loop {
            let projects = self.projects().search(name.to_string()).list().chain_err(|| "cannot get projects")?;

            // Loop over the found projects
            for project in projects {
                println!("############# project: {:?}", project);

                if project.namespace.name == namespace && project.name == name {
                    found_project = Some(project);
                    break;
                }
            }

            if found_project.is_some() {
                break;
            }

            // Bump to the next page
            self.pagination.as_mut().map(|pagination| pagination.page += 1);
        }

        // Restore the initial pagination
        self.pagination = initial_pagination;

        match found_project {
            None => bail!(format!("Project '{}/{}' not found!", namespace, name)),
            Some(project) => Ok(project)
        }
    }
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
                    private_token: XXXXXXXXXXXXXXXXXXXX, pagination: None }",
                   debug);

        let gl = gl.scheme("http").port(80);
        let debug = format!("{:?}", gl);
        assert_eq!("GitLab { scheme: http, domain: gitlab.com, port: no port provided, \
                    private_token: XXXXXXXXXXXXXXXXXXXX, pagination: None }",
                   debug);

        let mut gl = gl.port(81);
        let debug = format!("{:?}", gl);
        assert_eq!("GitLab { scheme: http, domain: gitlab.com, port: 81, private_token: \
                    XXXXXXXXXXXXXXXXXXXX, pagination: None }",
                   debug);

        gl.set_pagination(Pagination {
            page: 2,
            per_page: 5,
        });
        let debug = format!("{:?}", gl);
        assert_eq!("GitLab { scheme: http, domain: gitlab.com, port: 81, private_token: \
                    XXXXXXXXXXXXXXXXXXXX, pagination: Some(Pagination { page: 2, per_page: 5 }) }",
                   debug);
    }

    #[test]
    fn gitlab_listers_groups() {
        let gl = GitLab::new("gitlab.com", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        let groups_lister = gl.groups();
        let debug = format!("{:?}", groups_lister);
        assert_eq!("GroupsLister { gl: GitLab { scheme: https, domain: gitlab.com, port: no \
                    port provided, private_token: XXXXXXXXXXXXXXXXXXXX, pagination: None }, \
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
                    port provided, private_token: XXXXXXXXXXXXXXXXXXXX, pagination: None }, \
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
                    provided, private_token: XXXXXXXXXXXXXXXXXXXX, pagination: None }, internal: \
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

    #[test]
    fn build_url_pagination() {
        let expected_url = "https://gitlab.example.com\
                            /api/v3/groups?order_by=path&\
                            private_token=XXXXXXXXXXXXXXXXXXXX&page=2&per_page=5";
        let mut gl = GitLab::new("gitlab.example.com", "XXXXXXXXXXXXXXXXXXXX").unwrap();
        gl.set_pagination(Pagination {page: 2, per_page: 5});
        let url = gl.build_url("groups?order_by=path").unwrap();
        assert_eq!(url, expected_url);
    }
}
