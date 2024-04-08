use askama::Template;

#[derive(Template)]
#[template(path = "store.html")]
pub struct Store {}

impl Store {
    pub fn new() -> Self {
        Self {}
    }
}
