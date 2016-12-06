
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
               self.url.domain().expect("bad hostname provided"),
               self.url.port().expect("bad port provided"),
               self.pagination)
    }
}


impl GitLab {
    pub fn _new(scheme: &str, domain: &str, port: u16, private_token: &str) -> Result<GitLab> {
        let url_string = format!("{}://{}/api/v{}/", scheme, domain, API_VERSION);
        let mut url = url::Url::parse(&url_string)
            .chain_err(|| format!("failure to parse URL '{}'", url_string))?;
        url.set_port(Some(port)).expect("bad port provided");
        Ok(GitLab {
            url: url,
            private_token: private_token.to_string(),
            pagination: None,
            client: match env::var("HTTP_PROXY") {
                Ok(proxy) => {
                    let proxy: Vec<&str> = proxy.trim_left_matches("http://").split(':').collect();
                    let hostname = proxy[0].to_string();
                    // let port = proxy[1].parse().chain_err(|| format!("failure to set port to {}", port))?;
                    let port = proxy[1].parse().unwrap();

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
    ///                     /api/v3/groups?order_by=path&private_token=XXXXXXXXXXXXX";
    ///
    /// let gl = GitLab::new("gitlab.example.com", "XXXXXXXXXXXXX").unwrap();
    ///
    /// assert_eq!(gl.build_url("groups?order_by=path").unwrap(), expected_url);
    /// ```
    pub fn build_url(&self, query: &str) -> Result<String> {
        let mut new_url =
            self.url.clone().join(query).chain_err(|| {
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

    pub fn set_pagination(&mut self, pagination: Pagination) {
        self.pagination = Some(pagination);
    }

    pub fn get<T>(&self, query: &str) -> Result<T>
        where T: serde::Deserialize
    {
        let url = self.build_url(query).chain_err(|| format!("failure to build url for query '{}'", query))?;
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

        // assert_eq!(res.status, hyper::status::StatusCode::Ok);
        if res.status != hyper::status::StatusCode::Ok {
            bail!(format!("status code ({}) not Ok()", res.status));
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
}


// #[cfg(test)]
// mod tests {
//
// #[test]
// fn it_works() {
// }
// }
//
