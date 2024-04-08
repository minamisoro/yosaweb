use askama::Template;

#[derive(Template)]
#[template(path = "navbar.html")]
pub struct Navbar {}

impl Navbar {
    pub fn new() -> Self {
        Self {}
    }
}
