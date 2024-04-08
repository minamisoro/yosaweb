use askama::Template;

#[derive(Template)]
#[template(path = "footer.html")]
pub struct Footer {}

impl Footer {
    pub fn new() -> Self {
        Self {}
    }
}
