
use hyper;


pub const API_VERSION: u16 = 3;


#[derive(Debug)]
pub struct GitLab {
    scheme: String,
    domain: String,
    port: u16,
    private_token: String,
}


impl GitLab {

    pub fn new(scheme: &str, domain: &str, port: u16, private_token: &str) -> GitLab {
        let gl = GitLab {
            scheme: scheme.to_string(),
            domain: domain.to_string(),
            port:   port,
            private_token: private_token.to_string(),
        };

        gl
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

    pub fn attempt_connection(&self) -> Result<hyper::client::Response, hyper::Error> {
        let url = self.build_url("version");

        let client = hyper::Client::new();

        client.get(&url).send()

    }

    // pub fn projects(&self) -> ProjectManager {
    //     ProjectManager()
    // }
}
