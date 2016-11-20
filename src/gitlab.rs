
use std::io::Read;  // Trait providing read_to_string()

use hyper;
use rustc_serialize;


use projects;


pub const API_VERSION: u16 = 3;


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
            client: hyper::Client::new(),
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
        format!("{}://{}:{}/api/v{}/{}?private_token={}",
                                self.scheme,
                                self.domain,
                                self.port,
                                API_VERSION,
                                command,
                                self.private_token)
    }

    pub fn get(&self, command: &str) -> Result<hyper::client::Response, hyper::Error> {
        let url = self.build_url(&command);
        // Close connections after each GET.
        self.client.get(&url).header(hyper::header::Connection::close()).send()
    }

    pub fn set_pagination(&mut self, pagination: Pagination) {
        self.pagination = pagination;
    }

    pub fn attempt_connection(&self) -> Result<hyper::client::Response, hyper::Error> {
        self.get("version")
    }

    pub fn projects(&self) {
        let mut res: hyper::client::Response = self.get("projects").unwrap();
        println!("####################################################################");
        println!("res: {:?}", res);
        println!("####################################################################");
        println!("Response: {}", res.status);
        println!("####################################################################");
        println!("Headers:\n{}", res.headers);
        println!("####################################################################");


        // let fresh_request = Request::get(url);
        // let streaming_request = fresh_request.start();
        // let mut response = streaming_request.send();
        // Ok(response.read_to_string())

        // let mut body = Vec::new::<u8>();
        // body.resize(, 0);
        // let result = res.read_exact(&body);

        // println!("####################################################################");
        println!("----------------------------");
        let mut body = String::new();
        println!("----------------------------");
        res.read_to_string(&mut body).unwrap();
        // println!("####################################################################");
        // let result = res.read_to_string(&mut body);
        // println!("####################################################################");
        // println!("body:\n{}", body);

        let projects: projects::Projects = rustc_serialize::json::decode(body.as_str()).unwrap();
        println!("{}", projects.p[0].description);

    }

    // pub fn projects(&self) -> ProjectManager {
    //     ProjectManager()
    // }
}


#[cfg(test)]
mod tests {
    use std::env;
    use gitlab::GitLab;

    #[test]
    fn list_projects() {
        let token = match env::var("GITLAB_TOKEN") {
            Ok(val) => val,
            Err(_)  => panic!("Please set environment variable 'GITLAB_TOKEN'"),
        };

        let gl = GitLab::new_https("gitlab.com", &token);
        println!("gl: {:?}", gl);

        gl.projects();
        // assert_eq!(gl.attempt_connection().unwrap().status, hyper::Ok);

    }
}
