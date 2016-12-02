
use std::fmt;
use std::io::Read;  // Trait providing read_to_string()
use std::env;

use hyper;
use serde;
use serde_json;


use BuildQuery;
use Groups;


pub const API_VERSION: u16 = 3;




#[derive(Default, Clone, Copy, Debug)]
pub struct Pagination {
    pub page: u16,
    pub per_page: u16,
}

#[derive(Default)]
pub struct GitLab {
    scheme: String,
    domain: String,
    port: u16,
    private_token: String,
    // client: hyper::Client,
    http_proxy: Option<String>,
    pagination: Option<Pagination>,
}


// Explicitly implement Debug trait for GitLab so we can hide the token.
impl fmt::Debug for GitLab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "GitLab {{ scheme: {}, domain: {}, port: {}, private_token: XXXXXXXXXXXXXXXXXXXX, \
                pagination: {:?} }}",
               self.scheme,
               self.domain,
               self.port,
            // client: {:?}, \
            //    self.client,
               self.pagination)
    }
}


impl GitLab {
    pub fn _new(scheme: &str, domain: &str, port: u16, private_token: &str) -> GitLab {
        GitLab {
            scheme: scheme.to_string(),
            domain: domain.to_string(),
            port: port,
            private_token: private_token.to_string(),
            http_proxy: env::var("HTTP_PROXY").ok(),
            pagination: None,
        }
    }

    pub fn new_insecure(domain: &str, private_token: &str) -> GitLab {
        warn!("Using insecure http:// protocol: Token will be sent in clear!");
        GitLab::_new("http", domain, 80, private_token)
    }

    pub fn new(domain: &str, private_token: &str) -> GitLab {
        GitLab::_new("https", domain, 443, private_token)
    }

    pub fn http_proxy(&self) -> Option<&str> {
        self.http_proxy.as_ref().map(|proxy| &**proxy)
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
    ///                     443/api/v3/groups?order_by=path&private_token=XXXXXXXXXXXXX";
    ///
    /// let gl = GitLab::new("gitlab.example.com", "XXXXXXXXXXXXX");
    ///
    /// assert_eq!(gl.build_url("groups?order_by=path"), expected_url);
    /// ```
    pub fn build_url(&self, query: &str) -> String {
        let params_splitter = if query.find('?').is_some() { "&" } else { "?" };
        let mut url = format!("{}://{}:{}/api/v{}/{}{}private_token={}",
                              self.scheme,
                              self.domain,
                              self.port,
                              API_VERSION,
                              query,
                              params_splitter,
                              self.private_token);

        self.pagination.as_ref().map(|pagination| {
            url.push_str(&format!("&page={}&per_page={}", pagination.page, pagination.per_page));
        });

        url
    }

    // pub fn attempt_connection(&self) -> Result<hyper::client::Response, hyper::Error> {
    //     let url = self.build_url("version");
    //     // Close connections after each GET.
    //     self.client.get(&url).header(hyper::header::Connection::close()).send()
    // }

    pub fn set_pagination(&mut self, pagination: Pagination) {
        self.pagination = Some(pagination);
    }

    pub fn get<T>(&self, query: &str) -> Result<T, serde_json::Error>
        where T: serde::Deserialize
    {
        let url = self.build_url(query);
        info!("url: {:?}", url);
        unimplemented!();
        // let mut res: hyper::client::Response = self.client
        //     .get(&url)
        //     .header(hyper::header::Connection::close())
        //     .send()
        //     .unwrap();
        // info!("res.status: {:?}", res.status);
        // debug!("res.headers: {:?}", res.headers);
        // debug!("res.url: {:?}", res.url);
        //
        // let mut body = String::new();
        // res.read_to_string(&mut body).unwrap();
        // debug!("body:\n{:?}", body);
        //
        // // FIXME: Properly handle the error. Will require defining our own errors...
        // assert_eq!(res.status, hyper::status::StatusCode::Ok);
        //
        // serde_json::from_str(body.as_str())
    }

    // pub fn version(&self) -> Result<Version, serde_json::Error> {
    //     self.get("version")
    // }
    //
    // pub fn groups(&self) -> Result<Groups, serde_json::Error> {
    //     self.get("groups")
    // }

    pub fn projects(&self) -> ::projects::ProjectsLister {
        ::projects::ProjectsLister::new(&self)
    }

    pub fn groups(&mut self, listing: ::groups::Listing) -> Result<Groups, serde_json::Error> {
        let query = listing.build_query();
        // self.get(&query)
        unimplemented!();
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
