use askama::Template;

#[derive(Template)]
#[template(path = "header.html")]
pub struct Header {}

impl Header {
    pub fn new() -> Self {
        Self {}
    }
}
