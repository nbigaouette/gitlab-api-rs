// Inspired by http://python-gitlab.readthedocs.io/en/stable/

pub mod gitlab;

pub use gitlab::GitLab;





#[cfg(test)]
mod tests {
    use gitlab::GitLab;

    #[test]
    fn it_works() {

        let gl = GitLab::new("http", "gitlab.com", 80, "XXXXXXXXXXXXX");
        println!("gl: {:?}", gl);

        let gl = GitLab::new_http("gitlab.com", "XXXXXXXXXXXXX");
        println!("gl: {:?}", gl);

        let gl = GitLab::new_https("gitlab.com", "XXXXXXXXXXXXX");
        println!("gl: {:?}", gl);

    }
}
