use askama::Template;

#[derive(Template)]
#[template(path = "contact.html")]
pub struct Contact {}

impl Contact {
    pub fn new() -> Self {
        Self {}
    }
}
