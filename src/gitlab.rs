
use std::io::Read;  // Trait providing read_to_string()

use hyper;


pub const API_VERSION: u16 = 3;


#[derive(Debug)]
pub struct GitLab {
    scheme: String,
    domain: String,
    port: u16,
    private_token: String,
    client: hyper::Client,
}


impl GitLab {

    pub fn new(scheme: &str, domain: &str, port: u16, private_token: &str) -> GitLab {
        GitLab {
            scheme: scheme.to_string(),
            domain: domain.to_string(),
            port:   port,
            private_token: private_token.to_string(),
            client: hyper::Client::new()
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
        self.client.get(&url).send()
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

        println!("####################################################################");
        let mut body = String::new();
        println!("####################################################################");
        let result = res.read_to_string(&mut body);
        println!("####################################################################");
        println!("body:\n{}", body);
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
