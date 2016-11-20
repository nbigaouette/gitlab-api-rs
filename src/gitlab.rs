
use std::io::Read;  // Trait providing read_to_string()

use std::env;

use hyper;
use rustc_serialize;


use projects;
use groups;


pub const API_VERSION: u16 = 3;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Version {
    pub version: String,
    pub revision: String,
}


#[derive(Debug)]
pub struct Pagination {
    pub page: u16,
    pub per_page: u16,
}

#[derive(Debug)]
pub struct GitLab {
    scheme: String,
    domain: String,
    port: u16,
    private_token: String,
    client: hyper::Client,
    pagination: Pagination,
}


impl GitLab {

    pub fn new(scheme: &str, domain: &str, port: u16, private_token: &str) -> GitLab {
        GitLab {
            scheme: scheme.to_string(),
            domain: domain.to_string(),
            port:   port,
            private_token: private_token.to_string(),
            client: match env::var("HTTP_PROXY") {
                Ok(proxy) => {
                    println!("proxy: {:?}", proxy);
                    let proxy: Vec<&str> = proxy.trim_left_matches("http://").split(':').collect();
                    let hostname = proxy[0].to_string();
                    let port = proxy[1];

                    hyper::Client::with_http_proxy(hostname, port.parse().unwrap())
                },
                Err(_) => hyper::Client::new(),
            },
            pagination: Pagination {page: 1, per_page: 20},
        }
    }

    pub fn new_http(domain: &str, private_token: &str) -> GitLab {
        GitLab::new("http", &domain, 80, &private_token)
    }

    pub fn new_https(domain: &str, private_token: &str) -> GitLab {
        GitLab::new("https", &domain, 443, &private_token)
    }

    pub fn build_url(&self, command: &str) -> String {
        format!("{}://{}:{}/api/v{}/{}?private_token={}&page={}&per_page={}",
                                self.scheme,
                                self.domain,
                                self.port,
                                API_VERSION,
                                command,
                                self.private_token,
                                self.pagination.page,
                                self.pagination.per_page)
    }

    pub fn attempt_connection(&self) -> Result<hyper::client::Response, hyper::Error> {
        // self.get("version")
        let url = self.build_url("version");
        // Close connections after each GET.
        let res = self.client.get(&url).header(hyper::header::Connection::close()).send();

        res
    }

    pub fn set_pagination(&mut self, pagination: Pagination) {
        self.pagination = pagination;
    }

    pub fn get<T>(&self, command: &str) -> Result<T, rustc_serialize::json::DecoderError>
            where T: rustc_serialize::Decodable {

        let url = self.build_url(command);
        let mut res: hyper::client::Response =
                        self.client
                        .get(&url)
                        .header(hyper::header::Connection::close())
                        .send()
                        .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        rustc_serialize::json::decode(body.as_str())
    }

    pub fn version(&self) -> Result<Version, rustc_serialize::json::DecoderError> {
        self.get("version")
    }

    pub fn groups(&self) -> Result<groups::Groups, rustc_serialize::json::DecoderError> {
        self.get("groups")
    }

    pub fn projects(&self) -> Result<projects::Projects, rustc_serialize::json::DecoderError> {
        self.get("projects")
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use gitlab::GitLab;
    use gitlab::Pagination;

    #[test]
    fn list_groups() {
        let token = match env::var("GITLAB_TOKEN") {
            Ok(val) => val,
            Err(_)  => panic!("Please set environment variable 'GITLAB_TOKEN'"),
        };

        let mut gl = GitLab::new_https("gitlab.com", &token);
        gl.set_pagination(Pagination{page: 1, per_page: 100});
        println!("gl: {:?}", gl);

        let groups = gl.groups().unwrap();
        println!("groups: {:?}", groups);
    }

    #[test]
    fn list_projects() {
        let token = match env::var("GITLAB_TOKEN") {
            Ok(val) => val,
            Err(_)  => panic!("Please set environment variable 'GITLAB_TOKEN'"),
        };

        let mut gl = GitLab::new_https("gitlab.com", &token);
        // for i in 1..82 {
        //     gl.set_pagination(Pagination{page: i, per_page: 1});
        //     println!("projects: {:?}", gl.projects().unwrap());
        // }
        //     println!("projects: {:?}", gl.projects().unwrap());
        gl.set_pagination(Pagination{page: 1, per_page: 100});
        let projects = gl.projects().unwrap();

        // assert_eq!(gl.attempt_connection().unwrap().status, hyper::Ok);

    }
}
