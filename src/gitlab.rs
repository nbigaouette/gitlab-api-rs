
use std::fmt;
use std::io::Read;  // Trait providing read_to_string()
use std::env;

use hyper;
use serde;
use serde_json;


use BuildQuery;
use Groups;


pub const API_VERSION: u16 = 3;




#[derive(Debug)]
pub struct Pagination {
    pub page: u16,
    pub per_page: u16,
}

pub struct GitLab {
    scheme: String,
    domain: String,
    port: u16,
    private_token: String,
    client: hyper::Client,
    pagination: Pagination,
}


// Explicitly implement Debug trait for GitLab so we can hide the token.
impl fmt::Debug for GitLab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "GitLab {{ scheme: {}, domain: {}, port: {}, private_token: XXXXXXXXXXXXXXXXXXXX, \
                client: {:?}, pagination: {:?} }}",
               self.scheme,
               self.domain,
               self.port,
               self.client,
               self.pagination)
    }
}


impl GitLab {
    pub fn new(scheme: &str, domain: &str, port: u16, private_token: &str) -> GitLab {
        GitLab {
            scheme: scheme.to_string(),
            domain: domain.to_string(),
            port: port,
            private_token: private_token.to_string(),
            client: match env::var("HTTP_PROXY") {
                Ok(proxy) => {
                    let proxy: Vec<&str> = proxy.trim_left_matches("http://").split(':').collect();
                    let hostname = proxy[0].to_string();
                    let port = proxy[1];

                    hyper::Client::with_http_proxy(hostname, port.parse().unwrap())
                }
                Err(_) => hyper::Client::new(),
            },
            pagination: Pagination {
                page: 1,
                per_page: 20,
            },
        }
    }

    pub fn new_http(domain: &str, private_token: &str) -> GitLab {
        GitLab::new("http", domain, 80, private_token)
    }

    pub fn new_https(domain: &str, private_token: &str) -> GitLab {
        GitLab::new("https", domain, 443, private_token)
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
    /// let expected_url = "https://gitlab.example.com:\
    ///                     443/api/v3/groups?\
    ///                     order_by=path&private_token=XXXXXXXXXXXXX&page=1&per_page=20";
    ///
    /// let gl = GitLab::new_https("gitlab.example.com", "XXXXXXXXXXXXX");
    ///
    /// assert_eq!(gl.build_url("groups?order_by=path"), expected_url);
    /// ```
    pub fn build_url(&self, query: &str) -> String {
        let params_splitter = if query.find('?').is_some() { "&" } else { "?" };
        format!("{}://{}:{}/api/v{}/{}{}private_token={}&page={}&per_page={}",
                self.scheme,
                self.domain,
                self.port,
                API_VERSION,
                query,
                params_splitter,
                self.private_token,
                self.pagination.page,
                self.pagination.per_page)
    }

    pub fn attempt_connection(&self) -> Result<hyper::client::Response, hyper::Error> {
        let url = self.build_url("version");
        // Close connections after each GET.
        self.client.get(&url).header(hyper::header::Connection::close()).send()
    }

    pub fn set_pagination(&mut self, pagination: Pagination) {
        self.pagination = pagination;
    }

    pub fn get<T>(&self, query: &str) -> Result<T, serde_json::Error>
        where T: serde::Deserialize
    {
        let url = self.build_url(&query);
        info!("url: {:?}", url);
        let mut res: hyper::client::Response = self.client
            .get(&url)
            .header(hyper::header::Connection::close())
            .send()
            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        debug!("body: {:?}", body);

        // FIXME: Properly handle the error. Will require defining our own errors...
        assert_eq!(res.status, hyper::status::StatusCode::Ok);

        serde_json::from_str(&body.as_str())
    }

    // pub fn version(&self) -> Result<Version, serde_json::Error> {
    //     self.get("version")
    // }
    //
    // pub fn groups(&self) -> Result<Groups, serde_json::Error> {
    //     self.get("groups")
    // }
    //
    // pub fn projects(&self) -> Result<Projects, serde_json::Error> {
    //     self.get("projects")
    // }

    pub fn groups(&mut self, listing: ::groups::Listing) -> Result<Groups, serde_json::Error> {
        let query = listing.build_query();
        self.get(&query)
    }

    pub fn owned_groups(&mut self) -> Result<Groups, serde_json::Error> {
        let query = ::groups::owned_groups::Listing::new().build_query();
        info!("query: {:?}", query);
        self.get(&query)
    }
}


// #[cfg(test)]
// mod tests {
//
// #[test]
// fn it_works() {
// }
// }
//
